// cross_platform_development.rs
// Comprehensive examples of cross-platform development in Rust

use std::env;
use std::path::{Path, PathBuf};

// =========================================
// PLATFORM DETECTION
// =========================================

pub fn detect_platform() {
    println!("=== PLATFORM DETECTION ===");
    
    // Operating system detection
    let os = env::consts::OS;
    println!("Operating System: {}", os);
    
    // Architecture detection
    let arch = env::consts::ARCH;
    println!("Architecture: {}", arch);
    
    // Family detection
    let family = env::consts::FAMILY;
    println!("Family: {}", family);
    
    // Endianness detection
    let endian = if cfg!(target_endian = "little") {
        "little"
    } else {
        "big"
    };
    println!("Endianness: {}", endian);
    
    // Pointer width
    let pointer_width = env::consts::WIDTH;
    println!("Pointer width: {} bits", pointer_width);
    
    // Platform-specific information
    match os {
        "windows" => println!("Platform: Windows"),
        "macos" => println!("Platform: macOS"),
        "linux" => println!("Platform: Linux"),
        "android" => println!("Platform: Android"),
        "ios" => println!("Platform: iOS"),
        _ => println!("Platform: Unknown"),
    }
}

// =========================================
// CONDITIONAL COMPILATION EXAMPLES
// =========================================

// Compile-time platform detection
#[cfg(target_os = "windows")]
pub fn windows_specific() {
    println!("Windows-specific code executing");
    // Windows-specific code would go here
}

#[cfg(target_os = "macos")]
pub fn macos_specific() {
    println!("macOS-specific code executing");
    // macOS-specific code would go here
}

#[cfg(target_os = "linux")]
pub fn linux_specific() {
    println!("Linux-specific code executing");
    // Linux-specific code would go here
}

// Multiple platforms
#[cfg(any(target_os = "windows", target_os = "macos"))]
pub fn desktop_platforms() {
    println!("Desktop platform code executing");
}

// Negation
#[cfg(not(target_os = "windows"))]
pub fn non_windows() {
    println!("Non-Windows code executing");
}

// Architecture-specific code
#[cfg(target_arch = "x86_64")]
pub fn x86_64_code() {
    println!("x86_64 architecture code executing");
}

#[cfg(target_arch = "aarch64")]
pub fn aarch64_code() {
    println!("ARM64 architecture code executing");
}

// Pointer width
#[cfg(target_pointer_width = "64")]
pub fn sixty_four_bit() {
    println!("64-bit code executing");
}

#[cfg(target_pointer_width = "32")]
pub fn thirty_two_bit() {
    println!("32-bit code executing");
}

// =========================================
// RUNTIME PLATFORM DETECTION
// =========================================

pub fn runtime_platform_detection() {
    println!("=== RUNTIME PLATFORM DETECTION ===");
    
    // Environment variables
    if let Ok(home) = env::var("HOME") {
        println!("HOME: {}", home);
    }
    
    if let Ok(userprofile) = env::var("USERPROFILE") {
        println!("USERPROFILE: {}", userprofile);
    }
    
    if let Ok(path) = env::var("PATH") {
        println!("PATH length: {} characters", path.len());
    }
    
    // Platform-specific path handling
    let separator = if cfg!(target_os = "windows") {
        ';'
    } else {
        ':'
    };
    
    println!("Path separator: {}", separator);
    
    // Current working directory
    if let Ok(cwd) = env::current_dir() {
        println!("Current directory: {}", cwd.display());
    }
}

// =========================================
// CROSS-PLATFORM FILE SYSTEM
// =========================================

#[derive(Debug, Clone)]
pub struct PlatformPaths {
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub temp_dir: PathBuf,
    pub log_dir: PathBuf,
    pub home_dir: PathBuf,
}

impl PlatformPaths {
    pub fn new() -> Self {
        let home_dir = Self::get_home_directory();
        
        PlatformPaths {
            config_dir: Self::get_config_directory(&home_dir),
            data_dir: Self::get_data_directory(&home_dir),
            cache_dir: Self::get_cache_directory(&home_dir),
            temp_dir: Self::get_temp_directory(),
            log_dir: Self::get_log_directory(&home_dir),
            home_dir,
        }
    }
    
