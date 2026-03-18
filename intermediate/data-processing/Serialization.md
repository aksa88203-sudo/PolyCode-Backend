# Serialization in Rust

## Overview

Serialization is the process of converting data structures into a format that can be stored or transmitted, and then reconstructed later. Rust provides excellent support for serialization through the `serde` ecosystem and various format-specific libraries.

---

## Core Serialization Concepts

### Serde Ecosystem

| Crate | Purpose | Formats Supported |
|-------|---------|-------------------|
| `serde` | Core serialization framework | All serde-compatible formats |
| `serde_json` | JSON serialization/deserialization | JSON |
| `serde_yaml` | YAML serialization/deserialization | YAML |
| `toml` | TOML serialization/deserialization | TOML |
| `bincode` | Binary serialization | Compact binary format |
| `rmp-serde` | MessagePack serialization | MessagePack |
| `serde_cbor` | CBOR serialization | CBOR |
| `serde-pickle` | Python pickle format | Pickle |

### Derive Macros

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    active: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Active,
    Inactive,
    Pending { reason: String },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database_url: String,
    max_connections: u32,
    #[serde(default)]
    debug: bool,
    #[serde(rename = "log-level")]
    log_level: String,
}
```

---

## JSON Serialization

### Basic JSON Operations

```rust
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    skills: Vec<String>,
    address: Address,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    country: String,
}

fn json_serialization_examples() -> Result<(), Box<dyn std::error::Error>> {
    // Create a person
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        skills: vec!["Rust".to_string(), "JavaScript".to_string()],
        address: Address {
            street: "123 Main St".to_string(),
            city: "New York".to_string(),
            country: "USA".to_string(),
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
    
    // Create JSON object directly
    let json_obj = json!({
        "name": "Bob",
        "age": 25,
        "skills": ["Python", "Go"],
        "address": {
            "street": "456 Oak Ave",
            "city": "San Francisco",
            "country": "USA"
        }
    });
    
    println!("JSON object: {}", json_obj);
    
    // Access JSON values
    if let Some(name) = json_obj.get("name") {
        println!("Name: {}", name);
    }
    
    // Convert to struct
    let person_from_json: Person = serde_json::from_value(json_obj)?;
    println!("Person from JSON: {:?}", person_from_json);
    
    Ok(())
}
```

### Advanced JSON Handling

```rust
use serde_json::{Map, Value};
use std::collections::HashMap;

fn advanced_json_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Working with dynamic JSON
    let json_str = r#"
    {
        "users": [
            {"id": 1, "name": "Alice", "active": true},
            {"id": 2, "name": "Bob", "active": false}
        ],
        "metadata": {
            "total": 2,
            "page": 1,
            "filters": ["active", "recent"]
        }
    }
    "#;
    
    let data: Value = serde_json::from_str(json_str)?;
    
    // Access nested values
    if let Some(users) = data.get("users") {
        if let Some(users_array) = users.as_array() {
            for user in users_array {
                if let Some(name) = user.get("name") {
                    println!("User: {}", name);
                }
            }
        }
    }
    
    // Extract specific fields
    let total_users = data["metadata"]["total"].as_u64().unwrap_or(0);
    println!("Total users: {}", total_users);
    
    // Convert to typed struct
    #[derive(Debug, Deserialize)]
    struct ApiResponse {
        users: Vec<UserInfo>,
        metadata: Metadata,
    }
    
    #[derive(Debug, Deserialize)]
    struct UserInfo {
        id: u32,
        name: String,
        active: bool,
    }
    
    #[derive(Debug, Deserialize)]
    struct Metadata {
        total: u32,
        page: u32,
        filters: Vec<String>,
    }
    
    let api_response: ApiResponse = serde_json::from_str(json_str)?;
    println!("Typed response: {:?}", api_response);
    
    // Modify JSON values
    let mut modified_data = data;
    if let Some(metadata) = modified_data.get_mut("metadata") {
        if let Some(total) = metadata.get_mut("total") {
            *total = Value::Number(serde_json::Number::from(5));
        }
    }
    
    println!("Modified JSON: {}", serde_json::to_string_pretty(&modified_data)?);
    
    Ok(())
}
```

---

## YAML Serialization

### YAML with serde_yaml

```rust
use serde::{Deserialize, Serialize};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
    features: FeaturesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
    timeout: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    ssl: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct FeaturesConfig {
    auth: bool,
    logging: bool,
    cache: bool,
}

