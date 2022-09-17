<p align="center">
  <img src="logo-header.svg" alt="RustDesk - Your remote desktop"><br>
  <a href="#free-public-servers">Servers</a> •
  <a href="#raw-steps-to-build">Build</a> •
  <a href="#how-to-build-with-docker">Docker</a> •
  <a href="#file-structure">Structure</a> •
  <a href="#snapshot">Snapshot</a><br>
  [<a href="README-ZH.md">中文</a>] | [<a href="README-ES.md">Español</a>] | [<a href="README-FR.md">Français</a>] | [<a href="README-DE.md">Deutsch</a>] | [<a href="README-PL.md">Polski</a>] | [<a href="README-FI.md">Suomi</a>] | [<a href="README-ML.md">മലയാളം</a>] | [<a href="README-JP.md">日本語</a>] | [<a href="README-NL.md">Nederlands</a>] | [<a href="README-IT.md">Italiano</a>] | [<a href="README-RU.md">Русский</a>] | [<a href="README-PTBR.md">Português (Brasil)</a>] | [<a href="README-EO.md">Esperanto</a>]<br>
  <b>ഈ README നിങ്ങളുടെ മാതൃഭാഷയിലേക്ക് വിവർത്തനം ചെയ്യാൻ ഞങ്ങൾക്ക് നിങ്ങളുടെ സഹായം ആവശ്യമാണ്</b>
</p>

