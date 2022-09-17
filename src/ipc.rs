use crate::rendezvous_mediator::RendezvousMediator;
pub use clipboard::ClipbaordFile;
use hbb_common::{
    allow_err, bail, bytes,
    bytes_codec::BytesCodec,
    config::{self, Config, Config2},
    futures::StreamExt as _,
    futures_util::sink::SinkExt,
    log, timeout, tokio,
    tokio::io::{AsyncRead, AsyncWrite},
    tokio_util::codec::Framed,
    ResultType,
};
use parity_tokio_ipc::{
    Connection as Conn, ConnectionClient as ConnClient, Endpoint, Incoming, SecurityAttributes,
};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(windows))]
use std::{fs::File, io::prelude::*};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum FS {
    ReadDir {
        dir: String,
        include_hidden: bool,
    },
    RemoveDir {
        path: String,
        id: i32,
        recursive: bool,
    },
    RemoveFile {
        path: String,
        id: i32,
        file_num: i32,
    },
    CreateDir {
        path: String,
        id: i32,
    },
    NewWrite {
        path: String,
        id: i32,
        files: Vec<(String, u64)>,
    },
    CancelWrite {
        id: i32,
    },
    WriteBlock {
        id: i32,
        file_num: i32,
        data: Vec<u8>,
        compressed: bool,
    },
    WriteDone {
        id: i32,
        file_num: i32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "t", content = "c")]
pub enum Data {
    Login {
        id: i32,
        is_file_transfer: bool,
        peer_id: String,
        name: String,
        authorized: bool,
        port_forward: String,
        keyboard: bool,
        clipboard: bool,
        audio: bool,
        file: bool,
        file_transfer_enabled: bool,
    },
    ChatMessage {
        text: String,
    },
    SwitchPermission {
        name: String,
        enabled: bool,
    },
    SystemInfo(Option<String>),
    Authorize,
    Close,
    SAS,
    OnlineStatus(Option<(i64, bool)>),
    Config((String, Option<String>)),
    Options(Option<HashMap<String, String>>),
    NatType(Option<i32>),
    ConfirmedKey(Option<(Vec<u8>, Vec<u8>)>),
    RawMessage(Vec<u8>),
    Socks(Option<config::Socks5Server>),
    FS(FS),
    Test,
    SyncConfig(Option<(Config, Config2)>),
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    ClipbaordFile(ClipbaordFile),
    ClipboardFileEnabled(bool),
}

#[tokio::main(flavor = "current_thread")]
pub async fn start(postfix: &str) -> ResultType<()> {
    let mut incoming = new_listener(postfix).await?;
    loop {
        if let Some(result) = incoming.next().await {
            match result {
                Ok(stream) => {
                    let mut stream = Connection::new(stream);
                    let postfix = postfix.to_owned();
                    tokio::spawn(async move {
                        loop {
                            match stream.next().await {
                                Err(err) => {
                                    log::trace!("ipc{} connection closed: {}", postfix, err);
                                    break;
                                }
                                Ok(Some(data)) => {
                                    handle(data, &mut stream).await;
                                }
                                _ => {}
                            }
                        }
                    });
                }
                Err(err) => {
                    log::error!("Couldn't get client: {:?}", err);
                }
            }
        }
    }
}

pub async fn new_listener(postfix: &str) -> ResultType<Incoming> {
    let path = Config::ipc_path(postfix);
    #[cfg(not(windows))]
    check_pid(postfix).await;
    let mut endpoint = Endpoint::new(path.clone());
    match SecurityAttributes::allow_everyone_create() {
        Ok(attr) => endpoint.set_security_attributes(attr),
        Err(err) => log::error!("Failed to set ipc{} security: {}", postfix, err),
    };
    match endpoint.incoming() {
        Ok(incoming) => {
            log::info!("Started ipc{} server at path: {}", postfix, &path);
            #[cfg(not(windows))]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o0777)).ok();
                write_pid(postfix);
            }
            Ok(incoming)
        }
        Err(err) => {
            log::error!(
                "Failed to start ipc{} server at path {}: {}",
                postfix,
                path,
                err
            );
            Err(err.into())
        }
    }
}

pub struct CheckIfRestart(String, Vec<String>, String);

impl CheckIfRestart {
    pub fn new() -> CheckIfRestart {
        CheckIfRestart(
            Config::get_option("stop-service"),
            Config::get_rendezvous_servers(),
            Config::get_option("audio-input"),
        )
    }
}
impl Drop for CheckIfRestart {
    fn drop(&mut self) {
        if self.0 != Config::get_option("stop-service")
            || self.1 != Config::get_rendezvous_servers()
        {
            RendezvousMediator::restart();
        }
        if self.2 != Config::get_option("audio-input") {
            crate::audio_service::restart();
        }
    }
}