    fn get_home_directory() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("USERPROFILE")
                .map(PathBuf::from)
                .unwrap_or_else(|_| {
                    env::var("HOME")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| PathBuf::from("C:\\"))
                })
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/"))
        }
    }
    
    fn get_config_directory(home_dir: &Path) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join("AppData"))
                .join("MyApp")
        }
        
        #[cfg(target_os = "macos")]
        {
            home_dir.join("Library").join("Application Support").join("MyApp")
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            env::var("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".config"))
                .join("myapp")
        }
    }
    
    fn get_data_directory(home_dir: &Path) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join("AppData"))
                .join("MyApp")
                .join("Data")
        }
        
        #[cfg(target_os = "macos")]
        {
            home_dir.join("Library").join("Application Support").join("MyApp")
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            env::var("XDG_DATA_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".local/share"))
                .join("myapp")
        }
    }
    
    fn get_cache_directory(home_dir: &Path) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("TEMP")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join("AppData").join("Local").join("Temp"))
                .join("MyApp")
        }
        
        #[cfg(target_os = "macos")]
        {
            home_dir.join("Library").join("Caches").join("MyApp")
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            env::var("XDG_CACHE_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".cache"))
                .join("myapp")
        }
    }
    
    fn get_temp_directory() -> PathBuf {
        env::temp_dir()
    }
    
    fn get_log_directory(home_dir: &Path) -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join("AppData"))
                .join("MyApp")
                .join("Logs")
        }
        
        #[cfg(target_os = "macos")]
        {
            home_dir.join("Library").join("Logs").join("MyApp")
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "macos"))]
        {
            env::var("XDG_STATE_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| home_dir.join(".local/state"))
                .join("myapp")
        }
    }
    
    pub fn ensure_directory_exists(&self, path: &Path) -> std::io::Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
            println!("Created directory: {}", path.display());
        }
        Ok(())
    }
    
    pub fn initialize(&self) -> std::io::Result<()> {
        println!("=== INITIALIZING PLATFORM PATHS ===");
        
        self.ensure_directory_exists(&self.config_dir)?;
        self.ensure_directory_exists(&self.data_dir)?;
        self.ensure_directory_exists(&self.cache_dir)?;
        self.ensure_directory_exists(&self.log_dir)?;
        
        println!("Platform paths initialized:");
        println!("  Home: {}", self.home_dir.display());
        println!("  Config: {}", self.config_dir.display());
        println!("  Data: {}", self.data_dir.display());
        println!("  Cache: {}", self.cache_dir.display());
        println!("  Temp: {}", self.temp_dir.display());
        println!("  Logs: {}", self.log_dir.display());
        
        Ok(())
    }
}

// =========================================
// CROSS-PLATFORM PATH OPERATIONS
// =========================================

pub fn path_operations() {
    println!("=== PATH OPERATIONS ===");
    
    // Path joining
    let base = Path::new("/home/user");
    let file = base.join("documents").join("file.txt");
    println!("Joined path: {}", file.display());
    
    // Platform-specific path separators
    let path_str = if cfg!(target_os = "windows") {
        "C:\\Users\\User\\Documents\\file.txt"
    } else {
        "/home/user/documents/file.txt"
    };
    
    let path = Path::new(path_str);
    println!("Platform-specific path: {}", path.display());
    
    // Path components
    println!("Parent: {:?}", path.parent());
    println!("File name: {:?}", path.file_name());
    println!("File stem: {:?}", path.file_stem());
    println!("Extension: {:?}", path.extension());
    
    // Path properties
    println!("Is absolute: {}", path.is_absolute());
    println!("Is relative: {}", path.is_relative());
    
    // Path normalization
    let messy_path = Path::new("/home//user/../user/./documents/");
    let normalized = messy_path.components().collect::<PathBuf>();
    println!("Normalized path: {}", normalized.display());
    
    // Cross-platform path creation
    let cross_platform_path = Path::new("config").join("app.conf");
    println!("Cross-platform path: {}", cross_platform_path.display());
}

// =========================================
// CROSS-PLATFORM FILE OPERATIONS
// =========================================

