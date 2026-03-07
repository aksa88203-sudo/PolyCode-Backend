// web_development.rs
// Comprehensive examples of web development in Rust

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

// =========================================
// BASIC WEB SERVER EXAMPLES
// =========================================

// Simple HTTP server using std::net
pub fn simple_http_server() -> std::io::Result<()> {
    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};
    use std::thread;
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Simple HTTP server listening on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        thread::spawn(move || {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                stream.read(&mut buffer).unwrap();
                
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n
                    <html><body><h1>Hello from Rust!</h1></body></html>";
                
                stream.write(response.as_bytes()).unwrap();
            }
        });
    }
    
    Ok(())
}

// Simulated Axum-like routing
#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn ok() -> Self {
        HttpResponse {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    
    pub fn not_found() -> Self {
        HttpResponse {
            status: 404,
            headers: HashMap::new(),
            body: "Not Found".to_string(),
        }
    }
    
    pub fn json<T: serde::Serialize>(data: T) -> Self {
        let mut response = HttpResponse::ok();
        response.headers.insert("Content-Type".to_string(), "application/json".to_string());
        response.body = serde_json::to_string(&data).unwrap_or_default();
        response
    }
    
    pub fn html(content: String) -> Self {
        let mut response = HttpResponse::ok();
        response.headers.insert("Content-Type".to_string(), "text/html".to_string());
        response.body = content;
        response
    }
}

// Simple router
pub struct Router {
    routes: HashMap<(HttpMethod, String), Box<dyn Fn(HttpRequest) -> HttpResponse + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }
    
    pub fn get<F>(&mut self, path: &str, handler: F) 
    where
        F: Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::GET, path.to_string()),
            Box::new(handler),
        );
    }
    
    pub fn post<F>(&mut self, path: &str, handler: F) 
    where
        F: Fn(HttpRequest) -> HttpResponse + Send + Sync + 'static,
    {
        self.routes.insert(
            (HttpMethod::POST, path.to_string()),
            Box::new(handler),
        );
    }
    
    pub fn handle_request(&self, request: HttpRequest) -> HttpResponse {
        if let Some(handler) = self.routes.get(&(request.method.clone(), request.path.clone())) {
            handler(request)
        } else {
            HttpResponse::not_found()
        }
    }
}

// =========================================
// REST API EXAMPLES
// =========================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

type UserStore = Arc<Mutex<HashMap<u32, User>>>;

// Handler functions
pub fn list_users(_request: HttpRequest) -> HttpResponse {
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            created_at: "2023-01-02T00:00:00Z".to_string(),
        },
    ];
    
    HttpResponse::json(users)
}

pub fn get_user(request: HttpRequest) -> HttpResponse {
    // Extract user ID from path (simplified)
    let path_parts: Vec<&str> = request.path.split('/').collect();
    
    if path_parts.len() >= 3 {
        if let Ok(id) = path_parts[2].parse::<u32>() {
            let user = User {
                id,
                name: format!("User {}", id),
                email: format!("user{}@example.com", id),
                created_at: "2023-01-01T00:00:00Z".to_string(),
            };
            return HttpResponse::json(user);
        }
    }
    
    HttpResponse::not_found()
}