async fn handle(data: Data, stream: &mut Connection) {
    match data {
        Data::SystemInfo(_) => {
            let info = format!(
                "log_path: {}, config: {}, username: {}",
                Config::log_path().to_str().unwrap_or(""),
                Config::file().to_str().unwrap_or(""),
                crate::username(),
            );
            allow_err!(stream.send(&Data::SystemInfo(Some(info))).await);
        }
        Data::Close => {
            log::info!("Receive close message");
            crate::server::input_service::fix_key_down_timeout_at_exit();
            std::process::exit(0);
        }
        Data::OnlineStatus(_) => {
            let x = config::ONLINE
                .lock()
                .unwrap()
                .values()
                .max()
                .unwrap_or(&0)
                .clone();
            let confirmed = Config::get_key_confirmed();
            allow_err!(stream.send(&Data::OnlineStatus(Some((x, confirmed)))).await);
        }
        Data::ConfirmedKey(None) => {
            let out = if Config::get_key_confirmed() {
                Some(Config::get_key_pair())
            } else {
                None
            };
            allow_err!(stream.send(&Data::ConfirmedKey(out)).await);
        }
        Data::Socks(s) => match s {
            None => {
                allow_err!(stream.send(&Data::Socks(Config::get_socks())).await);
            }
            Some(data) => {
                if data.proxy.is_empty() {
                    Config::set_socks(None);
                } else {
                    Config::set_socks(Some(data));
                }
                crate::common::test_nat_type();
                RendezvousMediator::restart();
                log::info!("socks updated");
            }
        },
        Data::Config((name, value)) => match value {
            None => {
                let value;
                if name == "id" {
                    value = Some(Config::get_id());
                } else if name == "password" {
                    value = Some(Config::get_password());
                } else if name == "salt" {
                    value = Some(Config::get_salt());
                } else if name == "rendezvous_server" {
                    value = Some(Config::get_rendezvous_server());
                } else if name == "rendezvous_servers" {
                    value = Some(Config::get_rendezvous_servers().join(","));
                } else {
                    value = None;
                }
                allow_err!(stream.send(&Data::Config((name, value))).await);
            }
            Some(value) => {
                if name == "id" {
                    Config::set_key_confirmed(false);
                    Config::set_id(&value);
                } else if name == "password" {
                    Config::set_password(&value);
                } else if name == "salt" {
                    Config::set_salt(&value);
                } else {
                    return;
                }
                log::info!("{} updated", name);
            }
        },
        Data::Options(value) => match value {
            None => {
                let v = Config::get_options();
                allow_err!(stream.send(&Data::Options(Some(v))).await);
            }
            Some(value) => {
                let v0 = Config::get_option("stop-service");
                let v1 = Config::get_rendezvous_servers();
                let v2 = Config::get_option("audio-input");
                Config::set_options(value);
                if v0 != Config::get_option("stop-service")
                    || v1 != Config::get_rendezvous_servers()
                {
                    RendezvousMediator::restart();
                }
                if v2 != Config::get_option("audio-input") {
                    crate::audio_service::restart();
                }
                allow_err!(stream.send(&Data::Options(None)).await);
            }
        },
        Data::NatType(_) => {
            let t = Config::get_nat_type();
            allow_err!(stream.send(&Data::NatType(Some(t))).await);
        }
        Data::SyncConfig(Some((config, config2))) => {
            Config::set(config);
            Config2::set(config2);
            allow_err!(stream.send(&Data::SyncConfig(None)).await);
        }
        Data::SyncConfig(None) => {
            allow_err!(
                stream
                    .send(&Data::SyncConfig(Some((Config::get(), Config2::get()))))
                    .await
            );
        }
        _ => {}
    }
}

pub async fn connect(ms_timeout: u64, postfix: &str) -> ResultType<ConnectionTmpl<ConnClient>> {
    let path = Config::ipc_path(postfix);
    let client = timeout(ms_timeout, Endpoint::connect(&path)).await??;
    Ok(ConnectionTmpl::new(client))
}

#[inline]
#[cfg(not(windows))]
fn get_pid_file(postfix: &str) -> String {
    let path = Config::ipc_path(postfix);
    format!("{}.pid", path)
}