pub fn cross_platform_file_operations() -> std::io::Result<()> {
    println!("=== CROSS-PLATFORM FILE OPERATIONS ===");
    
    let paths = PlatformPaths::new();
    paths.initialize()?;
    
    // Write configuration file
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
    println!("Config file written: {}", config_file.display());
    
    // Read configuration file
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
    
    // Create directory with platform-specific handling
    let data_dir = paths.data_dir.join("user_data");
    std::fs::create_dir_all(&data_dir)?;
    println!("Created data directory: {}", data_dir.display());
    
    // Write data file
    let data_file = data_dir.join("data.json");
    let data_content = r#"{"user": "Alice", "score": 100}"#;
    std::fs::write(&data_file, data_content)?;
    println!("Data file written: {}", data_file.display());
    
    // List directory contents
    println!("Config directory contents:");
    for entry in std::fs::read_dir(&paths.config_dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("  {}", path.file_name().unwrap().to_string_lossy());
    }
    
    Ok(())
}

// =========================================
// PLATFORM-SPECIFIC FEATURES
// =========================================

pub fn platform_specific_features() {
    println!("=== PLATFORM-SPECIFIC FEATURES ===");
    
    // Windows features
    #[cfg(target_os = "windows")]
    {
        println!("Windows-specific features:");
        println!("- Registry access via winreg crate");
        println!("- Windows API via winapi crate");
        println!("- COM interfaces");
        println!("- Windows services");
        println!("- Windows Installer (MSI)");
    }
    
    // macOS features
    #[cfg(target_os = "macos")]
    {
        println!("macOS-specific features:");
        println!("- Core Foundation APIs");
        println!("- Cocoa frameworks");
        println!("- AppKit for GUI");
        println!("- Launch Services");
        println!("- Keychain for secure storage");
    }
    
    // Linux features
    #[cfg(target_os = "linux")]
    {
        println!("Linux-specific features:");
        println!("- X11/Wayland display servers");
        println!("- GTK/Qt GUI toolkits");
        println!("- SystemD services");
        println!("- D-Bus IPC");
        println!("/proc filesystem access");
    }
    
    // Unix features
    #[cfg(unix)]
    {
        println!("Unix-specific features:");
        println!("- POSIX APIs");
        println!("- File permissions");
        println!("- Signals");
        println!("- Pipes and sockets");
        println!("- Process management");
    }
    
    // Common features
    println!("Common cross-platform features:");
    println!("- Standard library I/O");
    println!("- Networking APIs");
    println!("- Threading");
    println!("- File system operations");
    println!("- Environment variables");
}

// =========================================
// CONDITIONAL COMPILATION WITH FEATURES
// =========================================

// Feature-based conditional compilation
#[cfg(feature = "windows")]
pub fn windows_feature_enabled() {
    println!("Windows feature enabled");
}

#[cfg(feature = "gui")]
pub fn gui_feature_enabled() {
    println!("GUI feature enabled");
}

#[cfg(feature = "cli")]
pub fn cli_feature_enabled() {
    println!("CLI feature enabled");
}

#[cfg(not(any(feature = "windows", feature = "gui")))]
pub fn default_features() {
    println!("Default features enabled");
}

// =========================================
// CROSS-PLATFORM ERROR HANDLING
// =========================================

#[derive(Debug)]
pub enum CrossPlatformError {
    IoError(std::io::Error),
    PlatformError(String),
    UnsupportedOperation(String),
}

impl std::fmt::Display for CrossPlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CrossPlatformError::IoError(e) => write!(f, "IO Error: {}", e),
            CrossPlatformError::PlatformError(msg) => write!(f, "Platform Error: {}", msg),
            CrossPlatformError::UnsupportedOperation(op) => write!(f, "Unsupported Operation: {}", op),
        }
    }
}

impl std::error::Error for CrossPlatformError {}

impl From<std::io::Error> for CrossPlatformError {
    fn from(error: std::io::Error) -> Self {
        CrossPlatformError::IoError(error)
    }
}