pub fn create_user(request: HttpRequest) -> HttpResponse {
    if let Ok(create_user) = serde_json::from_str::<CreateUser>(&request.body) {
        let new_user = User {
            id: rand::random::<u32>(),
            name: create_user.name,
            email: create_user.email,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        
        HttpResponse::json(new_user)
    } else {
        let mut response = HttpResponse::ok();
        response.status = 400;
        response.body = "Invalid JSON".to_string();
        response
    }
}

pub fn update_user(request: HttpRequest) -> HttpResponse {
    // Extract user ID from path (simplified)
    let path_parts: Vec<&str> = request.path.split('/').collect();
    
    if path_parts.len() >= 3 {
        if let Ok(id) = path_parts[2].parse::<u32>() {
            if let Ok(update_user) = serde_json::from_str::<UpdateUser>(&request.body) {
                let mut user = User {
                    id,
                    name: format!("User {}", id),
                    email: format!("user{}@example.com", id),
                    created_at: "2023-01-01T00:00:00Z".to_string(),
                };
                
                if let Some(name) = update_user.name {
                    user.name = name;
                }
                
                if let Some(email) = update_user.email {
                    user.email = email;
                }
                
                return HttpResponse::json(user);
            }
        }
    }
    
    HttpResponse::not_found()
}

pub fn delete_user(request: HttpRequest) -> HttpResponse {
    // Extract user ID from path (simplified)
    let path_parts: Vec<&str> = request.path.split('/').collect();
    
    if path_parts.len() >= 3 {
        if let Ok(_id) = path_parts[2].parse::<u32>() {
            let mut response = HttpResponse::ok();
            response.status = 204; // No Content
            return response;
        }
    }
    
    HttpResponse::not_found()
}

pub fn create_rest_router() -> Router {
    let mut router = Router::new();
    
    router.get("/users", list_users);
    router.get("/users/:id", get_user);
    router.post("/users", create_user);
    router.put("/users/:id", update_user);
    router.delete("/users/:id", delete_user);
    
    router
}

// =========================================
// AUTHENTICATION EXAMPLES
// =========================================

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
    pub role: String,  // User role
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct UserInfo {
    pub id: u32,
    pub username: String,
    pub role: String,
}

// Simulated JWT functions
pub fn create_jwt(user: &UserInfo) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    
    let claims = Claims {
        sub: user.id.to_string(),
        exp: now + 3600, // 1 hour expiration
        iat: now,
        role: user.role.clone(),
    };
    
    // In real implementation, use proper JWT library
    format!("jwt_token_for_user_{}", user.id)
}

pub fn verify_jwt(token: &str) -> Option<Claims> {
    // In real implementation, use proper JWT verification
    if token.starts_with("jwt_token_for_user_") {
        let user_id: u32 = token.split('_').last()?.parse().ok()?;
        
        Some(Claims {
            sub: user_id.to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize + 3600,
            iat: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize,
            role: "user".to_string(),
        })
    } else {
        None
    }
}

pub fn login(request: HttpRequest) -> HttpResponse {
    if let Ok(login_req) = serde_json::from_str::<LoginRequest>(&request.body) {
        // Simulate user validation
        if login_req.username == "admin" && login_req.password == "password" {
            let user_info = UserInfo {
                id: 1,
                username: login_req.username,
                role: "admin".to_string(),
            };
            
            let token = create_jwt(&user_info);
            
            let response = LoginResponse {
                token,
                user: user_info,
            };
            
            return HttpResponse::json(response);
        }
    }
    
    let mut response = HttpResponse::ok();
    response.status = 401;
    response.body = "Invalid credentials".to_string();
    response
}

pub fn protected_endpoint(request: HttpRequest) -> HttpResponse {
    // Extract token from Authorization header
    let auth_header = request.headers.get("Authorization");
    
    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = &header[7..];
            
            if let Some(claims) = verify_jwt(token) {
                let response_data = serde_json::json!({
                    "message": "This is a protected endpoint",
                    "user_id": claims.sub,
                    "role": claims.role
                });
                
                return HttpResponse::json(response_data);
            }
        }
    }
    
    let mut response = HttpResponse::ok();
    response.status = 401;
    response.body = "Unauthorized".to_string();
    response
}

// =========================================
// MIDDLEWARE EXAMPLES
// =========================================

pub trait Middleware {
    fn handle(&self, request: HttpRequest, next: Box<dyn Fn(HttpRequest) -> HttpResponse>) -> HttpResponse;
}

pub struct LoggingMiddleware;

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: HttpRequest, next: Box<dyn Fn(HttpRequest) -> HttpResponse>) -> HttpResponse {
        println!("{} {} {:?}", request.method as u8, request.path, request.headers);
        
        let response = next(request);
        
        println!("Response status: {}", response.status);
        
        response
    }
}

pub struct CorsMiddleware;

impl Middleware for CorsMiddleware {
    fn handle(&self, request: HttpRequest, next: Box<dyn Fn(HttpRequest) -> HttpResponse>) -> HttpResponse {
        let response = next(request);
        
        // Add CORS headers
        let mut response = response;
        response.headers.insert("Access-Control-Allow-Origin".to_string(), "*".to_string());
        response.headers.insert("Access-Control-Allow-Methods".to_string(), "GET, POST, PUT, DELETE".to_string());
        response.headers.insert("Access-Control-Allow-Headers".to_string(), "Content-Type, Authorization".to_string());
        
        response
    }
}