#[cfg(not(windows))]
async fn check_pid(postfix: &str) {
    let pid_file = get_pid_file(postfix);
    if let Ok(mut file) = File::open(&pid_file) {
        let mut content = String::new();
        file.read_to_string(&mut content).ok();
        let pid = content.parse::<i32>().unwrap_or(0);
        if pid > 0 {
            use sysinfo::{ProcessExt, System, SystemExt};
            let mut sys = System::new();
            sys.refresh_processes();
            if let Some(p) = sys.process(pid.into()) {
                if let Some(current) = sys.process((std::process::id() as i32).into()) {
                    if current.name() == p.name() {
                        // double check with connect
                        if connect(1000, postfix).await.is_ok() {
                            return;
                        }
                    }
                }
            }
        }
    }
    hbb_common::allow_err!(std::fs::remove_file(&Config::ipc_path(postfix)));
}

#[inline]
#[cfg(not(windows))]
fn write_pid(postfix: &str) {
    let path = get_pid_file(postfix);
    if let Ok(mut file) = File::create(&path) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o0777)).ok();
        file.write_all(&std::process::id().to_string().into_bytes())
            .ok();
    }
}

pub struct ConnectionTmpl<T> {
    inner: Framed<T, BytesCodec>,
}

pub type Connection = ConnectionTmpl<Conn>;

