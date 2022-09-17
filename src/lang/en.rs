lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Your desktop can be accessed with this ID and password."),
        ("connecting_status", "Connecting to the RustDesk network..."),
        ("not_ready_status", "Not ready. Please check your connection"),
        ("id_change_tip", "Only a-z, A-Z, 0-9 and _ (underscore) characters allowed. The first letter must be a-z, A-Z. Length between 6 and 16."),
        ("install_tip", "Due to UAC, RustDesk can not work properly as the remote side in some cases. To avoid UAC, please click the button below to install RustDesk to the system."),
        ("config_acc", "In order to control your Desktop remotely, you need to grant RustDesk \"Accessibility\" permissions."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant RustDesk \"Screen Recording\" permissions."),
        ("agreement_tip", "By starting the installation, you accept the license agreement."),
        ("not_close_tcp_tip", "Don't close this window while you are using the tunnel"),
        ("setup_server_tip", "For faster connection, please set up your own server"),
        ("Auto Login", "Auto Login (Only valid if you set \"Lock after session end\")"),
        ("whitelist_tip", "Only whitelisted IP can access me"),
        ("whitelist_sep", "Seperated by comma, semicolon, spaces or new line"),
        ("Wrong credentials", "Wrong username or password"),
        ("invalid_http", "must start with http:// or https://"),
        ("install_daemon_tip", "For starting on boot, you need to install system service."),
        ("android_input_permission_tip1", "In order for a remote device to control your Android device via mouse or touch, you need to allow RustDesk to use the \"Accessibility\" service."),
        ("android_input_permission_tip2", "Please go to the next system settings page, find and enter [Installed Services], turn on [RustDesk Input] service."),
        ("android_new_connection_tip", "New control request has been received, which wants to control your current device."),
        ("android_service_will_start_tip", "Turning on \"Screen Capture\" will automatically start the service, allowing other devices to request a connection to your device."),
        ("android_stop_service_tip", "Closing the service will automatically close all established connections."),
        ("android_version_audio_tip", "The current Android version does not support audio capture, please upgrade to Android 10 or higher."),
        ("android_start_service_tip", "Tap [Start Service] or OPEN [Screen Capture] permission to start the screen sharing service."),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
    ].iter().cloned().collect();
}