// Cross-platform file operations with error handling
pub fn safe_file_operations() -> Result<(), CrossPlatformError> {
    println!("=== SAFE FILE OPERATIONS ===");
    
    let paths = PlatformPaths::new();
    
    // Safe file reading
    let config_file = paths.config_dir.join("app.conf");
    match std::fs::read_to_string(&config_file) {
        Ok(content) => println!("Config file content: {}", content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("Config file not found, using defaults");
        }
        Err(e) => return Err(CrossPlatformError::IoError(e)),
    }
    
    // Platform-specific operations
    #[cfg(target_os = "windows")]
    {
        println!("Windows-specific operation");
        // Windows-specific code here
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        println!("Unix-specific operation");
        // Unix-specific code here
    }
    
    Ok(())
}

// =========================================
// BUILD CONFIGURATION EXAMPLES
// =========================================

pub fn build_configuration() {
    println!("=== BUILD CONFIGURATION ===");
    
    // Compile-time information
    println!("Target OS: {}", env::consts::OS);
    println!("Target Arch: {}", env::consts::ARCH);
    println!("Target Family: {}", env::consts::FAMILY);
    println!("Target Endian: {}", if cfg!(target_endian = "little") { "little" } else { "big" });
    println!("Target Pointer Width: {}", env::consts::WIDTH);
    
    // Feature flags
    println!("Debug build: {}", cfg!(debug_assertions));
    println!("Release build: {}", !cfg!(debug_assertions));
    
    // SIMD features
    println!("AVX2 available: {}", cfg!(target_feature = "avx2"));
    println!("SSE4.1 available: {}", cfg!(target_feature = "sse4.1"));
    println!("NEON available: {}", cfg!(target_feature = "neon"));
    
    // Platform-specific optimizations
    #[cfg(target_os = "windows")]
    {
        println!("Windows optimizations enabled");
    }
    
    #[cfg(target_os = "macos")]
    {
        println!("macOS optimizations enabled");
    }
    
    #[cfg(target_os = "linux")]
    {
        println!("Linux optimizations enabled");
    }
}

// =========================================
// CROSS-PLATFORM TESTING
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_platform_paths() {
        let paths = PlatformPaths::new();
        
        // Test that all paths are not empty
        assert!(!paths.config_dir.as_os_str().is_empty());
        assert!(!paths.data_dir.as_os_str().is_empty());
        assert!(!paths.cache_dir.as_os_str().is_empty());
        assert!(!paths.temp_dir.as_os_str().is_empty());
        assert!(!paths.log_dir.as_os_str().is_empty());
        assert!(!paths.home_dir.as_os_str().is_empty());
    }
    
    #[test]
    fn test_path_operations() {
        let base = Path::new("/home/user");
        let file = base.join("documents").join("file.txt");
        
        assert_eq!(file.to_string_lossy(), "/home/user/documents/file.txt");
    }
    
    #[test]
    fn test_error_handling() {
        let error = CrossPlatformError::PlatformError("Test error".to_string());
        assert!(error.to_string().contains("Platform Error"));
    }
    
    #[test]
    fn test_platform_detection() {
        let os = env::consts::OS;
        
        // Should be one of the known platforms
        assert!(matches!(os, "windows" | "macos" | "linux" | "android" | "ios"));
    }
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CROSS-PLATFORM DEVELOPMENT DEMONSTRATIONS ===\n");
    
    detect_platform();
    println!();
    
    runtime_platform_detection();
    println!();
    
    path_operations();
    println!();
    
    cross_platform_file_operations()?;
    println!();
    
    platform_specific_features();
    println!();
    
    safe_file_operations()?;
    println!();
    
    build_configuration();
    println!();
    
    // Call platform-specific functions
    windows_specific();
    macos_specific();
    linux_specific();
    desktop_platforms();
    non_windows();
    x86_64_code();
    aarch64_code();
    sixty_four_bit();
    thirty_two_bit();
    
    // Call feature-specific functions
    windows_feature_enabled();
    gui_feature_enabled();
    cli_feature_enabled();
    default_features();
    
    println!("\n=== CROSS-PLATFORM DEVELOPMENT DEMONSTRATIONS COMPLETE ===");
    println!("Key takeaways:");
    println!("- Use cfg attributes for conditional compilation");
    println!("- Detect platform at runtime when needed");
    println!("- Abstract platform differences in interfaces");
    println!("- Use standard library for cross-platform code");
    println!("- Handle platform-specific errors gracefully");
    println!("- Test on all target platforms");
    println!("- Use feature flags for optional functionality");
    
    Ok(())
}
