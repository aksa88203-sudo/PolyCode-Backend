# Cross-Platform Development in Rust

## Overview

Cross-platform development enables Rust applications to run on multiple operating systems and architectures. This guide covers platform-specific considerations, conditional compilation, and building portable applications in Rust.

---

## Platform Detection

### Compile-Time Platform Detection

```rust
// Using cfg attributes for conditional compilation
#[cfg(target_os = "windows")]
fn windows_specific() {
    println!("Running on Windows");
    // Windows-specific code
    use std::os::windows::ffi::OsStrExt;
    let path = std::path::Path::new("C:\\Program Files");
}

#[cfg(target_os = "macos")]
fn macos_specific() {
    println!("Running on macOS");
    // macOS-specific code
    use std::os::macos::ffi::OsStrExt;
    let path = std::path::Path::new("/Applications");
}

#[cfg(target_os = "linux")]
fn linux_specific() {
    println!("Running on Linux");
    // Linux-specific code
    use std::os::linux::fs::MetadataExt;
    let path = std::path::Path::new("/usr/local/bin");
}

#[cfg(target_os = "android")]
fn android_specific() {
    println!("Running on Android");
    // Android-specific code
}

#[cfg(target_os = "ios")]
fn ios_specific() {
    println!("Running on iOS");
    // iOS-specific code
}

// Multiple platforms
#[cfg(any(target_os = "windows", target_os = "macos"))]
fn desktop_platforms() {
    println!("Running on desktop platform");
}

// Negation
#[cfg(not(target_os = "windows"))]
fn non_windows() {
    println!("Not running on Windows");
}
```

### Runtime Platform Detection

```rust
use std::env;

fn detect_platform() {
    let os = env::consts::OS;
    
    match os {
        "windows" => println!("Windows detected"),
        "macos" => println!("macOS detected"),
        "linux" => println!("Linux detected"),
        "android" => println!("Android detected"),
        "ios" => println!("iOS detected"),
        _ => println!("Unknown OS: {}", os),
    }
    
    // Architecture detection
    let arch = env::consts::ARCH;
    println!("Architecture: {}", arch);
    
    // Platform-specific paths
    let home_dir = if cfg!(target_os = "windows") {
        env::var("USERPROFILE").unwrap_or_else(|_| {
            env::var("HOME").unwrap_or_else(|_| {
                "C:\\".to_string()
            })
        })
    } else {
        env::var("HOME").unwrap_or_else(|_| "/".to_string())
    };
    
    println!("Home directory: {}", home_dir);
}
```

---

## File System Abstraction

### Cross-Platform File Operations

```rust
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct PlatformPaths {
    config_dir: PathBuf,
    data_dir: PathBuf,
    cache_dir: PathBuf,
    temp_dir: PathBuf,
    log_dir: PathBuf,
}

impl PlatformPaths {
    fn new() -> Self {
        let base_dir = Self::get_base_directory();
        
        PlatformPaths {
            config_dir: base_dir.join("config"),
            data_dir: base_dir.join("data"),
            cache_dir: base_dir.join("cache"),
            temp_dir: std::env::temp_dir(),
            log_dir: base_dir.join("logs"),
        }
    }
    
    fn get_base_directory() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    env::var("USERPROFILE")
                        .map(|p| PathBuf::from(p).join("AppData"))
                        .unwrap_or_else(|_| PathBuf::from("C:\\"))
                })
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"))
        }
        .map(|p| p.join(".myapp"))
    }
    
    fn ensure_directory_exists(&self, path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
            println!("Created directory: {:?}", path);
        }
        Ok(())
    }
    
    fn initialize(&self) -> std::io::Result<()> {
        println!("Initializing platform paths...");
        
        self.ensure_directory_exists(&self.config_dir)?;
        self.ensure_directory_exists(&self.data_dir)?;
        self.ensure_directory_exists(&self.cache_dir)?;
        self.ensure_directory_exists(&self.log_dir)?;
        
        println!("Platform paths initialized:");
        println!("  Config: {:?}", self.config_dir);
        println!("  Data: {:?}", self.data_dir);
        println!("  Cache: {:?}", self.cache_dir);
        println!("  Temp: {:?}", self.temp_dir);
        println!("  Logs: {:?}", self.log_dir);
        
        Ok(())
    }
}

// Cross-platform file operations
fn cross_platform_file_ops() -> std::io::Result<()> {
    let paths = PlatformPaths::new();
    paths.initialize()?;
    
    // Write config file
    let config_file = paths.config_dir.join("app.conf");
    let config_content = r#"
[application]
name = "MyApp"
version = "1.0.0"
debug = true

[database]
url = "sqlite:./app.db"
max_connections = 10
    "#;
    
    std::fs::write(&config_file, config_content)?;
    println!("Config file written: {:?}", config_file);
    
    // Read config file
    let read_content = std::fs::read_to_string(&config_file)?;
    println!("Config file content:\n{}", read_content);
    
    // Platform-specific file permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        
        let mut perms = std::fs::metadata(&config_file)?.permissions();
        perms.set_mode(0o644); // rw-r--r--
        std::fs::set_permissions(&config_file, perms)?;
        println!("Set file permissions on Unix");
    }
    
    Ok(())
}
```

