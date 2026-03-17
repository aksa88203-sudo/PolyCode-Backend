// serialization.rs
// Comprehensive examples of serialization in Rust

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// =========================================
// CORE SERIALIZATION STRUCTURES
// =========================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub country: String,
    pub postal_code: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub skills: Vec<String>,
    pub address: Address,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Status {
    Active,
    Inactive,
    Pending { reason: String },
    Suspended { until: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub database_url: String,
    pub max_connections: u32,
    #[serde(default)]
    pub debug: bool,
    #[serde(rename = "log-level")]
    pub log_level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
}

// =========================================
// JSON SERIALIZATION EXAMPLES
// =========================================

pub fn basic_json_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BASIC JSON SERIALIZATION ===");
    
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "JavaScript".to_string(), "Python".to_string()],
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
            postal_code: "10001".to_string(),
        },
        metadata: {
            let mut map = HashMap::new();
            map.insert("department".to_string(), "Engineering".to_string());
            map.insert("level".to_string(), "Senior".to_string());
            map
        },
    };
    
    // Serialize to JSON string
    let json_string = serde_json::to_string(&person)?;
    println!("JSON string: {}", json_string);
    
    // Serialize to pretty JSON
    let pretty_json = serde_json::to_string_pretty(&person)?;
    println!("Pretty JSON:\n{}", pretty_json);
    
    // Deserialize from JSON string
    let deserialized: Person = serde_json::from_str(&json_string)?;
    println!("Deserialized: {:?}", deserialized);
    
    // Verify round-trip
    assert_eq!(person.name, deserialized.name);
    assert_eq!(person.age, deserialized.age);
    
    Ok(())
}