ഞങ്ങളുമായി ചാറ്റ് ചെയ്യുക: [Discord](https://discord.gg/nDceKgxnkV) | [Reddit](https://www.reddit.com/r/rustdesk)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/I2I04VU09)

റസ്റ്റിൽ എഴുതിയ മറ്റൊരു റിമോട്ട് ഡെസ്ക്ടോപ്പ് സോഫ്റ്റ്‌വെയർ. ബോക്‌സിന് പുറത്ത് പ്രവർത്തിക്കുന്നു, കോൺഫിഗറേഷൻ ആവശ്യമില്ല. സുരക്ഷയെക്കുറിച്ച് ആശങ്കകളൊന്നുമില്ലാതെ, നിങ്ങളുടെ ഡാറ്റയുടെ പൂർണ്ണ നിയന്ത്രണം നിങ്ങൾക്കുണ്ട്. നിങ്ങൾക്ക് ഞങ്ങളുടെ rendezvous/relay സെർവർ ഉപയോഗിക്കാം, [സ്വന്തമായി സജ്ജീകരിക്കുക](https://rustdesk.com/server), അല്ലെങ്കിൽ [നിങ്ങളുടെ സ്വന്തം rendezvous/relay സെർവർ എഴുതുക](https://github.com/rustdesk/rustdesk-server-demo).

എല്ലാവരുടെയും സംഭാവനയെ RustDesk സ്വാഗതം ചെയ്യുന്നു. ആരംഭിക്കുന്നതിനുള്ള സഹായത്തിന് [`CONTRIBUTING.md`](CONTRIBUTING.md) കാണുക.

[**BINARY DOWNLOAD**](https://github.com/rustdesk/rustdesk/releases)

## സൗജന്യ പൊതു സെർവറുകൾ

നിങ്ങൾ സൗജന്യമായി ഉപയോഗിക്കുന്ന സെർവറുകൾ ചുവടെയുണ്ട്, അത് സമയത്തിനനുസരിച്ച് മാറിയേക്കാം. നിങ്ങൾ ഇവയിലൊന്നിനോട് അടുത്തല്ലെങ്കിൽ, നിങ്ങളുടെ നെറ്റ്‌വർക്ക് സ്ലോ ആയേക്കാം.
| സ്ഥാനം | കച്ചവടക്കാരൻ | വിവരണം |
| --------- | ------------- | ------------------ |
| Seoul | AWS lightsail | 1 VCPU / 0.5GB RAM |
| Singapore | Vultr | 1 VCPU / 1GB RAM |
| Dallas | Vultr | 1 VCPU / 1GB RAM | |

## ഡിപെൻഡൻസികൾ

ഡെസ്‌ക്‌ടോപ്പ് പതിപ്പുകൾ GUI-യ്‌ക്കായി [sciter](https://sciter.com/) ഉപയോഗിക്കുന്നു, ദയവായി സ്‌സൈറ്റർ ഡൈനാമിക് ലൈബ്രറി സ്വയം ഡൗൺലോഡ് ചെയ്യുക.

[Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll) |
[Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so) |
[MacOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

## നിർമ്മിക്കാനുള്ള അസംസ്കൃത പടികൾ

- നിങ്ങളുടെ Rust development envയും and C++ build envയും തയ്യാറാക്കുക

- [vcpkg](https://github.com/microsoft/vcpkg) ഇൻസ്റ്റാൾ ചെയ്ത് `VCPKG_ROOT` env വേരിയബിൾ ശരിയായി സജ്ജമാക്കുക

  - Windows: vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static
  - Linux/MacOS: vcpkg install libvpx libyuv opus

- run `cargo run`

## ലിനക്സിൽ എങ്ങനെ നിർമ്മിക്കാം

### ഉബുണ്ടു 18 (ഡെബിയൻ 10)

```sh
sudo apt install -y g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake
```

### ഫെഡോറ 28 (CentOS 8)

```sh
sudo yum -y install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel pulseaudio-libs-devel cmake alsa-lib-devel
```

### ആർച് (മഞ്ചാരോ)

```sh
sudo pacman -Syu --needed unzip git cmake gcc curl wget yasm nasm zip make pkg-config clang gtk3 xdotool libxcb libxfixes alsa-lib pulseaudio
```

### vcpkg ഇൻസ്റ്റാൾ ചെയ്യുക

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2021.12.01
cd ..
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus
```

### libvpx പരിഹരിക്കുക (ഫെഡോറയ്ക്ക്)

```sh
cd vcpkg/buildtrees/libvpx/src
cd *
./configure
sed -i 's/CFLAGS+=-I/CFLAGS+=-fPIC -I/g' Makefile
sed -i 's/CXXFLAGS+=-I/CXXFLAGS+=-fPIC -I/g' Makefile
make
cp libvpx.a $HOME/vcpkg/installed/x64-linux/lib/
cd
```

### നിർമാണം

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
mkdir -p target/debug
wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
mv libsciter-gtk.so target/debug
VCPKG_ROOT=$HOME/vcpkg cargo run
```

### വേലാൻഡ് X11 (Xorg) ആയി മാറ്റുക

RustDesk Wayland-നെ പിന്തുണയ്ക്കുന്നില്ല. സ്ഥിരസ്ഥിതി ഗ്നോം സെഷനായി Xorg കോൺഫിഗർ ചെയ്യുന്നതിന് [ഇത്](https://docs.fedoraproject.org/en-US/quick-docs/configuring-xorg-as-default-gnome-session/) പരിശോധിക്കുക.

## ഡോക്കർ ഉപയോഗിച്ച് എങ്ങനെ നിർമ്മിക്കാം

 റെപ്പോസിറ്റോറി ക്ലോണുചെയ്‌ത് ഡോക്കർ കണ്ടെയ്‌നർ നിർമ്മിക്കുന്നതിലൂടെ ആരംഭിക്കുക:

```sh
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
docker build -t "rustdesk-builder" .
```

തുടർന്ന്, ഓരോ തവണയും നിങ്ങൾ ആപ്ലിക്കേഷൻ നിർമ്മിക്കേണ്ടതുണ്ട്, ഇനിപ്പറയുന്ന കമാൻഡ് പ്രവർത്തിപ്പിക്കുക:

```sh
docker run --rm -it -v $PWD:/home/user/rustdesk -v rustdesk-git-cache:/home/user/.cargo/git -v rustdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" rustdesk-builder
```

ഡിപൻഡൻസികൾ കാഷെ ചെയ്യുന്നതിനുമുമ്പ് ആദ്യ ബിൽഡ് കൂടുതൽ സമയമെടുത്തേക്കാം, തുടർന്നുള്ള ബിൽഡുകൾ വേഗത്തിലാകും. കൂടാതെ, നിങ്ങൾക്ക് ബിൽഡ് കമാൻഡിലേക്ക് വ്യത്യസ്ത ആർഗ്യുമെന്റുകൾ വ്യക്തമാക്കണമെങ്കിൽ, കമാൻഡിന്റെ അവസാനം `<OPTIONAL-ARGS>` സ്ഥാനത്ത് നിങ്ങൾക്ക് അങ്ങനെ ചെയ്യാം. ഉദാഹരണത്തിന്, നിങ്ങൾ ഒരു ഒപ്റ്റിമൈസ് ചെയ്ത റിലീസ് പതിപ്പ് നിർമ്മിക്കാൻ ആഗ്രഹിക്കുന്നുവെങ്കിൽ, മുകളിലുള്ള കമാൻഡ് തുടർന്ന് `---release` നിങ്ങൾ പ്രവർത്തിപ്പിക്കും. തത്ഫലമായുണ്ടാകുന്ന എക്സിക്യൂട്ടബിൾ നിങ്ങളുടെ സിസ്റ്റത്തിലെ ടാർഗെറ്റ് ഫോൾഡറിൽ ലഭ്യമാകും, കൂടാതെ ഇത് ഉപയോഗിച്ച് പ്രവർത്തിപ്പിക്കാം:

```sh
target/debug/rustdesk
```

അല്ലെങ്കിൽ, നിങ്ങൾ ഒരു റിലീസ് എക്സിക്യൂട്ടബിൾ പ്രവർത്തിപ്പിക്കുകയാണെങ്കിൽ:

```sh
target/release/rustdesk
```

RustDesk റിപ്പോസിറ്ററിയുടെ റൂട്ടിൽ നിന്നാണ് നിങ്ങൾ ഈ കമാൻഡുകൾ പ്രവർത്തിപ്പിക്കുന്നതെന്ന് ദയവായി ഉറപ്പാക്കുക, അല്ലാത്തപക്ഷം ആപ്ലിക്കേഷന് ആവശ്യമായ ഉറവിടങ്ങൾ കണ്ടെത്താൻ കഴിഞ്ഞേക്കില്ല. ഹോസ്റ്റിന് പകരം കണ്ടെയ്‌നറിനുള്ളിൽ പ്രോഗ്രാം ഇൻസ്റ്റാൾ ചെയ്യുകയോ പ്രവർത്തിപ്പിക്കുകയോ ചെയ്യുന്നതിനാൽ, `install` അല്ലെങ്കിൽ `run` പോലുള്ള മറ്റ് കാർഗോ സബ്‌കമാൻഡുകൾ നിലവിൽ ഈ രീതിയെ പിന്തുണയ്ക്കുന്നില്ല എന്നതും ശ്രദ്ധിക്കുക.

## ഫയൽ ഘടന

- **[libs/hbb_common](https://github.com/rustdesk/rustdesk/tree/master/libs/hbb_common)**: video codec, config, tcp/udp wrapper, protobuf, fs functions for file transfer, and some other utility functions
- **[libs/scrap](https://github.com/rustdesk/rustdesk/tree/master/libs/scrap)**: screen capture
- **[libs/enigo](https://github.com/rustdesk/rustdesk/tree/master/libs/enigo)**: platform specific keyboard/mouse control
- **[src/ui](https://github.com/rustdesk/rustdesk/tree/master/src/ui)**: GUI
- **[src/server](https://github.com/rustdesk/rustdesk/tree/master/src/server)**: audio/clipboard/input/video services, and network connections
- **[src/client.rs](https://github.com/rustdesk/rustdesk/tree/master/src/client.rs)**: start a peer connection
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/rustdesk/tree/master/src/rendezvous_mediator.rs)**: Communicate with [rustdesk-server](https://github.com/rustdesk/rustdesk-server), wait for remote direct (TCP hole punching) or relayed connection
- **[src/platform](https://github.com/rustdesk/rustdesk/tree/master/src/platform)**: platform specific code

## സ്നാപ്പ്ഷോട്ടുകൾ

![image](https://user-images.githubusercontent.com/71636191/113112362-ae4deb80-923b-11eb-957d-ff88daad4f06.png)

![image](https://user-images.githubusercontent.com/71636191/113112619-f705a480-923b-11eb-911d-97e984ef52b6.png)

![image](https://user-images.githubusercontent.com/71636191/113112857-3fbd5d80-923c-11eb-9836-768325faf906.png)

![image](https://user-images.githubusercontent.com/71636191/135385039-38fdbd72-379a-422d-b97f-33df71fb1cec.png)