### Path Handling

```rust
use std::path::{Component, Path, PathBuf};

fn path_operations() {
    let path = Path::new("/home/user/documents/file.txt");
    
    // Path components
    println!("Path: {:?}", path);
    println!("Parent: {:?}", path.parent());
    println!("File name: {:?}", path.file_name());
    println!("File stem: {:?}", path.file_stem());
    println!("Extension: {:?}", path.extension());
    
    // Path joining
    let base = Path::new("/home/user");
    let file = base.join("documents").join("file.txt");
    println!("Joined path: {:?}", file);
    
    // Platform-specific path separators
    let path_str = if cfg!(target_os = "windows") {
        "C:\\Users\\User\\Documents\\file.txt"
    } else {
        "/home/user/documents/file.txt"
    };
    
    let path = Path::new(path_str);
    println!("Platform-specific path: {:?}", path);
    
    // Path normalization
    let messy_path = Path::new("/home//user/../user/./documents/");
    let normalized = messy_path.components().collect::<PathBuf>();
    println!("Normalized path: {:?}", normalized);
    
    // Check path properties
    println!("Is absolute: {}", path.is_absolute());
    println!("Is relative: {}", path.is_relative());
    println!("Exists: {}", path.exists());
    
    // Create directories in path
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).unwrap();
            println!("Created parent directory: {:?}", parent);
        }
    }
}
```

---

## Platform-Specific Features

### Windows Integration

```rust
#[cfg(target_os = "windows")]
mod windows_integration {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use std::ptr;
    use winapi::um::winuser;
    use winapi::shared::windef;
    use winapi::shared::minwindef::HWND;
    
    pub fn get_windows_version() -> String {
        let mut info: OSVERSIONINFOEXW = unsafe { std::mem::zeroed() };
        info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOEXW>() as u32;
        
        unsafe {
            RtlGetVersionExW(&mut info);
        }
        
        format!("{}.{}.{}",
                info.dwMajorVersion,
                info.dwMinorVersion)
    }
    
    pub fn show_message_box(title: &str, message: &str) {
        let title_wide: Vec<u16> = OsString::from(title).encode_wide().collect();
        let message_wide: Vec<u16> = OsString::from(message).encode_wide().collect();
        
        unsafe {
            MessageBoxW(
                ptr::null_mut(),
                title_wide.as_ptr(),
                message_wide.as_ptr(),
                MB_OK | MB_ICONINFORMATION
            );
        }
    }
    
    pub fn get_known_folder(folder_id: u32) -> Option<String> {
        let mut path: [u16; MAX_PATH] = [0; MAX_PATH];
        
        unsafe {
            if SHGetFolderPathW(
                ptr::null_mut(),
                folder_id,
                ptr::null(),
                CSIDL_FLAG_CREATE,
                ptr::null_mut(),
                path.as_mut_ptr()
            ) == S_OK {
                if let Some(end) = path.iter().position(|&c| c == 0) {
                    let os_string = OsString::from_wide(&path[..end]);
                    os_string.into_string().ok()
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
    
    pub fn get_documents_folder() -> Option<String> {
        get_known_folder(CSID_PERSONAL)
    }
    
    pub fn get_desktop_folder() -> Option<String> {
        get_known_folder(CSID_DESKTOP)
    }
    
    pub fn get_app_data_folder() -> Option<String> {
        get_known_folder(CSID_LOCAL_APPDATA)
    }
}

#[cfg(target_os = "windows")]
fn windows_demo() {
    println!("=== WINDOWS INTEGRATION ===");
    
    windows_integration::get_windows_version();
    
    if let Some(documents) = windows_integration::get_documents_folder() {
        println!("Documents folder: {}", documents);
    }
    
    if let Some(desktop) = windows_integration::get_desktop_folder() {
        println!("Desktop folder: {}", desktop);
    }
    
    windows_integration::show_message_box("Hello", "This is a Windows message box");
}
```