impl<T> ConnectionTmpl<T>
where
    T: AsyncRead + AsyncWrite + std::marker::Unpin,
{
    pub fn new(conn: T) -> Self {
        Self {
            inner: Framed::new(conn, BytesCodec::new()),
        }
    }

    pub async fn send(&mut self, data: &Data) -> ResultType<()> {
        let v = serde_json::to_vec(data)?;
        self.inner.send(bytes::Bytes::from(v)).await?;
        Ok(())
    }

    async fn send_config(&mut self, name: &str, value: String) -> ResultType<()> {
        self.send(&Data::Config((name.to_owned(), Some(value))))
            .await
    }

    pub async fn next_timeout(&mut self, ms_timeout: u64) -> ResultType<Option<Data>> {
        Ok(timeout(ms_timeout, self.next()).await??)
    }

    pub async fn next_timeout2(&mut self, ms_timeout: u64) -> Option<ResultType<Option<Data>>> {
        if let Ok(x) = timeout(ms_timeout, self.next()).await {
            Some(x)
        } else {
            None
        }
    }

    pub async fn next(&mut self) -> ResultType<Option<Data>> {
        match self.inner.next().await {
            Some(res) => {
                let bytes = res?;
                if let Ok(s) = std::str::from_utf8(&bytes) {
                    if let Ok(data) = serde_json::from_str::<Data>(s) {
                        return Ok(Some(data));
                    }
                }
                return Ok(None);
            }
            _ => {
                bail!("reset by the peer");
            }
        }
    }

    pub async fn send_raw(&mut self, data: Vec<u8>) -> ResultType<()> {
        self.inner.send(bytes::Bytes::from(data)).await?;
        Ok(())
    }

    pub async fn next_raw(&mut self) -> ResultType<bytes::BytesMut> {
        match self.inner.next().await {
            Some(Ok(res)) => Ok(res),
            _ => {
                bail!("reset by the peer");
            }
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn get_config(name: &str) -> ResultType<Option<String>> {
    get_config_async(name, 1_000).await
}

async fn get_config_async(name: &str, ms_timeout: u64) -> ResultType<Option<String>> {
    let mut c = connect(ms_timeout, "").await?;
    c.send(&Data::Config((name.to_owned(), None))).await?;
    if let Some(Data::Config((name2, value))) = c.next_timeout(ms_timeout).await? {
        if name == name2 {
            return Ok(value);
        }
    }
    return Ok(None);
}

#[tokio::main(flavor = "current_thread")]
async fn set_config(name: &str, value: String) -> ResultType<()> {
    let mut c = connect(1000, "").await?;
    c.send_config(name, value).await?;
    Ok(())
}

pub fn set_password(v: String) -> ResultType<()> {
    Config::set_password(&v);
    set_config("password", v)
}

pub fn get_id() -> String {
    if let Ok(Some(v)) = get_config("id") {
        // update salt also, so that next time reinstallation not causing first-time auto-login failure
        if let Ok(Some(v2)) = get_config("salt") {
            Config::set_salt(&v2);
        }
        if v != Config::get_id() {
            Config::set_key_confirmed(false);
            Config::set_id(&v);
        }
        v
    } else {
        Config::get_id()
    }
}

pub fn get_password() -> String {
    if let Ok(Some(v)) = get_config("password") {
        Config::set_password(&v);
        v
    } else {
        Config::get_password()
    }
}

pub async fn get_rendezvous_server(ms_timeout: u64) -> String {
    if let Ok(Some(v)) = get_config_async("rendezvous_server", ms_timeout).await {
        v
    } else {
        Config::get_rendezvous_server()
    }
}

async fn get_options_(ms_timeout: u64) -> ResultType<HashMap<String, String>> {
    let mut c = connect(ms_timeout, "").await?;
    c.send(&Data::Options(None)).await?;
    if let Some(Data::Options(Some(value))) = c.next_timeout(ms_timeout).await? {
        Config::set_options(value.clone());
        Ok(value)
    } else {
        Ok(Config::get_options())
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_options() -> HashMap<String, String> {
    get_options_(1000).await.unwrap_or(Config::get_options())
}

pub fn get_option(key: &str) -> String {
    if let Some(v) = get_options().get(key) {
        v.clone()
    } else {
        "".to_owned()
    }
}

pub fn set_option(key: &str, value: &str) {
    let mut options = get_options();
    if value.is_empty() {
        options.remove(key);
    } else {
        options.insert(key.to_owned(), value.to_owned());
    }
    set_options(options).ok();
}

#[tokio::main(flavor = "current_thread")]
pub async fn set_options(value: HashMap<String, String>) -> ResultType<()> {
    if let Ok(mut c) = connect(1000, "").await {
        c.send(&Data::Options(Some(value.clone()))).await?;
        // do not put below before connect, because we need to check should_exit
        c.next_timeout(1000).await.ok();
    }
    Config::set_options(value);
    Ok(())
}

#[inline]
async fn get_nat_type_(ms_timeout: u64) -> ResultType<i32> {
    let mut c = connect(ms_timeout, "").await?;
    c.send(&Data::NatType(None)).await?;
    if let Some(Data::NatType(Some(value))) = c.next_timeout(ms_timeout).await? {
        Config::set_nat_type(value);
        Ok(value)
    } else {
        Ok(Config::get_nat_type())
    }
}

pub async fn get_nat_type(ms_timeout: u64) -> i32 {
    get_nat_type_(ms_timeout)
        .await
        .unwrap_or(Config::get_nat_type())
}

#[inline]
async fn get_socks_(ms_timeout: u64) -> ResultType<Option<config::Socks5Server>> {
    let mut c = connect(ms_timeout, "").await?;
    c.send(&Data::Socks(None)).await?;
    if let Some(Data::Socks(value)) = c.next_timeout(ms_timeout).await? {
        Config::set_socks(value.clone());
        Ok(value)
    } else {
        Ok(Config::get_socks())
    }
}

pub async fn get_socks_async(ms_timeout: u64) -> Option<config::Socks5Server> {
    get_socks_(ms_timeout).await.unwrap_or(Config::get_socks())
}

#[tokio::main(flavor = "current_thread")]
pub async fn get_socks() -> Option<config::Socks5Server> {
    get_socks_async(1_000).await
}

#[tokio::main(flavor = "current_thread")]
pub async fn set_socks(value: config::Socks5Server) -> ResultType<()> {
    Config::set_socks(if value.proxy.is_empty() {
        None
    } else {
        Some(value.clone())
    });
    connect(1_000, "")
        .await?
        .send(&Data::Socks(Some(value)))
        .await?;
    Ok(())
}

/*
static mut SHARED_MEMORY: *mut i64 = std::ptr::null_mut();

pub fn initialize_shared_memory(create: bool) {
    let mut shmem_flink = "shared-memory".to_owned();
    if cfg!(windows) {
        let df = "C:\\ProgramData";
        let df = if std::path::Path::new(df).exists() {
            df.to_owned()
        } else {
            std::env::var("TEMP").unwrap_or("C:\\Windows\\TEMP".to_owned())
        };
        let df = format!("{}\\{}", df, *hbb_common::config::APP_NAME.read().unwrap());
        std::fs::create_dir(&df).ok();
        shmem_flink = format!("{}\\{}", df, shmem_flink);
    } else {
        shmem_flink = Config::ipc_path("").replace("ipc", "") + &shmem_flink;
    }
    use shared_memory::*;
    let shmem = if create {
        match ShmemConf::new()
            .force_create_flink()
            .size(16)
            .flink(&shmem_flink)
            .create()
        {
            Err(ShmemError::LinkExists) => ShmemConf::new().flink(&shmem_flink).open(),
            Ok(m) => Ok(m),
            Err(e) => Err(e),
        }
    } else {
        ShmemConf::new().flink(&shmem_flink).open()
    };
    if create {
        set_all_perm(&shmem_flink);
    }
    match shmem {
        Ok(shmem) => unsafe {
            SHARED_MEMORY = shmem.as_ptr() as *mut i64;
            std::mem::forget(shmem);
        },
        Err(err) => {
            log::error!(
                "Unable to create or open shmem flink {} : {}",
                shmem_flink,
                err
            );
        }
    }
}

fn set_all_perm(p: &str) {
    #[cfg(not(windows))]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(p, std::fs::Permissions::from_mode(0o0777)).ok();
    }
}
*/
