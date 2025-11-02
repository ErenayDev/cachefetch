pub const LINUX_FOLDERS: &[&str] = &[
    "~/.cache/pip",
    "~/.cache/yarn",
    "~/.cache/pnpm",
    "~/.cache/deno",
    "~/.cache/go-build",
    "~/.cache/pre-commit",
    "~/.cache/black",
    "~/.cache/mypy",
    "~/.cache/eslint",
    "~/.cache/prettier",
    "~/.npm/_cacache",
    "~/.npm/_logs",
    "~/.yarn/cache",
    "~/.bun/install/cache",
    "~/.cargo/registry/cache",
    "~/.cargo/registry/src",
    "~/.cargo/git/db",
    "~/.gem/cache",
    "~/.bundle/cache",
    "~/.composer/cache",
    "~/.cache/composer",
    "~/.gradle/caches",
    "~/.sbt/boot",
    "~/.ivy2/cache",
    "~/.nuget/packages",
];

pub const DEBIAN_UBUNTU_FOLDERS: &[&str] = &["/var/cache/apt/archives", "/var/cache/debconf"];

pub const RHEL_CENTOS_FOLDERS: &[&str] = &["/var/cache/yum", "/var/cache/dnf"];

pub const FEDORA_FOLDERS: &[&str] = &["/var/cache/dnf"];

pub const ARCH_FOLDERS: &[&str] = &["/var/cache/pacman/pkg", "~/.cache/yay", "~/.cache/paru"];

pub const OPENSUSE_FOLDERS: &[&str] = &["/var/cache/zypp/packages"];

pub const MACOS_FOLDERS: &[&str] = &[
    "~/Library/Caches/pip",
    "~/Library/Caches/yarn",
    "~/Library/Caches/pnpm",
    "~/Library/Caches/deno",
    "~/Library/Caches/go-build",
    "~/Library/Caches/com.github.wez.wezterm",
    "~/Library/Caches/Homebrew",
    "~/.npm/_cacache",
    "~/.npm/_logs",
    "~/.yarn/cache",
    "~/.bun/install/cache",
    "~/.cargo/registry/cache",
    "~/.cargo/registry/src",
    "~/.cargo/git/db",
    "~/.gem/cache",
    "~/.bundle/cache",
    "~/.composer/cache",
    "~/.gradle/caches",
    "~/.sbt/boot",
    "~/.ivy2/cache",
    "~/.nuget/packages",
];

pub const WINDOWS_FOLDERS: &[&str] = &[
    "%LOCALAPPDATA%\\pip\\cache",
    "%LOCALAPPDATA%\\yarn\\cache",
    "%LOCALAPPDATA%\\pnpm\\cache",
    "%LOCALAPPDATA%\\deno",
    "%LOCALAPPDATA%\\go-build",
    "%APPDATA%\\npm-cache",
    "%USERPROFILE%\\.cargo\\registry\\cache",
    "%USERPROFILE%\\.cargo\\registry\\src",
    "%USERPROFILE%\\.cargo\\git\\db",
    "%USERPROFILE%\\.gem\\cache",
    "%USERPROFILE%\\.bundle\\cache",
    "%APPDATA%\\Composer\\cache",
    "%USERPROFILE%\\.gradle\\caches",
    "%USERPROFILE%\\.sbt\\boot",
    "%USERPROFILE%\\.ivy2\\cache",
    "%USERPROFILE%\\.nuget\\packages",
    "%TEMP%",
    "%LOCALAPPDATA%\\Temp",
];