### macOS Integration

```rust
#[cfg(target_os = "macos")]
mod macos_integration {
    use std::ffi::CStr;
    use std::os::macos::ffi::OsStrExt;
    
    pub fn get_system_info() -> SystemInfo {
        SystemInfo {
            os_version: get_macos_version(),
            model: get_mac_model(),
            architecture: get_mac_architecture(),
        }
    }
    
    fn get_macos_version() -> String {
        // Use system_profiler or similar in production
        "macOS".to_string()
    }
    
    fn get_mac_model() -> String {
        // Use system_profiler in production
        "Mac".to_string()
    }
    
    fn get_mac_architecture() -> String {
        // Use system_profiler in production
        "x86_64".to_string()
    }
    
    pub fn show_notification(title: &str, message: &str) {
        // Use user notifications in production
        println!("macOS Notification: {} - {}", title, message);
    }
    
    pub fn get_application_support_directory() -> Option<String> {
        use std::env;
        
        env::var("HOME")
            .map(|home| format!("{}/Library/Application Support", home))
            .ok()
    }
    
    pub fn open_file_with_default_app(path: &str) -> std::io::Result<()> {
        use std::process::Command;
        
        Command::new("open")
            .arg(path)
            .spawn()?;
        
        Ok(())
    }
    
    pub fn open_url_in_browser(url: &str) -> std::io::Result<()> {
        open_file_with_default_app(url)
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    pub os_version: String,
    pub model: String,
    pub architecture: String,
}

#[cfg(target_os = "macos")]
fn macos_demo() {
    println!("=== MACOS INTEGRATION ===");
    
    let info = macos_integration::get_system_info();
    println!("System info: {:?}", info);
    
    if let Some(app_support) = macos_integration::get_application_support_directory() {
        println!("Application Support: {}", app_support);
    }
    
    macos_integration::show_notification("Rust App", "Hello from macOS!");
    
    macos_integration::open_url_in_browser("https://www.rust-lang.org").unwrap();
}
```

### Linux Integration