pub fn dynamic_json_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DYNAMIC JSON HANDLING ===");
    
    let json_str = r#"
    {
        "users": [
            {"id": 1, "name": "Alice", "active": true, "role": "admin"},
            {"id": 2, "name": "Bob", "active": false, "role": "user"},
            {"id": 3, "name": "Charlie", "active": true, "role": "user"}
        ],
        "metadata": {
            "total": 3,
            "page": 1,
            "filters": ["active", "recent"],
            "last_updated": "2023-12-01T10:00:00Z"
        }
    }
    "#;
    
    let data: serde_json::Value = serde_json::from_str(json_str)?;
    
    // Access nested values
    if let Some(users) = data.get("users") {
        if let Some(users_array) = users.as_array() {
            println!("Users:");
            for user in users_array {
                if let Some(name) = user.get("name") {
                    if let Some(active) = user.get("active") {
                        if let Some(role) = user.get("role") {
                            println!("  {} - Active: {}, Role: {}", 
                                     name, active, role);
                        }
                    }
                }
            }
        }
    }
    
    // Extract specific fields
    let total_users = data["metadata"]["total"].as_u64().unwrap_or(0);
    let last_updated = data["metadata"]["last_updated"].as_str().unwrap_or("Unknown");
    
    println!("Total users: {}", total_users);
    println!("Last updated: {}", last_updated);
    
    // Convert to typed struct
    #[derive(Debug, serde::Deserialize)]
    struct ApiResponse {
        users: Vec<UserInfo>,
        metadata: Metadata,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct User {
        id: u32,
        name: String,
        active: bool,
        role: String,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct User {
        id: u32,
        name: String,
        active: bool,
        role: String,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct Metadata {
        total: u32,
        page: u32,
        filters: Vec<String>,
        last_updated: String,
    }
    
    let api_response: ApiResponse = serde_json::from_str(json_str)?;
    println!("Typed response: {:?}", api_response);
    
    // Modify JSON values
    let mut modified_data = data.clone();
    if let Some(metadata) = modified_data.get_mut("metadata") {
        if let Some(total) = metadata.get_mut("total") {
            *total = serde_json::Value::Number(serde_json::Number::from(5));
        }
    }
    
    println!("Modified JSON: {}", serde_json::to_string_pretty(&modified_data)?);
    
    Ok(())
}

pub fn json_builder_patterns() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON BUILDER PATTERNS ===");
    
    // Create JSON object using json! macro
    let user_json = serde_json::json!({
        "id": 123,
        "name": "John Doe",
        "email": "john@example.com",
        "active": true,
        "roles": ["user", "admin"],
        "profile": {
            "age": 30,
            "city": "New York",
            "interests": ["programming", "music", "travel"]
        }
    });
    
    println!("Built JSON: {}", serde_json::to_string_pretty(&user_json)?);
    
    // Build JSON incrementally
    let mut builder = serde_json::Map::new();
    builder.insert("timestamp".to_string(), 
                  serde_json::Value::Number(serde_json::Number::from(1640995200)));
    builder.insert("event".to_string(), 
                  serde_json::Value::String("user_login".to_string()));
    builder.insert("user_id".to_string(), 
                  serde_json::Value::Number(serde_json::Number::from(456)));
    
    let event_json = serde_json::Value::Object(builder);
    println!("Built event JSON: {}", serde_json::to_string_pretty(&event_json)?);
    
    // Merge JSON objects
    let base_data = serde_json::json!({
        "name": "Alice",
        "age": 25,
        "city": "New York"
    });
    
    let update_data = serde_json::json!({
        "age": 26,
        "email": "alice@example.com",
        "active": true
    });
    
    let merged = merge_json_objects(base_data, update_data)?;
    println!("Merged JSON: {}", serde_json::to_string_pretty(&merged)?);
    
    Ok(())
}

fn merge_json_objects(a: serde_json::Value, b: serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    if let (serde_json::Value::Object(mut a_obj), serde_json::Value::Object(b_obj)) = (a, b) {
        for (key, value) in b_obj {
            a_obj.insert(key, value);
        }
        Ok(serde_json::Value::Object(a_obj))
    } else {
        Err("Both values must be objects".into())
    }
}

// =========================================
// YAML SERIALIZATION EXAMPLES
// =========================================

pub fn yaml_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== YAML SERIALIZATION ===");
    
    let config = Config {
        database_url: "postgresql://localhost/mydb".to_string(),
        max_connections: 10,
        debug: true,
        log_level: "info".to_string(),
        optional_field: None,
    };
    
    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&config)?;
    println!("YAML config:\n{}", yaml_string);
    
    // Deserialize from YAML
    let yaml_content = r#"