fn yaml_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        database: DatabaseConfig {
            url: "postgresql://localhost/mydb".to_string(),
            max_connections: 10,
            timeout: 30,
        },
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            ssl: true,
        },
        features: FeaturesConfig {
            auth: true,
            logging: true,
            cache: false,
        },
    };
    
    // Serialize to YAML
    let yaml_string = serde_yaml::to_string(&config)?;
    println!("YAML config:\n{}", yaml_string);
    
    // Deserialize from YAML
    let yaml_content = r#"
database:
  url: "postgresql://localhost/mydb"
  max_connections: 10
  timeout: 30
server:
  host: "0.0.0.0"
  port: 8080
  ssl: true
features:
  auth: true
  logging: true
  cache: false
    "#;
    
    let loaded_config: Config = serde_yaml::from_str(yaml_content)?;
    println!("Loaded config: {:?}", loaded_config);
    
    Ok(())
}
```

---

## TOML Serialization

### Configuration with TOML

```rust
use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    package: PackageConfig,
    dependencies: DependenciesConfig,
    [[bin]]
    binaries: Vec<BinaryConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageConfig {
    name: String,
    version: String,
    authors: Vec<String>,
    edition: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DependenciesConfig {
    serde: String,
    tokio: String,
    #[serde(default)]
    tracing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinaryConfig {
    name: String,
    path: String,
}

fn toml_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig {
        package: PackageConfig {
            name: "my-app".to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["Alice <alice@example.com>".to_string()],
            edition: "2021".to_string(),
        },
        dependencies: DependenciesConfig {
            serde: "1.0".to_string(),
            tokio: "1.0".to_string(),
            tracing: Some("0.1".to_string()),
        },
        binaries: vec![
            BinaryConfig {
                name: "my-app".to_string(),
                path: "src/main.rs".to_string(),
            },
        ],
    };
    
    // Serialize to TOML
    let toml_string = toml::to_string_pretty(&config)?;
    println!("TOML config:\n{}", toml_string);
    
    // Deserialize from TOML
    let toml_content = r#"
[package]
name = "my-app"
version = "0.1.0"
authors = ["Alice <alice@example.com>"]
edition = "2021"

[dependencies]
serde = "1.0"
tokio = "1.0"
tracing = "0.1"

[[bin]]
name = "my-app"
path = "src/main.rs"
    "#;
    
    let loaded_config: AppConfig = toml::from_str(toml_content)?;
    println!("Loaded config: {:?}", loaded_config);
    
    Ok(())
}
```

---

## Binary Serialization

### High-Performance Binary with bincode

```rust
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Message {
    id: u64,
    timestamp: u64,
    data: Vec<u8>,
}

fn binary_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let message = Message {
        id: 12345,
        timestamp: 1640995200, // Unix timestamp
        data: vec![1, 2, 3, 4, 5],
    };
    
    // Serialize to binary
    let binary_data: Vec<u8> = bincode::serialize(&message)?;
    println!("Binary data length: {} bytes", binary_data.len());
    
    // Deserialize from binary
    let deserialized: Message = bincode::deserialize(&binary_data)?;
    println!("Deserialized: {:?}", deserialized);
    
    // Verify round-trip
    assert_eq!(message, deserialized);
    
    // Size comparison
    let json_size = serde_json::to_string(&message)?.len();
    let binary_size = binary_data.len();
    
    println!("JSON size: {} bytes", json_size);
    println!("Binary size: {} bytes", binary_size);
    println!("Compression ratio: {:.2}%", (binary_size as f64 / json_size as f64) * 100.0);
    
    Ok(())
}
```

### Custom Serialization Formats

```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use std::fmt;