```rust
#[cfg(target_os = "linux")]
mod linux_integration {
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    
    pub fn get_distribution_info() -> DistributionInfo {
        DistributionInfo {
            name: get_distribution_name(),
            version: get_distribution_version(),
            desktop_environment: get_desktop_environment(),
        }
    }
    
    fn get_distribution_name() -> String {
        if Path::new("/etc/os-release").exists() {
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("ID=") {
                        return line.split('=').nth(1).unwrap_or("unknown").trim_matches('"');
                    }
                }
            }
        }
        "Linux".to_string()
    }
    
    fn get_distribution_version() -> String {
        if Path::new("/etc/os-release").exists() {
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("VERSION_ID=") {
                        return line.split('=').nth(1).unwrap_or("unknown").trim_matches('"');
                    }
                }
            }
        }
        "unknown".to_string()
    }
    
    fn get_desktop_environment() -> String {
        std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "unknown".to_string())
    }
    
    pub fn get_config_directories() -> ConfigDirectories {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
        
        ConfigDirectories {
            config: Path::new(&home).join(".config"),
            cache: Path::new(&home).join(".cache"),
            data: std::env::var("XDG_DATA_HOME")
                .unwrap_or_else(|_| Path::new(&home).join(".local/share")),
            runtime: std::env::var("XDG_RUNTIME_DIR")
                .unwrap_or_else(|_| Path::new(&home).join(".cache")),
        }
    }
    
    pub fn show_desktop_notification(title: &str, message: &str) -> bool {
        Command::new("notify-send")
            .arg(title)
            .arg(message)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
    
    pub fn open_file_manager(path: &str) -> std::io::Result<()> {
        Command::new("xdg-open")
            .arg(path)
            .spawn()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct DistributionInfo {
    pub name: String,
    pub version: String,
    pub desktop_environment: String,
}

#[derive(Debug)]
pub struct ConfigDirectories {
    pub config: std::path::PathBuf,
    pub cache: std::path::PathBuf,
    pub data: std::path::PathBuf,
    pub runtime: std::path::PathBuf,
}

#[cfg(target_os = "linux")]
fn linux_demo() {
    println!("=== LINUX INTEGRATION ===");
    
    let distro = linux_integration::get_distribution_info();
    println!("Distribution: {:?}", distro);
    
    let config_dirs = linux_integration::get_config_directories();
    println!("Config directories: {:?}", config_dirs);
    
    linux_integration::show_desktop_notification(
        "Rust App",
        "Hello from Linux!"
    );
    
    linux_integration::open_file_manager("/home/user/documents").unwrap();
}
```

---

## Conditional Compilation

### Feature Flags

```rust
// Cargo.toml
[features]
default = []
windows = []
macos = []
linux = []
gui = ["windows", "macos"]
cli = ["windows", "macos", "linux"]
all = ["windows", "macos", "linux"]

// Conditional compilation based on features
#[cfg(feature = "windows")]
fn windows_feature() {
    println!("Windows-specific features enabled");
    // Windows GUI code
}

#[cfg(feature = "gui")]
fn gui_feature() {
    println!("GUI features enabled");
    // Cross-platform GUI code
}

#[cfg(feature = "cli")]
fn cli_feature() {
    println!("CLI features enabled");
    // Command-line interface code
}

// Default implementation
#[cfg(not(any(feature = "windows", feature = "macos", feature = "linux"))]
fn default_implementation() {
    println!("No platform-specific features enabled");
}
```

### Architecture Detection

```rust
// Architecture-specific code
#[cfg(target_arch = "x86_64")]
fn x86_64_code() {
    println!("Running on x86_64");
}

#[cfg(target_arch = "aarch64")]
fn aarch64_code() {
    println!("Running on ARM64");
}

#[cfg(target_pointer_width = "64")]
fn sixty_four_bit() {
    println!("64-bit architecture");
}

#[cfg(target_pointer_width = "32")]
fn thirty_two_bit() {
    println!("32-bit architecture");
}

// Endianness detection
fn detect_endianness() {
    println!("Endianness: {}", if cfg!(target_endian = "little") { "little" } else { "big" });
}

// SIMD feature detection
#[cfg(target_feature = "avx2")]
fn avx2_optimized() {
    println!("AVX2 optimizations available");
}

#[cfg(target_feature = "neon")]
fn neon_optimized() {
    println!("NEON optimizations available");
}
```

---

## Package Management

### Cross-Platform Dependencies