database: "postgresql://localhost/mydb"
max_connections: 10
debug: true
log-level: "info"
    "#;
    
    let loaded_config: Config = serde_yaml::from_str(yaml_content)?;
    println!("Loaded config: {:?}", loaded_config);
    
    // Complex YAML structure
    #[derive(Debug, serde::Deserialize)]
    struct AppConfig {
        application: ApplicationConfig,
        database: DatabaseConfig,
        features: FeaturesConfig,
        servers: Vec<ServerConfig>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct ApplicationConfig {
        name: String,
        version: String,
        authors: Vec<String>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct DatabaseConfig {
        url: String,
        max_connections: u32,
        timeout: u32,
        pool: PoolConfig,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct PoolConfig {
        min_connections: u32,
        max_connections: u32,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct FeaturesConfig {
        auth: bool,
        logging: bool,
        cache: CacheConfig,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct CacheConfig {
        enabled: bool,
        ttl: u32,
        provider: String,
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct ServerConfig {
        name: String,
        host: String,
        port: u16,
        ssl: bool,
    }
    
    let complex_yaml = r#"
application:
  name: "my-web-app"
  version: "1.0.0"
  authors:
    - "Alice <alice@example.com>"
    - "Bob <bob@example.com>"

database:
  url: "postgresql://localhost/myapp"
  max_connections: 20
  timeout: 30
  pool:
    min_connections: 5
    max_connections: 15

features:
  auth: true
  logging: true
  cache:
    enabled: true
    ttl: 3600
    provider: "redis"

servers:
  - name: "web-server"
    host: "0.0.0.0"
    port: 8080
    ssl: true
  - name: "api-server"
    host: "0.0.0.0"
    port: 3000
    ssl: false
    "#;
    
    let app_config: AppConfig = serde_yaml::from_str(complex_yaml)?;
    println!("Complex app config: {:?}", app_config);
    
    Ok(())
}

// =========================================
// TOML SERIALIZATION EXAMPLES
// =========================================

pub fn toml_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TOML SERIALIZATION ===");
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct CargoConfig {
        package: PackageConfig,
        dependencies: DependenciesConfig,
        [[bin]]
        binaries: Vec<BinaryConfig>,
        dev_dependencies: DevDependenciesConfig,
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct PackageConfig {
        name: String,
        version: String,
        authors: Vec<String>,
        edition: String,
        description: Option<String>,
        license: String,
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct DependenciesConfig {
        serde: String,
        tokio: String,
        serde_json: String,
        #[serde(default)]
        tracing: Option<String>,
        #[serde(default)]
        clap: Option<String>,
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct DevDependenciesConfig {
        tokio-test: String,
        criterion: String,
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct BinaryConfig {
        name: String,
        path: String,
        #[serde(default)]
        required-features: Vec<String>,
    }
    
    let cargo_config = CargoConfig {
        package: PackageConfig {
            name: "my-awesome-app".to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["Alice <alice@example.com>".to_string()],
            edition: "2021".to_string(),
            description: Some("An awesome Rust application".to_string()),
            license: "MIT".to_string(),
        },
        dependencies: DependenciesConfig {
            serde: "1.0".to_string(),
            tokio: "1.0".to_string(),
            serde_json: "1.0".to_string(),
            tracing: Some("0.1".to_string()),
            clap: Some("4.0".to_string()),
        },
        binaries: vec![
            BinaryConfig {
                name: "my-awesome-app".to_string(),
                path: "src/main.rs".to_string(),
                required_features: vec!["full".to_string()],
            },
        ],
        dev_dependencies: DevDependenciesConfig {
            tokio_test: "0.4".to_string(),
            criterion: "0.5".to_string(),
        },
    };
    
    // Serialize to TOML
    let toml_string = toml::to_string_pretty(&cargo_config)?;
    println!("TOML config:\n{}", toml_string);
    
    // Parse existing TOML
    let existing_toml = r#"
[package]
name = "existing-app"
version = "0.2.0"
authors = ["Bob <bob@example.com>"]
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"

[dev-dependencies]
tokio-test = "0.4"
    "#;
    
    let parsed_config: CargoConfig = toml::from_str(existing_toml)?;
    println!("Parsed TOML config: {:?}", parsed_config);
    
    Ok(())
}

// =========================================
// BINARY SERIALIZATION EXAMPLES
// =========================================

pub fn binary_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BINARY SERIALIZATION ===");
    
    #[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
    struct Message {
        id: u64,
        timestamp: u64,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
    }
    
    let message = Message {
        id: 12345,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        metadata: {
            let mut map = HashMap::new();
            map.insert("type".to_string(), "data".to_string());
            map.insert("priority".to_string(), "high".to_string());
            map
        },
    };
    
    // Serialize to binary
    let binary_data: Vec<u8> = bincode::serialize(&message)?;
    println!("Binary data length: {} bytes", binary_data.len());
    println!("Binary data: {:?}", binary_data);
    
    // Deserialize from binary
    let deserialized: Message = bincode::deserialize(&binary_data)?;
    println!("Deserialized: {:?}", deserialized);
    
    // Verify round-trip
    assert_eq!(message, deserialized);
    
    // Size comparison
    let json_size = serde_json::to_string(&message)?.len();
    let binary_size = binary_data.len();
    
    println!("Size comparison:");
    println!("  JSON: {} bytes", json_size);
    println!("  Binary: {} bytes", binary_size);
    println!("  Compression ratio: {:.2}%", 
             (binary_size as f64 / json_size as f64) * 100.0);
    
    // Performance benchmark
    benchmark_serialization_performance(message)?;
    
    Ok(())
}

fn benchmark_serialization_performance(message: Message) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SERIALIZATION PERFORMANCE BENCHMARK ===");
    
    let iterations = 10000;
    
    // Benchmark JSON serialization
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _json = serde_json::to_string(&message)?;
    }
    let json_duration = start.elapsed();
    
    // Benchmark JSON deserialization
    let json_string = serde_json::to_string(&message)?;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _: Message = serde_json::from_str(&json_string)?;
    }
    let json_deserialize_duration = start.elapsed();
    
    // Benchmark binary serialization
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _binary = bincode::serialize(&message)?;
    }
    let binary_duration = start.elapsed();
    
    // Benchmark binary deserialization
    let binary_data = bincode::serialize(&message)?;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _: Message = bincode::deserialize(&binary_data)?;
    }
    let binary_deserialize_duration = start.elapsed();
    
    println!("Performance ({} iterations):", iterations);
    println!("  JSON serialize: {:?} ({:.2} μs/op)", 
             json_duration, json_duration.as_micros() as f64 / iterations as f64);
    println!("  JSON deserialize: {:?} ({:.2} μs/op)", 
             json_deserialize_duration, json_deserialize_duration.as_micros() as f64 / iterations as f64);
    println!("  Binary serialize: {:?} ({:.2} μs/op)", 
             binary_duration, binary_duration.as_micros() as f64 / iterations as f64);
    println!("  Binary deserialize: {:?} ({:.2} μs/op)", 
             binary_deserialize_duration, binary_deserialize_duration.as_micros() as f64 / iterations as f64);
    
    Ok(())
}

// =========================================
// CUSTOM SERIALIZATION EXAMPLES
// =========================================

pub fn custom_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== CUSTOM SERIALIZATION ===");
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Record {
        id: u32,
        name: String,
        value: f64,
        #[serde(serialize_with = "serialize_vec")]
        #[serde(deserialize_with = "deserialize_vec")]
        data: Vec<u8>,
    }
    
    let record = Record {
        id: 1,
        name: "Test Record".to_string(),
        value: 3.14159,
        data: vec![1, 2, 3, 4, 5],
    };
    
    // Custom serialize to pipe-delimited format
    let custom_string = serialize_to_pipe_format(&record)?;
    println!("Custom pipe format: {}", custom_string);
    
    // Custom deserialize from pipe-delimited format
    let deserialized: Record = deserialize_from_pipe_format(&custom_string)?;
    println!("Deserialized: {:?}", deserialized);
    
    // Verify round-trip
    assert_eq!(record.id, deserialized.id);
    assert_eq!(record.name, deserialized.name);
    
    Ok(())
}

fn serialize_to_pipe_format<T: serde::Serialize>(data: &T) -> Result<String, Box<dyn std::error::Error>> {
    // Convert to JSON first, then apply custom formatting
    let json = serde_json::to_string(data)?;
    
    // Simple pipe-delimited format
    let custom = json
        .replace('{', "")
        .replace('}', "")
        .replace(':', '|')
        .replace(',', ';');
    
    Ok(custom)
}

fn deserialize_from_pipe_format<'de, T: serde::Deserialize<'de>>(s: &'de str) -> Result<T, Box<dyn std::error::Error>> {
    // Convert custom format back to JSON
    let json = s
        .replace('|', ':')
        .replace(';', ',');
    
    let json_with_braces = format!("{{{}}}", json);
    
    serde_json::from_str(&json_with_braces).map_err(Into::into)
}

// =========================================
// ENUM SERIALIZATION EXAMPLES
// =========================================

pub fn enum_serialization() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ENUM SERIALIZATION ===");
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(tag = "type")] // Externally tagged
    enum Message {
        #[serde(rename = "text_message")]
        Text { content: String, timestamp: u64 },
        
        #[serde(rename = "image_message")]
        Image { url: String, alt_text: String, width: u32, height: u32 },
        
        #[serde(rename = "file_message")]
        File { filename: String, size: u64, mime_type: String },
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    #[serde(untagged)] // Internally tagged
    enum ApiResponse {
        Success { data: serde_json::Value },
        Error { error: String, code: u32, details: Option<String> },
    }
    
    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    enum Status {
        #[serde(rename = "active")]
        Active,
        #[serde(rename = "inactive")]
        Inactive,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "suspended")]
        Suspended,
    }
    
    let messages = vec![
        Message::Text {
            content: "Hello, world!".to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
        Message::Image {
            url: "https://example.com/image.jpg".to_string(),
            alt_text: "Example image".to_string(),
            width: 800,
            height: 600,
        },
        Message::File {
            filename: "document.pdf".to_string(),
            size: 1024000,
            mime_type: "application/pdf".to_string(),
        },
    ];
    
    // Serialize tagged enum
    let json = serde_json::to_string(&messages)?;
    println!("Tagged enum JSON: {}", serde_json::to_string_pretty(&json)?);
    
    // Untagged enum examples
    let success_response = ApiResponse::Success {
        data: serde_json::json!({
            "users": ["Alice", "Bob", "Charlie"],
            "total": 3
        }),
    };
    
    let error_response = ApiResponse::Error {
        error: "Invalid credentials".to_string(),
        code: 401,
        details: Some("Username or password is incorrect".to_string()),
    };
    
    println!("Success response: {}", serde_json::to_string_pretty(&success_response)?);
    println!("Error response: {}", serde_json::to_string_pretty(&error_response)?);
    
    // Status enum
    let statuses = vec![Status::Active, Status::Inactive, Status::Pending, Status::Suspended];
    let status_json = serde_json::to_string(&statuses)?;
    println!("Status enum: {}", serde_json::to_string_pretty(&status_json)?);
    
    Ok(())
}

