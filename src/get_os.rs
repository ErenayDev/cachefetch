use std::fs;

pub fn detect_system() -> &'static [&'static str] {
    if cfg!(target_os = "windows") {
        return super::cache_folders::WINDOWS_FOLDERS;
    }

    if cfg!(target_os = "macos") {
        return super::cache_folders::MACOS_FOLDERS;
    }

    let mut folders = super::cache_folders::LINUX_FOLDERS.to_vec();

    if let Ok(contents) = fs::read_to_string("/etc/os-release") {
        if contents.contains("Ubuntu") || contents.contains("Debian") {
            folders.extend_from_slice(super::cache_folders::DEBIAN_UBUNTU_FOLDERS);
        } else if contents.contains("Fedora") {
            folders.extend_from_slice(super::cache_folders::FEDORA_FOLDERS);
        } else if contents.contains("CentOS") || contents.contains("Red Hat") {
            folders.extend_from_slice(super::cache_folders::RHEL_CENTOS_FOLDERS);
        } else if contents.contains("Arch") || contents.contains("Manjaro") {
            folders.extend_from_slice(super::cache_folders::ARCH_FOLDERS);
        } else if contents.contains("openSUSE") {
            folders.extend_from_slice(super::cache_folders::OPENSUSE_FOLDERS);
        }
    }

    Box::leak(folders.into_boxed_slice())
}