```toml
# Cargo.toml
[package]
name = "cross-platform-app"
version = "0.1.0"
edition = "2021"

[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser", "shellapi"] }
windows = "0.48"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.24"
core-foundation = "0.9"

[target.'cfg(target_os = "linux")'.dependencies]
gtk = "0.15"
libx = "0.15"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"

# Platform-specific dependencies
[target.'cfg(unix)'.dependencies]
nix = "0.26"

[target.'cfg(windows)'.dependencies]
winreg = "0.51"
```

### Conditional Dependencies

```rust
// Platform-specific module structure
#[cfg(target_os = "windows")]
mod platform {
    pub mod file_system;
    pub mod ui;
    pub mod notifications;
}

#[cfg(target_os = "macos")]
mod platform {
    pub mod file_system;
    pub mod ui;
    pub mod notifications;
}

#[cfg(target_os = "linux")]
mod platform {
    pub mod file_system;
    pub mod ui;
    pub mod notifications;
}

// Common interface
pub trait FileSystem {
    fn read_file(&self, path: &str) -> Result<String, Box<dyn std::error::Error>>;
    fn write_file(&self, path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait UserInterface {
    fn show_message(&self, title: &str, message: &str);
    fn get_user_input(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>>;
}

pub trait Notifications {
    fn show_notification(&self, title: &str, message: &str);
    fn show_error(&self, error: &str);
}

// Platform-specific implementations
#[cfg(target_os = "windows")]
mod platform {
    pub mod file_system {
        use std::fs;
        use super::FileSystem;
        
        pub struct WindowsFileSystem;
        
        impl FileSystem for WindowsFileSystem {
            fn read_file(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
                fs::read_to_string(path).map_err(Into::into)
            }
            
            fn write_file(&self, path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
                fs::write(path, content).map_err(Into::into)
            }
        }
    }
}

// Main application using platform abstraction
struct CrossPlatformApp {
    file_system: Box<dyn FileSystem>,
    ui: Box<dyn UserInterface>,
    notifications: Box<dyn Notifications>,
}

impl CrossPlatformApp {
    fn new() -> Self {
        CrossPlatformApp {
            file_system: Box::new(platform::file_system::WindowsFileSystem),
            ui: Box::new(platform::ui::WindowsUI),
            notifications: Box::new(platform::notifications::WindowsNotifications),
        }
    }
    
    #[cfg(target_os = "macos")]
    fn new() -> Self {
        CrossPlatformApp {
            file_system: Box::new(platform::file_system::MacFileSystem),
            ui: Box::new(platform::ui::MacUI),
            notifications: Box::new(platform::notifications::MacNotifications),
        }
    }
    
    #[cfg(target_os = "linux")]
    fn new() -> Self {
        CrossPlatformApp {
            file_system: Box::new(platform::file_system::LinuxFileSystem),
            ui: Box::new(platform::ui::LinuxUI),
            notifications: Box::new(platform::notifications::LinuxNotifications),
        }
    }
    
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Use platform-agnostic interface
        let content = self.file_system.read_file("config.txt")?;
        self.ui.show_message("Config", &content);
        self.notifications.show_notification("App Started", "Configuration loaded");
        
        Ok(())
    }
}
```

---

## Key Takeaways

- **cfg attributes** enable conditional compilation
- **Platform detection** helps adapt behavior at runtime
- **Standard library** provides cross-platform abstractions
- **Platform-specific crates** handle OS-specific features
- **Feature flags** control optional functionality
- **Path handling** abstracts platform differences
- **Package management** supports platform-specific dependencies

---

## Cross-Platform Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Use std library** | Prefer standard library abstractions | Path, fs, env modules |
| **Conditional compilation** | Use cfg for platform-specific code | cfg!(target_os), cfg!(target_arch) |
| **Feature flags** | Enable optional functionality | [features] in Cargo.toml |
| **Test on all platforms** | Verify cross-platform compatibility | CI/CD on multiple OS |
| **Handle edge cases** | Consider different path formats | Path separators, case sensitivity |
| **Document limitations** | Clearly state platform support | README and documentation |
| **Use external crates** | Leverage platform-specific libraries | winapi, cocoa, gtk |
