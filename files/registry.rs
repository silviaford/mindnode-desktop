//! Windows registry read helpers for Mindnode Desktop.

pub const APP_KEY: &str = "Software\\Mindnode\\Mindnode Desktop";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Mindnode Desktop";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Mindnode Desktop") },
        RegValue { name: "Version",     value: String::from("v4.5.7") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Mindnode Desktop") },
        RegValue { name: "Publisher",       value: String::from("Mindnode") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Mindnode Desktop\\uninstall.exe\"") },
    ]
}