// =========================================
// ERROR HANDLING EXAMPLES
// =========================================

pub fn serialization_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SERIALIZATION ERROR HANDLING ===");
    
    use thiserror::Error;
    
    #[derive(Error, Debug)]
    pub enum SerializationError {
        #[error("JSON serialization failed: {0}")]
        JsonSerialization(String),
        
        #[error("Binary serialization failed: {0}")]
        BinarySerialization(String),
        
        #[error("YAML serialization failed: {0}")]
        YamlSerialization(String),
        
        #[error("TOML serialization failed: {0}")]
        TomlSerialization(String),
        
        #[error("Invalid data format: {0}")]
        InvalidFormat(String),
        
        #[error("IO error during serialization: {0}")]
        IoError(#[from] std::io::Error),
    }
    
    pub trait Serializable {
        fn serialize_to_json(&self) -> Result<String, SerializationError>;
        fn serialize_to_binary(&self) -> Result<Vec<u8>, SerializationError>;
        fn serialize_to_yaml(&self) -> Result<String, SerializationError>;
    }
    
    #[derive(Debug)]
    pub struct DataPacket {
        pub id: u32,
        pub payload: Vec<u8>,
        pub checksum: u32,
    }
    
    impl Serializable for DataPacket {
        fn serialize_to_json(&self) -> Result<String, SerializationError> {
            serde_json::to_string(self)
                .map_err(|e| SerializationError::JsonSerialization(e.to_string()))
        }
        
        fn serialize_to_binary(&self) -> Result<Vec<u8>, SerializationError> {
            bincode::serialize(self)
                .map_err(|e| SerializationError::BinarySerialization(e.to_string()))
        }
        
        fn serialize_to_yaml(&self) -> Result<String, SerializationError> {
            serde_yaml::to_string(self)
                .map_err(|e| SerializationError::YamlSerialization(e.to_string()))
        }
    }
    
    let packet = DataPacket {
        id: 42,
        payload: vec![1, 2, 3, 4, 5],
        checksum: 12345,
    };
    
    // Test error handling
    match packet.serialize_to_json() {
        Ok(json) => println!("JSON serialization successful: {} bytes", json.len()),
        Err(e) => eprintln!("JSON serialization failed: {}", e),
    }
    
    match packet.serialize_to_binary() {
        Ok(binary) => println!("Binary serialization successful: {} bytes", binary.len()),
        Err(e) => eprintln!("Binary serialization failed: {}", e),
    }
    
    // Test with invalid data
    let invalid_json = r#"
    {
        "id": 123,
        "name": "Test",
        "invalid_field": unclosed_string
    "#;
    
    match serde_json::from_str::<serde_json::Value>(invalid_json) {
        Ok(_) => println!("Unexpectedly parsed invalid JSON"),
        Err(e) => println!("Expected error parsing invalid JSON: {}", e),
    }
    
    Ok(())
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SERIALIZATION DEMONSTRATIONS ===\n");
    
    basic_json_serialization()?;
    println!();
    
    dynamic_json_handling()?;
    println!();
    
    json_builder_patterns()?;
    println!();
    
    yaml_serialization()?;
    println!();
    
    toml_serialization()?;
    println!();
    
    binary_serialization()?;
    println!();
    
    custom_serialization()?;
    println!();
    
    enum_serialization()?;
    println!();
    
    serialization_error_handling()?;
    
    println!("\n=== SERIALIZATION DEMONSTRATIONS COMPLETE ===");
    println!("Key takeaways:");
    println!("- serde provides unified serialization framework");
    println!("- JSON is widely supported with serde_json");
    println!("- YAML/TOML are excellent for configuration");
    println!("- Binary formats offer better performance");
    println!("- Attributes provide fine-grained control");
    println!("- Error handling should be comprehensive");
    
    Ok(())
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_round_trip() {
        let person = Person {
            name: "Test".to_string(),
            age: 25,
            skills: vec!["Rust".to_string()],
            address: Address {
                street: "123 Test St".to_string(),
                city: "Test City".to_string(),
                country: "Test Country".to_string(),
                postal_code: "12345".to_string(),
            },
            metadata: HashMap::new(),
        };
        
        let json = serde_json::to_string(&person).unwrap();
        let deserialized: Person = serde_json::from_str(&json).unwrap();
        
        assert_eq!(person.name, deserialized.name);
        assert_eq!(person.age, deserialized.age);
    }
    
    #[test]
    fn test_binary_round_trip() {
        let message = Message {
            id: 12345,
            timestamp: 1640995200,
            data: vec![1, 2, 3, 4, 5],
            metadata: HashMap::new(),
        };
        
        let binary = bincode::serialize(&message).unwrap();
        let deserialized: Message = bincode::deserialize(&binary).unwrap();
        
        assert_eq!(message, deserialized);
    }
    
    #[test]
    fn test_enum_serialization() {
        let message = Message::Text {
            content: "Hello".to_string(),
            timestamp: 1640995200,
        };
        
        let json = serde_json::to_string(&message).unwrap();
        let deserialized: Message = serde_json::from_str(&json).unwrap();
        
        match (message, deserialized) {
            (Message::Text { content: c1, timestamp: t1 }, 
             Message::Text { content: c2, timestamp: t2 }) => {
                assert_eq!(c1, c2);
                assert_eq!(t1, t2);
            }
            _ => panic!("Enum variant mismatch"),
        }
    }
    
    #[test]
    fn test_custom_format() {
        let record = Record {
            id: 1,
            name: "Test".to_string(),
            value: 3.14,
            data: vec![1, 2, 3],
        };
        
        let custom = serialize_to_pipe_format(&record).unwrap();
        let deserialized: Record = deserialize_from_pipe_format(&custom).unwrap();
        
        assert_eq!(record.id, deserialized.id);
        assert_eq!(record.name, deserialized.name);
    }
    
    #[test]
    fn test_error_handling() {
        let invalid_json = r#"{ "invalid": json }"#;
        
        let result: Result<serde_json::Value, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }
}