// Custom format: CSV-like with custom delimiter
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    id: u32,
    name: String,
    value: f64,
}

// Custom serializer
pub fn serialize_custom<T: Serialize>(data: &T) -> Result<String, String> {
    // Convert to JSON first, then apply custom formatting
    let json = serde_json::to_string(data)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    // Remove braces and quotes, replace with custom format
    let custom = json
        .replace('{', "")
        .replace('}', "")
        .replace(':', '|')
        .replace(',', ';');
    
    Ok(custom)
}

// Custom deserializer
pub fn deserialize_custom<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, String> {
    // Convert custom format back to JSON
    let json = s
        .replace('|', ':')
        .replace(';', ',');
    
    let json_with_braces = format!("{{{}}}", json);
    
    serde_json::from_str(&json_with_braces)
        .map_err(|e| format!("Deserialization error: {}", e))
}

fn custom_format_example() -> Result<(), Box<dyn std::error::Error>> {
    let record = Record {
        id: 1,
        name: "Test".to_string(),
        value: 3.14,
    };
    
    // Serialize to custom format
    let custom_string = serialize_custom(&record)?;
    println!("Custom format: {}", custom_string);
    
    // Deserialize from custom format
    let deserialized: Record = deserialize_custom(&custom_string)?;
    println!("Deserialized: {:?}", deserialized);
    
    Ok(())
}
```

---

## Serialization Attributes

### Field-Level Control

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AdvancedStruct {
    // Rename field
    #[serde(rename = "user_id")]
    id: u32,
    
    // Skip serialization
    #[serde(skip)]
    password: String,
    
    // Skip deserialization
    #[serde(skip_deserializing)]
    computed_field: String,
    
    // Default value
    #[serde(default)]
    optional_field: Option<String>,
    
    // Default with function
    #[serde(default = "default_timestamp")]
    timestamp: u64,
    
    // Serialize with custom function
    #[serde(serialize_with = "serialize_vec")]
    #[serde(deserialize_with = "deserialize_vec")]
    data: Vec<u8>,
    
    // Flatten nested struct
    #[serde(flatten)]
    metadata: Metadata,
    
    // Conditional serialization
    #[serde(skip_serializing_if = "is_empty")]
    optional_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    version: String,
    author: String,
}

fn default_timestamp() -> u64 {
    1640995200 // Default timestamp
}

fn serialize_vec<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex_string = hex::encode(data);
    serializer.serialize_str(&hex_string)
}

fn deserialize_vec<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let hex_string: String = String::deserialize(deserializer)?;
    hex::decode(&hex_string)
        .map_err(serde::de::Error::custom)
}

fn is_empty(value: &Option<String>) -> bool {
    value.as_ref().map_or(true, |s| s.is_empty())
}
```

### Enum Serialization

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")] // Externally tagged
enum Message {
    #[serde(rename = "text_message")]
    Text { content: String },
    
    #[serde(rename = "image_message")]
    Image { url: String, alt_text: String },
    
    #[serde(rename = "file_message")]
    File { filename: String, size: u64 },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)] // Internally tagged
