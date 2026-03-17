# Web Development in Rust

## Overview

Rust has emerged as a powerful choice for web development, offering performance, safety, and a growing ecosystem of web frameworks and tools. This guide covers building web applications, APIs, and web services in Rust.

---

## Web Frameworks

### Major Web Frameworks

| Framework | Type | Features | Best For |
|-----------|-------|----------|-----------|
| **Axum** | Async | Built on Tokio, type-safe routing, middleware | High-performance APIs |
| **Actix Web** | Async/Actor | Actor model, middleware, WebSocket | Complex web applications |
| **Rocket** | Sync/Async | Type-safe routing, guards, templates | Rapid development |
| **Warp** | Async | Composable filters, WebSocket | Functional style APIs |
| **Tide** | Async | Minimal, middleware-focused | Simple web services |
| **Salvo** | Async | High performance, extensible | Production APIs |

### Framework Comparison

```rust
// Axum Example
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

async fn axum_example() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, Axum!" }))
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Actix Web Example
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn actix_index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[get("/users/{id}")]
async fn actix_get_user(path: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().json(format!("User ID: {}", *path))
}

async fn actix_example() {
    HttpServer::new(|| {
        App::new()
            .service(actix_index)
            .service(actix_get_user)
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
```

---

## Building REST APIs

### RESTful API with Axum

```rust
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

type AppState = Arc<Mutex<HashMap<u32, User>>>;

// GET /users
async fn list_users(
    Query(query): Query<UserQuery>,
    State(state): State<AppState>,
) -> Json<Vec<User>> {
    let users = state.lock().unwrap();
    let mut user_list: Vec<User> = users.values().cloned().collect();
    
    // Apply pagination
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(user_list.len());
    
    user_list.sort_by_key(|u| u.id);
    Json(user_list[start..end].to_vec())
}

// GET /users/:id
async fn get_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<User>, StatusCode> {
    let users = state.lock().unwrap();
    
    match users.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// POST /users
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.lock().unwrap();
    let new_id = users.len() as u32 + 1;
    
    let new_user = User {
        id: new_id,
        name: payload.name,
        email: payload.email,
    };
    
    users.insert(new_id, new_user.clone());
    Ok(Json(new_user))
}

// PUT /users/:id
async fn update_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    let mut users = state.lock().unwrap();
    
    match users.get_mut(&id) {
        Some(user) => {
            if let Some(name) = payload.name {
                user.name = name;
            }
            if let Some(email) = payload.email {
                user.email = email;
            }
            Ok(Json(user.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// DELETE /users/:id
async fn delete_user(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> StatusCode {
    let mut users = state.lock().unwrap();
    
    match users.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

pub async fn run_api() {
    let state: AppState = Arc::new(Mutex::new(HashMap::new()));
    
    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("API server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

---

## Database Integration

### SQLx with Axum

```rust
use sqlx::{postgres::PgPoolOptions, PgPool, Row};
use axum::{extract::State, response::Json, routing::get, Router};

#[derive(Debug, sqlx::FromRow)]
struct Post {
    id: i32,
    title: String,
    content: String,
    published: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

struct AppState {
    db: PgPool,
}

async fn create_database_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost/rust_web".to_string());
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

// GET /posts
async fn list_posts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Post>>, (StatusCode, String)> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(posts))
}

// GET /posts/:id
async fn get_post(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    
    Ok(Json(post))
}

// POST /posts
async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let title = payload.get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing title".to_string()))?;
    
    let content = payload.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "Missing content".to_string()))?;
    
    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (title, content, published)
        VALUES ($1, $2, true)
        RETURNING *
        "#
    )
    .bind(title)
    .bind(content)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(post))
}

