lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Your desktop can be accessed with this ID and password."),
        ("connecting_status", "Connecting to the GerarDesk network..."),
        ("not_ready_status", "Not ready. Please check your connection"),
        ("id_change_tip", "Only a-z, A-Z, 0-9 and _ (underscore) characters allowed. The first letter must be a-z, A-Z. Length between 6 and 16."),
        ("install_tip", "Due to UAC, GerarDesk can not work properly as the remote side in some cases. To avoid UAC, please click the button below to install GerarDesk to the system."),
        ("config_acc", "In order to control your Desktop remotely, you need to grant GerarDesk \"Accessibility\" permissions."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant GerarDesk \"Screen Recording\" permissions."),
        ("agreement_tip", "By starting the installation, you accept the license agreement."),
        ("not_close_tcp_tip", "Don't close this window while you are using the tunnel"),
        ("setup_server_tip", ""),// "For faster connection, please set up your own server"),
        ("Auto Login", "Auto Login (Only valid if you set \"Lock after session end\")"),
        ("whitelist_tip", "Only whitelisted IP can access me"),
        ("whitelist_sep", "Seperated by comma, semicolon, spaces or new line"),
        ("Wrong credentials", "Wrong username or password"),
        ("invalid_http", "must start with http:// or https://"),
        ("install_daemon_tip", "For starting on boot, you need to install system service."),
        ("android_input_permission_tip1", "In order for a remote device to control your Android device via mouse or touch, you need to allow GerarDesk to use the \"Accessibility\" service."),
        ("android_input_permission_tip2", "Please go to the next system settings page, find and enter [Installed Services], turn on [GerarDesk Input] service."),
        ("android_new_connection_tip", "New control request has been received, which wants to control your current device."),
        ("android_service_will_start_tip", "Turning on \"Screen Capture\" will automatically start the service, allowing other devices to request a connection to your device."),
        ("android_stop_service_tip", "Closing the service will automatically close all established connections."),
        ("android_version_audio_tip", "The current Android version does not support audio capture, please upgrade to Android 10 or higher."),
        ("android_start_service_tip", "Tap [Start Service] or OPEN [Screen Capture] permission to start the screen sharing service."),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("doc_fix_wayland", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
        ("server_not_support", "Not yet supported by the server"),
        ("android_open_battery_optimizations_tip", "If you want to disable this feature, please go to the next GerarDesk application settings page, find and enter [Battery], Uncheck [Unrestricted]"),
        ("remote_restarting_tip", "Remote device is restarting, please close this message box and reconnect with permanent password after a while"),
        ("Are you sure to close the connection?", "Are you sure you want to close the connection?"),
        ("elevated_foreground_window_tip", "The current window of the remote desktop requires higher privilege to operate, so it's unable to use the mouse and keyboard temporarily. You can request the remote user to minimize the current window, or click elevation button on the connection management window. To avoid this problem, it is recommended to install the software on the remote device."),
        ("JumpLink", "View"),
        ("Stop service", "Stop Service"),
        ("hide_cm_tip", "Allow hiding only if accepting sessions via password and using permanent password"),
        ("wayland_experiment_tip", "Wayland support is in experimental stage, please use X11 if you require unattended access."),
        ].iter().cloned().collect();
}