enum ApiResponse {
    Success { data: serde_json::Value },
    Error { error: String, code: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
}

fn enum_serialization_examples() -> Result<(), Box<dyn std::error::Error>> {
    let messages = vec![
        Message::Text {
            content: "Hello, world!".to_string(),
        },
        Message::Image {
            url: "https://example.com/image.jpg".to_string(),
            alt_text: "Example image".to_string(),
        },
    ];
    
    // Serialize tagged enum
    let json = serde_json::to_string(&messages)?;
    println!("Tagged enum JSON: {}", json);
    
    // Untagged enum
    let success_response = ApiResponse::Success {
        data: serde_json::json!({"users": ["Alice", "Bob"]}),
    };
    
    let success_json = serde_json::to_string(&success_response)?;
    println!("Untagged enum JSON: {}", success_json);
    
    Ok(())
}
```

---

## Performance Considerations

### Serialization Benchmarks

```rust
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
struct BenchmarkData {
    id: u64,
    name: String,
    description: String,
    tags: Vec<String>,
    metadata: std::collections::HashMap<String, String>,
}

fn benchmark_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let data = BenchmarkData {
        id: 12345,
        name: "Benchmark item".to_string(),
        description: "This is a benchmark item for serialization performance testing".to_string(),
        tags: vec!["test".to_string(), "benchmark".to_string(), "performance".to_string()],
        metadata: {
            let mut map = std::collections::HashMap::new();
            map.insert("category".to_string(), "testing".to_string());
            map.insert("priority".to_string(), "high".to_string());
            map
        },
    };
    
    // Benchmark JSON serialization
    let start = Instant::now();
    for _ in 0..1000 {
        let _json = serde_json::to_string(&data)?;
    }
    let json_duration = start.elapsed();
    
    // Benchmark binary serialization
    let start = Instant::now();
    for _ in 0..1000 {
        let _binary = bincode::serialize(&data)?;
    }
    let binary_duration = start.elapsed();
    
    // Compare sizes
    let json_size = serde_json::to_string(&data)?.len();
    let binary_size = bincode::serialize(&data)?.len();
    
    println!("Serialization Benchmark (1000 iterations):");
    println!("JSON: {:?} ({} bytes)", json_duration, json_size);
    println!("Binary: {:?} ({} bytes)", binary_duration, binary_size);
    println!("Binary is {:.1}% smaller than JSON", 
             (1.0 - binary_size as f64 / json_size as f64) * 100.0);
    
    Ok(())
}
```

---

## Error Handling

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerializationError {
    #[error("JSON serialization failed: {0}")]
    JsonSerialization(String),
    
    #[error("Binary serialization failed: {0}")]
    BinarySerialization(String),
    
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    
    #[error("IO error during serialization: {0}")]
    IoError(#[from] std::io::Error),
}

pub trait Serializable {
    fn serialize_to_json(&self) -> Result<String, SerializationError>;
    fn serialize_to_binary(&self) -> Result<Vec<u8>, SerializationError>;
}

#[derive(Debug)]
pub struct DataPacket {
    pub id: u32,
    pub payload: Vec<u8>,
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
}

fn error_handling_example() -> Result<(), Box<dyn std::error::Error>> {
    let packet = DataPacket {
        id: 42,
        payload: vec![1, 2, 3, 4, 5],
    };
    
    match packet.serialize_to_json() {
        Ok(json) => println!("JSON: {}", json),
        Err(e) => eprintln!("JSON serialization failed: {}", e),
    }
    
    match packet.serialize_to_binary() {
        Ok(binary) => println!("Binary: {} bytes", binary.len()),
        Err(e) => eprintln!("Binary serialization failed: {}", e),
    }
    
    Ok(())
}
```

---

## Key Takeaways

- **serde** provides a unified serialization framework
- **JSON** is widely supported with serde_json
- **YAML/TOML** are excellent for configuration files
- **Binary formats** offer better performance and smaller size
- **Attributes** provide fine-grained control over serialization
- **Error handling** should be comprehensive and user-friendly
- **Performance** varies significantly between formats

---

## Serialization Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Consistent naming** | Use standard field names | Apply serde(rename) when needed |
| **Version compatibility** | Handle format evolution | Use optional fields and defaults |
| **Error handling** | Provide clear error messages | Use thiserror for custom errors |
| **Performance** | Choose appropriate format | Binary for performance, JSON for compatibility |
| **Validation** | Validate data during deserialization | Use custom deserialize functions |
| **Documentation** | Document serialization format | Include examples and format specifications |