pub struct AuthMiddleware;

impl Middleware for AuthMiddleware {
    fn handle(&self, request: HttpRequest, next: Box<dyn Fn(HttpRequest) -> HttpResponse>) -> HttpResponse {
        // Extract token from Authorization header
        let auth_header = request.headers.get("Authorization");
        
        if let Some(header) = auth_header {
            if header.starts_with("Bearer ") {
                let token = &header[7..];
                
                if verify_jwt(token).is_some() {
                    return next(request);
                }
            }
        }
        
        let mut response = HttpResponse::ok();
        response.status = 401;
        response.body = "Unauthorized".to_string();
        response
    }
}

// =========================================
// WEBSOCKET EXAMPLES
// =========================================

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
    pub timestamp: String,
}

pub struct ChatRoom {
    pub users: Arc<Mutex<HashMap<String, ChatUser>>>,
}

pub struct ChatUser {
    pub username: String,
    pub messages: Vec<ChatMessage>,
}

impl ChatRoom {
    pub fn new() -> Self {
        ChatRoom {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn add_user(&self, username: String) {
        let mut users = self.users.lock().unwrap();
        users.insert(username.clone(), ChatUser {
            username: username.clone(),
            messages: Vec::new(),
        });
        
        // Broadcast user joined
        let join_message = ChatMessage {
            username: "system".to_string(),
            message: format!("{} joined the chat", username),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        self.broadcast_message(join_message);
    }
    
    pub fn remove_user(&self, username: &str) {
        let mut users = self.users.lock().unwrap();
        users.remove(username);
        
        // Broadcast user left
        let leave_message = ChatMessage {
            username: "system".to_string(),
            message: format!("{} left the chat", username),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        self.broadcast_message(leave_message);
    }
    
    pub fn add_message(&self, username: String, message: String) {
        let chat_message = ChatMessage {
            username: username.clone(),
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        
        // Add to user's message history
        let mut users = self.users.lock().unwrap();
        if let Some(user) = users.get_mut(&username) {
            user.messages.push(chat_message.clone());
        }
        
        // Broadcast to all users
        self.broadcast_message(chat_message);
    }
    
    fn broadcast_message(&self, message: ChatMessage) {
        let users = self.users.lock().unwrap();
        
        if let Ok(message_json) = serde_json::to_string(&message) {
            // In real implementation, send to all connected WebSocket clients
            println!("Broadcasting: {}", message_json);
        }
    }
}

pub fn websocket_connect(chat_room: Arc<ChatRoom>, username: String) {
    println!("WebSocket connection for user: {}", username);
    
    chat_room.add_user(username.clone());
    
    // Simulate receiving messages
    let messages = vec![
        "Hello, everyone!",
        "How is everyone doing?",
        "This is a test message",
    ];
    
    for (i, msg) in messages.iter().enumerate() {
        std::thread::sleep(std::time::Duration::from_secs(1));
        chat_room.add_message(username.clone(), msg.to_string());
        println!("User {} sent: {}", username, msg);
    }
    
    // Simulate disconnect
    std::thread::sleep(std::time::Duration::from_secs(2));
    chat_room.remove_user(&username);
    println!("User {} disconnected", username);
}

// =========================================
// STATIC FILE SERVING
// =========================================

pub fn serve_static_file(request: HttpRequest) -> HttpResponse {
    // Extract file path from URL
    let path = request.path.strip_prefix("/static").unwrap_or("/index.html");
    let file_path = format!("static{}", path);
    
    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            let mut response = HttpResponse::ok();
            
            // Set content type based on file extension
            if path.ends_with(".html") {
                response.headers.insert("Content-Type".to_string(), "text/html".to_string());
            } else if path.ends_with(".css") {
                response.headers.insert("Content-Type".to_string(), "text/css".to_string());
            } else if path.ends_with(".js") {
                response.headers.insert("Content-Type".to_string(), "application/javascript".to_string());
            }
            
            response.body = content;
            response
        }
        Err(_) => {
            HttpResponse::not_found()
        }
    }
}

pub fn index_page() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Rust Web Development Demo</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .container { max-width: 800px; margin: 0 auto; }
        .endpoint { margin: 20px 0; padding: 20px; border: 1px solid #ccc; border-radius: 5px; }
        .method { color: #666; font-weight: bold; }
        .path { color: #0066cc; font-family: monospace; }
    </style>
</head>
<body>
    <div class="container">
        <h1>Rust Web Development Demo</h1>
        
        <div class="endpoint">
            <span class="method">GET</span> <span class="path">/users</span> - List all users
        </div>
        
        <div class="endpoint">
            <span class="method">GET</span> <span class="path">/users/:id</span> - Get user by ID
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span> <span class="path">/users</span> - Create new user
        </div>
        
        <div class="endpoint">
            <span class="method">PUT</span> <span class="path">/users/:id</span> - Update user
        </div>
        
        <div class="endpoint">
            <span class="method">DELETE</span> <span class="path">/users/:id</span> - Delete user
        </div>
        
        <div class="endpoint">
            <span class="method">POST</span> <span class="path">/login</span> - User login
        </div>
        
        <div class="endpoint">
            <span class="method">GET</span> <span class="path">/protected</span> - Protected endpoint
        </div>
    </div>
</body>
</html>
    "#;
    
    HttpResponse::html(html.to_string())
}

// =========================================
// DEMONSTRATION FUNCTIONS
// =========================================

pub fn demonstrate_rest_api() {
    println!("=== REST API DEMONSTRATION ===");
    
    let router = create_rest_router();
    
    // Test different endpoints
    let requests = vec![
        HttpRequest {
            method: HttpMethod::GET,
            path: "/users".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        },
        HttpRequest {
            method: HttpMethod::GET,
            path: "/users/1".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        },
        HttpRequest {
            method: HttpMethod::POST,
            path: "/users".to_string(),
            headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            body: r#"{"name": "Charlie", "email": "charlie@example.com"}"#.to_string(),
        },
    ];
    
    for request in requests {
        println!("Request: {} {}", request.method as u8, request.path);
        let response = router.handle_request(request);
        println!("Response: {} {}", response.status, response.body);
        println!();
    }
}

pub fn demonstrate_authentication() {
    println!("=== AUTHENTICATION DEMONSTRATION ===");
    
    let router = create_rest_router();
    
    // Test login
    let login_request = HttpRequest {
        method: HttpMethod::POST,
        path: "/login".to_string(),
        headers: HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
        body: r#"{"username": "admin", "password": "password"}"#.to_string(),
    };
    
    println!("Login request:");
    let login_response = login(login_request);
    println!("Response: {} {}", login_response.status, login_response.body);
    
    // Extract token from response
    if let Ok(login_data) = serde_json::from_str::<LoginResponse>(&login_response.body) {
        println!("Token: {}", login_data.token);
        
        // Test protected endpoint
        let protected_request = HttpRequest {
            method: HttpMethod::GET,
            path: "/protected".to_string(),
            headers: HashMap::from([("Authorization".to_string(), format!("Bearer {}", login_data.token))]),
            body: String::new(),
        };
        
        println!("\nProtected request:");
        let protected_response = protected_endpoint(protected_request);
        println!("Response: {} {}", protected_response.status, protected_response.body);
    }
    
    println!();
}

pub fn demonstrate_middleware() {
    println!("=== MIDDLEWARE DEMONSTRATION ===");
    
    let logging = LoggingMiddleware;
    let cors = CorsMiddleware;
    
    let request = HttpRequest {
        method: HttpMethod::GET,
        path: "/test".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    
    // Apply middleware chain
    let response = cors.handle(request, Box::new(|req| {
        logging.handle(req, Box::new(|_| {
            HttpResponse::ok()
        }))
    }));
    
    println!("Final response: {}", response.status);
    println!();
}

pub fn demonstrate_websocket() {
    println!("=== WEBSOCKET DEMONSTRATION ===");
    
    let chat_room = Arc::new(ChatRoom::new());
    
    // Simulate multiple users connecting
    let users = vec!["Alice", "Bob", "Charlie"];
    
    for username in users {
        let chat_room_clone = chat_room.clone();
        std::thread::spawn(move || {
            websocket_connect(chat_room_clone, username.to_string());
        });
    }
    
    // Wait for all connections to complete
    std::thread::sleep(std::time::Duration::from_secs(10));
    
    println!();
}

pub fn demonstrate_static_files() {
    println!("=== STATIC FILE SERVING DEMONSTRATION ===");
    
    let requests = vec![
        "/index.html",
        "/style.css",
        "/script.js",
        "/nonexistent.html",
    ];
    
    for path in requests {
        let request = HttpRequest {
            method: HttpMethod::GET,
            path: format!("/static{}", path),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        println!("Request: {}", path);
        let response = serve_static_file(request);
        println!("Response: {}", response.status);
        
        if response.status == 200 {
            println!("Content type: {:?}", response.headers.get("Content-Type"));
        }
        
        println!();
    }
}

// =========================================
// MAIN DEMONSTRATION
// =========================================

fn main() {
    println!("=== WEB DEVELOPMENT DEMONSTRATIONS ===\n");
    
    demonstrate_rest_api();
    demonstrate_authentication();
    demonstrate_middleware();
    demonstrate_websocket();
    demonstrate_static_files();
    
    // Show index page
    println!("=== INDEX PAGE ===");
    let index_response = index_page();
    println!("Index page response: {}", index_response.status);
    println!("Content length: {} bytes", index_response.body.len());
    
    println!("\n=== WEB DEVELOPMENT DEMONSTRATIONS COMPLETE ===");
    println!("Note: For production web development, consider using:");
    println!("- Axum: Modern async web framework");
    println!("- Actix Web: Actor-based web framework");
    println!("- Rocket: Type-safe web framework");
    println!("- Warp: Functional-style web framework");
    println!("- SQLx: Compile-time checked SQL");
    println!("- JWT libraries: jsonwebtoken, jsonwebtoken");
    println!("- WebSocket libraries: tokio-tungstenite, axum-websockets");
    println!("- Template engines: askama, tera, handlebars");
}

// =========================================
// UNIT TESTS
// =========================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_http_response_creation() {
        let response = HttpResponse::ok();
        assert_eq!(response.status, 200);
        
        let response = HttpResponse::not_found();
        assert_eq!(response.status, 404);
        
        let data = serde_json::json!({"message": "test"});
        let response = HttpResponse::json(data);
        assert_eq!(response.status, 200);
        assert!(response.headers.contains_key("Content-Type"));
    }
    
    #[test]
    fn test_router() {
        let mut router = Router::new();
        router.get("/test", |_req| HttpResponse::ok());
        
        let request = HttpRequest {
            method: HttpMethod::GET,
            path: "/test".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let response = router.handle_request(request);
        assert_eq!(response.status, 200);
        
        let request = HttpRequest {
            method: HttpMethod::GET,
            path: "/notfound".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let response = router.handle_request(request);
        assert_eq!(response.status, 404);
    }
    
    #[test]
    fn test_jwt_creation() {
        let user = UserInfo {
            id: 123,
            username: "testuser".to_string(),
            role: "user".to_string(),
        };
        
        let token = create_jwt(&user);
        assert!(token.starts_with("jwt_token_for_user_"));
    }
    
    #[test]
    fn test_jwt_verification() {
        let user = UserInfo {
            id: 123,
            username: "testuser".to_string(),
            role: "user".to_string(),
        };
        
        let token = create_jwt(&user);
        let claims = verify_jwt(&token);
        
        assert!(claims.is_some());
        let claims = claims.unwrap();
        assert_eq!(claims.sub, "123");
        assert_eq!(claims.role, "user");
    }
    
    #[test]
    fn test_chat_room() {
        let chat_room = ChatRoom::new();
        
        chat_room.add_user("Alice".to_string());
        chat_room.add_message("Alice".to_string(), "Hello, world!".to_string());
        
        let users = chat_room.users.lock().unwrap();
        assert!(users.contains_key("Alice"));
        
        let alice = users.get("Alice").unwrap();
        assert_eq!(alice.messages.len(), 2); // join message + chat message
    }
}