pub async fn run_database_api() -> Result<(), Box<dyn std::error::Error>> {
    let db_pool = create_database_pool().await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;
    
    let state = AppState { db: db_pool };
    
    let app = Router::new()
        .route("/posts", get(list_posts).post(create_post))
        .route("/posts/:id", get(get_post))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Database API server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

---

## Authentication and Authorization

### JWT Authentication

```rust
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,  // Subject (user ID)
    exp: usize,   // Expiration time
    iat: usize,   // Issued at
    role: String,  // User role
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    token: String,
    user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    id: u32,
    username: String,
    role: String,
}

// JWT middleware
async fn auth_middleware(
    State(secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request.headers().get(header::AUTHORIZATION);
    
    let token = match auth_header {
        Some(header) => {
            let header_str = header.to_str().unwrap();
            if header_str.starts_with("Bearer ") {
                &header_str[7..]
            } else {
                return Err(StatusCode::UNAUTHORIZED);
            }
        }
        None => return Err(StatusCode::UNAUTHORIZED),
    };
    
    // Validate token
    let validation = Validation::new(
        jsonwebtoken::Algorithm::HS256,
        &jsonwebtoken::Validation::default(),
    );
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );
    
    match token_data {
        Ok(data) => {
            // Add user info to request extensions
            request.extensions_mut().insert(data.claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

// Login endpoint
async fn login(
    State(secret): State<String>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Validate credentials (in real app, check against database)
    if payload.username != "admin" || payload.password != "password" {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    
    let claims = Claims {
        sub: "1".to_string(),
        exp: now + 3600, // 1 hour expiration
        iat: now,
        role: "admin".to_string(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let user_info = UserInfo {
        id: 1,
        username: payload.username,
        role: claims.role,
    };
    
    Ok(Json(LoginResponse {
        token,
        user: user_info,
    }))
}

// Protected endpoint
async fn protected_endpoint(
    claims: axum::extract::Extension<Claims>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "This is a protected endpoint",
        "user_id": claims.sub,
        "role": claims.role
    }))
}

pub async fn run_auth_api() {
    let secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string());
    
    let app = Router::new()
        .route("/login", post(login))
        .route("/protected", get(protected_endpoint))
        .layer(axum::middleware::from_fn_with_state(
            secret.clone(),
            auth_middleware,
        ))
        .with_state(secret);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Auth API server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

---

## Middleware

### Custom Middleware

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use std::time::Instant;

// Logging middleware
async fn logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    println!(
        "{} {} {} {}ms",
        method,
        uri,
        duration.as_millis(),
        status.as_u16()
    );
    
    response
}

// CORS middleware
async fn cors_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let response = next.run(request).await;
    
    let mut response = response;
    let headers = response.headers_mut();
    
    headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "Content-Type, Authorization".parse().unwrap());
    
    Ok(response)
}

// Rate limiting middleware
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Clone)]
struct RateLimiter {
    requests: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    max_requests: u32,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: u32, window: Duration) -> Self {
        RateLimiter {
            requests: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }
    
    fn is_allowed(&self, ip: IpAddr) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();
        
        let entry = requests.entry(ip).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        entry.retain(|&time| now.duration_since(time) < self.window);
        
        // Check if under the limit
        if entry.len() < self.max_requests as usize {
            entry.push(now);
            true
        } else {
            false
        }
    }
}

async fn rate_limit_middleware(
    State(limiter): State<RateLimiter>,
    axum::extract::ConnectInfo(addr): axum::extract::ConnectInfo<std::net::SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if limiter.is_allowed(addr.ip()) {
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}
```

---

## WebSocket Support

### WebSocket Chat with Axum

```rust
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

type Users = Arc<Mutex<HashMap<String, WebSocket>>>;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct ChatMessage {
    username: String,
    message: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(users): State<Users>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, users))
}

async fn handle_socket(socket: WebSocket, users: Users) {
    let (mut sender, mut receiver) = socket.split();
    let username = format!("user_{}", rand::random::<u32>());
    
    // Add user to the list
    users.lock().unwrap().insert(username.clone(), sender.clone());
    
    // Send welcome message
    let welcome = ChatMessage {
        username: "system".to_string(),
        message: format!("Welcome to the chat, {}!", username),
        timestamp: chrono::Utc::now(),
    };
    
    if let Ok(welcome_json) = serde_json::to_string(&welcome) {
        let _ = sender.send(Message::Text(welcome_json)).await;
    }
    
    // Handle incoming messages
    let users_clone = users.clone();
    let username_clone = username.clone();
    
    let task = tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                        let mut chat_msg = chat_msg;
                        chat_msg.username = username_clone.clone();
                        chat_msg.timestamp = chrono::Utc::now();
                        
                        if let Ok(response) = serde_json::to_string(&chat_msg) {
                            // Broadcast to all users
                            let users = users_clone.lock().unwrap();
                            for (_, user_sender) in users.iter() {
                                let _ = user_sender.send(Message::Text(response.clone())).await;
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        
        // Remove user on disconnect
        users_clone.lock().unwrap().remove(&username_clone);
        
        let goodbye = ChatMessage {
            username: "system".to_string(),
            message: format!("{} left the chat", username_clone),
            timestamp: chrono::Utc::now(),
        };
        
        if let Ok(goodbye_json) = serde_json::to_string(&goodbye) {
            let users = users_clone.lock().unwrap();
            for (_, user_sender) in users.iter() {
                let _ = user_sender.send(Message::Text(goodbye_json.clone())).await;
            }
        }
    });
    
    task.await.unwrap();
}

pub async fn run_websocket_server() {
    let users: Users = Arc::new(Mutex::new(HashMap::new()));
    
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(users);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("WebSocket server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

---

## Static Files and Templates

### Static File Serving

```rust
use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::PathBuf;

// Static file handler
async fn static_file(
    Path(file_path): Path<PathBuf>,
) -> impl IntoResponse {
    let base_path = PathBuf::from("static");
    let full_path = base_path.join(file_path);
    
    match std::fs::read(&full_path) {
        Ok(contents) => {
            let mime_type = mime_guess::from_path(&full_path)
                .first_or_octet_stream()
                .as_ref()
                .to_string();
            
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type)
                .body(contents.into())
                .into_response()
        }
        Err(_) => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("File not found".into())
                .into_response()
        }
    }
}

// Template rendering with askama
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    users: Vec<User>,
}

async fn index_page() -> impl IntoResponse {
    let users = vec![
        User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() },
        User { id: 2, name: "Bob".to_string(), email: "bob@example.com".to_string() },
    ];
    
    let template = IndexTemplate {
        title: "User List".to_string(),
        users,
    };
    
    Html(template.render().unwrap())
}

pub async fn run_static_server() {
    let app = Router::new()
        .route("/", get(index_page))
        .route("/static/*file_path", get(static_file));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Static file server listening on 0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
```

---

## Key Takeaways

- **Axum** provides modern, type-safe async web development
- **SQLx** offers compile-time checked SQL queries
- **JWT** enables stateless authentication
- **Middleware** provides cross-cutting concerns
- **WebSockets** support real-time communication
- **Static files** and templates build complete web applications

---

## Web Development Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Type safety** | Leverage Rust's type system | Use typed routes and handlers |
| **Error handling** | Proper error responses | Use Result types consistently |
| **Security** | Authentication and validation | JWT, input validation, CORS |
| **Performance** | Async and connection pooling | Use async/await, database pools |
| **Testing** | Comprehensive testing | Unit tests, integration tests |
| **Logging** | Structured logging | Use tracing crate |
| **Configuration** | Environment-based config | Use dotenv, config crates |
