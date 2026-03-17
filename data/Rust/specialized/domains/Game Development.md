# Game Development in Rust

## Overview

Rust's performance, memory safety, and growing ecosystem make it increasingly popular for game development. This guide covers game engines, graphics programming, physics simulation, and building games in Rust.

---

## Game Development Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `bevy` | Game engine | ECS, rendering, audio |
| `ggez` | 2D game framework | Simple 2D games |
| `winit` | Window management | Cross-platform windows |
| `wgpu` | Graphics abstraction | Vulkan, Metal, DirectX |
| `rapier3d` | Physics engine | 2D/3D physics simulation |
| `rodio` | Audio playback | Cross-platform audio |
| `specs` | ECS framework | Entity Component System |
| `nalgebra` | Linear algebra | Math for games |

---

## Game Engine Architecture

### Entity Component System (ECS)

```rust
use std::collections::HashMap;
use std::any::{Any, TypeId};

// Component system
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Basic components
#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Position { x, y, z }
    }
}

impl Component for Position {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
    pub dz: f32,
}

impl Velocity {
    pub fn new(dx: f32, dy: f32, dz: f32) -> Self {
        Velocity { dx, dy, dz }
    }
}

impl Component for Velocity {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

impl Health {
    pub fn new(maximum: f32) -> Self {
        Health {
            current: maximum,
            maximum,
        }
    }
    
    pub fn take_damage(&mut self, amount: f32) {
        self.current = (self.current - amount).max(0.0);
    }
    
    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.maximum);
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }
}

impl Component for Health {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Entity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub id: u64,
}

impl Entity {
    pub fn new(id: u64) -> Self {
        Entity { id }
    }
}

// Component storage
pub struct ComponentStorage {
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Component>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            components: HashMap::new(),
        }
    }
    
    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        let type_id = TypeId::of::<C>();
        let component_map = self.components.entry(type_id).or_insert_with(HashMap::new);
        component_map.insert(entity, Box::new(component));
    }
    
    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        self.components.get(&type_id)
            .and_then(|map| map.get(&entity))
            .and_then(|component| component.as_any().downcast_ref::<C>())
    }
    
    pub fn get_component_mut<C: Component + 'static>(&mut self, entity: Entity) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        self.components.get_mut(&type_id)
            .and_then(|map| map.get_mut(&entity))
            .and_then(|component| component.as_any_mut().downcast_mut::<C>())
    }
    
    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.components.get_mut(&type_id)
            .and_then(|map| map.remove(&entity))
            .and_then(|component| component.into_any().downcast::<C>().ok())
    }
    
    pub fn entities_with_component<C: Component + 'static>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<C>();
        if let Some(component_map) = self.components.get(&type_id) {
            component_map.keys().copied().collect()
        } else {
            Vec::new()
        }
    }
}

// System trait
pub trait System {
    fn update(&mut self, storage: &mut ComponentStorage, delta_time: f32);
}

// Movement system
pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&mut self, storage: &mut ComponentStorage, delta_time: f32) {
        let entities_to_move: Vec<Entity> = storage.entities_with_component::<Position>()
            .into_iter()
            .filter(|entity| storage.get_component::<Velocity>(*entity).is_some())
            .collect();
        
        for entity in entities_to_move {
            if let (Some(position), Some(velocity)) = (
                storage.get_component_mut::<Position>(entity),
                storage.get_component::<Velocity>(entity),
            ) {
                position.x += velocity.dx * delta_time;
                position.y += velocity.dy * delta_time;
                position.z += velocity.dz * delta_time;
            }
        }
    }
}

// Health system
pub struct HealthSystem;

impl System for HealthSystem {
    fn update(&mut self, storage: &mut ComponentStorage, _delta_time: f32) {
        let entities_to_check: Vec<Entity> = storage.entities_with_component::<Health>();
        
        for entity in entities_to_check {
            if let Some(health) = storage.get_component::<Health>(entity) {
                if !health.is_alive() {
                    println!("Entity {:?} has died!", entity);
                    // In a real game, you would remove the entity or trigger death behavior
                }
            }
        }
    }
}

// Game world
pub struct GameWorld {
    storage: ComponentStorage,
    systems: Vec<Box<dyn System>>,
    next_entity_id: u64,
}

impl GameWorld {
    pub fn new() -> Self {
        GameWorld {
            storage: ComponentStorage::new(),
            systems: Vec::new(),
            next_entity_id: 0,
        }
    }
    
    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity::new(self.next_entity_id);
        self.next_entity_id += 1;
        entity
    }
    
    pub fn add_component<C: Component + 'static>(&mut self, entity: Entity, component: C) {
        self.storage.add_component(entity, component);
    }
    
    pub fn get_component<C: Component + 'static>(&self, entity: Entity) -> Option<&C> {
        self.storage.get_component(entity)
    }
    
    pub fn get_component_mut<C: Component + 'static>(&mut self, entity: Entity) -> Option<&mut C> {
        self.storage.get_component_mut(entity)
    }
    
    pub fn add_system<S: System + 'static>(&mut self, system: S) {
        self.systems.push(Box::new(system));
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for system in &mut self.systems {
            system.update(&mut self.storage, delta_time);
        }
    }
}
```

### 2D Game Framework

```rust
use std::collections::HashMap;
use std::time::Instant;

// 2D vector
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    
    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vec2::new(self.x / mag, self.y / mag)
        } else {
            Vec2::zero()
        }
    }
    
    pub fn distance_to(&self, other: &Vec2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    
    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

// 2D sprite
#[derive(Debug, Clone)]
pub struct Sprite {
    pub position: Vec2,
    pub size: Vec2,
    pub color: (u8, u8, u8),
    pub visible: bool,
}

impl Sprite {
    pub fn new(position: Vec2, size: Vec2, color: (u8, u8, u8)) -> Self {
        Sprite {
            position,
            size,
            color,
            visible: true,
        }
    }
    
    pub fn contains_point(&self, point: &Vec2) -> bool {
        point.x >= self.position.x &&
        point.x <= self.position.x + self.size.x &&
        point.y >= self.position.y &&
        point.y <= self.position.y + self.size.y
    }
}

// 2D renderer
pub struct Renderer2D {
    sprites: HashMap<String, Sprite>,
    screen_size: Vec2,
}

impl Renderer2D {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Renderer2D {
            sprites: HashMap::new(),
            screen_size: Vec2::new(screen_width, screen_height),
        }
    }
    
    pub fn add_sprite(&mut self, name: String, sprite: Sprite) {
        self.sprites.insert(name, sprite);
    }
    
    pub fn update_sprite(&mut self, name: &str, sprite: Sprite) {
        if let Some(existing) = self.sprites.get_mut(name) {
            *existing = sprite;
        }
    }
    
    pub fn render(&self) {
        println!("=== RENDERING FRAME ===");
        println!("Screen size: {}x{}", self.screen_size.x, self.screen_size.y);
        
        for (name, sprite) in &self.sprites {
            if sprite.visible {
                println!("Sprite '{}' at ({:.1}, {:.1}) size {:.1}x{:.1} color ({}, {}, {})",
                         name,
                         sprite.position.x, sprite.position.y,
                         sprite.size.x, sprite.size.y,
                         sprite.color.0, sprite.color.1, sprite.color.2);
            }
        }
    }
    
    pub fn get_sprite(&self, name: &str) -> Option<&Sprite> {
        self.sprites.get(name)
    }
    
    pub fn get_sprites_at_position(&self, position: &Vec2) -> Vec<&str> {
        self.sprites.iter()
            .filter(|(_, sprite)| sprite.visible && sprite.contains_point(position))
            .map(|(name, _)| name.as_str())
            .collect()
    }
}

// Input handler
#[derive(Debug, Clone, Copy)]
pub enum InputKey {
    Up,
    Down,
    Left,
    Right,
    Space,
    Escape,
}

#[derive(Debug)]
pub struct InputState {
    keys_pressed: HashMap<InputKey, bool>,
    mouse_position: Vec2,
    mouse_clicked: bool,
}

impl InputState {
    pub fn new() -> Self {
        InputState {
            keys_pressed: HashMap::new(),
            mouse_position: Vec2::zero(),
            mouse_clicked: false,
        }
    }
    
    pub fn press_key(&mut self, key: InputKey) {
        self.keys_pressed.insert(key, true);
    }
    
    pub fn release_key(&mut self, key: InputKey) {
        self.keys_pressed.insert(key, false);
    }
    
    pub fn is_key_pressed(&self, key: InputKey) -> bool {
        self.keys_pressed.get(&key).copied().unwrap_or(false)
    }
    
    pub fn set_mouse_position(&mut self, position: Vec2) {
        self.mouse_position = position;
    }
    
    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }
    
    pub fn set_mouse_clicked(&mut self, clicked: bool) {
        self.mouse_clicked = clicked;
    }
    
    pub fn is_mouse_clicked(&self) -> bool {
        self.mouse_clicked
    }
}

// Simple 2D game
pub struct Game2D {
    renderer: Renderer2D,
    input: InputState,
    player_position: Vec2,
    player_velocity: Vec2,
    enemies: Vec<Sprite>,
    score: u32,
    game_over: bool,
    last_update: Instant,
}

impl Game2D {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let mut renderer = Renderer2D::new(screen_width, screen_height);
        
        // Add player
        let player = Sprite::new(
            Vec2::new(screen_width / 2.0, screen_height - 50.0),
            Vec2::new(50.0, 50.0),
            (0, 255, 0),
        );
        renderer.add_sprite("player".to_string(), player);
        
        Game2D {
            renderer,
            input: InputState::new(),
            player_position: Vec2::new(screen_width / 2.0, screen_height - 50.0),
            player_velocity: Vec2::zero(),
            enemies: Vec::new(),
            score: 0,
            game_over: false,
            last_update: Instant::now(),
        }
    }
    
    pub fn handle_input(&mut self) {
        if self.game_over {
            if self.input.is_key_pressed(InputKey::Space) {
                self.restart_game();
            }
            return;
        }
        
        // Player movement
        self.player_velocity = Vec2::zero();
        
        if self.input.is_key_pressed(InputKey::Left) {
            self.player_velocity.x = -200.0;
        }
        if self.input.is_key_pressed(InputKey::Right) {
            self.player_velocity.x = 200.0;
        }
        if self.input.is_key_pressed(InputKey::Up) {
            self.player_velocity.y = -200.0;
        }
        if self.input.is_key_pressed(InputKey::Down) {
            self.player_velocity.y = 200.0;
        }
        
        // Shooting
        if self.input.is_key_pressed(InputKey::Space) {
            self.shoot();
        }
    }
    
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;
        
        // Update player position
        self.player_position = self.player_position + self.player_velocity * delta_time;
        
        // Keep player in bounds
        let screen_size = self.renderer.screen_size;
        self.player_position.x = self.player_position.x.max(0.0).min(screen_size.x - 50.0);
        self.player_position.y = self.player_position.y.max(0.0).min(screen_size.y - 50.0);
        
        // Update player sprite
        let player_sprite = Sprite::new(
            self.player_position,
            Vec2::new(50.0, 50.0),
            (0, 255, 0),
        );
        self.renderer.update_sprite("player", player_sprite);
        
        // Spawn enemies
        if rand::random::<f32>() < 0.02 {
            self.spawn_enemy();
        }
        
        // Update enemies
        self.update_enemies(delta_time);
        
        // Check collisions
        self.check_collisions();
        
        // Check game over
        if self.enemies.iter().any(|enemy| {
            enemy.position.y > self.renderer.screen_size.y - 100.0
        }) {
            self.game_over = true;
        }
    }
    
    fn spawn_enemy(&mut self) {
        let x = rand::random::<f32>() * self.renderer.screen_size.x;
        let enemy = Sprite::new(
            Vec2::new(x, 0.0),
            Vec2::new(30.0, 30.0),
            (255, 0, 0),
        );
        
        let name = format!("enemy_{}", self.enemies.len());
        self.renderer.add_sprite(name.clone(), enemy.clone());
        self.enemies.push(enemy);
    }
    
    fn update_enemies(&mut self, delta_time: f32) {
        for enemy in &mut self.enemies {
            enemy.position.y += 50.0 * delta_time; // Move down
        }
        
        // Update enemy sprites
        for (i, enemy) in self.enemies.iter().enumerate() {
            let name = format!("enemy_{}", i);
            self.renderer.update_sprite(&name, enemy.clone());
        }
    }
    
    fn shoot(&mut self) {
        // Simple shooting - just increase score for demo
        self.score += 10;
        
        // In a real game, you would create bullet sprites
        println!("Bang! Score: {}", self.score);
    }
    
    fn check_collisions(&mut self) {
        let player_sprite = Sprite::new(
            self.player_position,
            Vec2::new(50.0, 50.0),
            (0, 255, 0),
        );
        
        self.enemies.retain(|enemy| {
            let collision = enemy.contains_point(&player_sprite.position) ||
                           player_sprite.contains_point(&enemy.position);
            
            if collision {
                self.score += 100;
                println!("Enemy destroyed! Score: {}", self.score);
                false // Remove enemy
            } else {
                true // Keep enemy
            }
        });
        
        // Remove destroyed enemy sprites
        for i in self.enemies.len()..100 { // Clean up old sprites
            let name = format!("enemy_{}", i);
            self.renderer.sprites.remove(&name);
        }
    }
    
    fn restart_game(&mut self) {
        self.game_over = false;
        self.score = 0;
        self.player_position = Vec2::new(self.renderer.screen_size.x / 2.0, self.renderer.screen_size.y - 50.0);
        self.enemies.clear();
        
        // Clear enemy sprites
        for i in 0..100 {
            let name = format!("enemy_{}", i);
            self.renderer.sprites.remove(&name);
        }
        
        println!("Game restarted!");
    }
    
    pub fn render(&self) {
        self.renderer.render();
        
        if self.game_over {
            println!("GAME OVER! Score: {}", self.score);
            println!("Press SPACE to restart");
        } else {
            println!("Score: {}", self.score);
        }
    }
    
    pub fn simulate_input(&mut self) {
        // Simulate random input for demonstration
        use std::collections::hash_map::Entry;
        
        // Random key presses
        let keys = vec![InputKey::Left, InputKey::Right, InputKey::Up, InputKey::Down, InputKey::Space];
        
        for key in keys {
            if rand::random::<f32>() < 0.1 {
                self.input.press_key(key);
            } else {
                self.input.release_key(key);
            }
        }
        
        // Random mouse position
        let mouse_x = rand::random::<f32>() * self.renderer.screen_size.x;
        let mouse_y = rand::random::<f32>() * self.renderer.screen_size.y;
        self.input.set_mouse_position(Vec2::new(mouse_x, mouse_y));
        self.input.set_mouse_clicked(rand::random::<bool>());
    }
}
```

---

## Physics Simulation

### Basic Physics Engine

```rust
use std::collections::HashMap;

// Physics body
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub restitution: f32, // Bounciness (0 = no bounce, 1 = perfect bounce)
    pub is_static: bool,
}

impl PhysicsBody {
    pub fn new(position: Vec2, mass: f32, radius: f32) -> Self {
        PhysicsBody {
            position,
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
            mass,
            radius,
            restitution: 0.8,
            is_static: false,
        }
    }
    
    pub fn static_body(position: Vec2, mass: f32, radius: f32) -> Self {
        let mut body = Self::new(position, mass, radius);
        body.is_static = true;
        body
    }
    
    pub fn apply_force(&mut self, force: Vec2) {
        if !self.is_static {
            self.acceleration = self.acceleration + force * (1.0 / self.mass);
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if !self.is_static {
            self.velocity = self.velocity + self.acceleration * delta_time;
            self.position = self.position + self.velocity * delta_time;
            self.acceleration = Vec2::zero(); // Reset acceleration
        }
    }
    
    pub fn apply_gravity(&mut self, gravity: f32) {
        if !self.is_static {
            self.acceleration.y += gravity;
        }
    }
    
    pub fn apply_friction(&mut self, friction_coefficient: f32) {
        if !self.is_static {
            let friction = self.velocity * (-friction_coefficient);
            self.velocity = self.velocity + friction;
        }
    }
}

// Collision detection
pub struct CollisionDetector;

impl CollisionDetector {
    pub fn check_collision(body1: &PhysicsBody, body2: &PhysicsBody) -> bool {
        let distance = body1.position.distance_to(&body2.position);
        distance < (body1.radius + body2.radius)
    }
    
    pub fn resolve_collision(body1: &mut PhysicsBody, body2: &mut PhysicsBody) {
        if body1.is_static && body2.is_static {
            return; // Two static bodies can't move
        }
        
        // Calculate collision normal
        let normal = (body2.position - body1.position).normalize();
        
        // Relative velocity
        let relative_velocity = body2.velocity - body1.velocity;
        
        // Velocity along collision normal
        let velocity_along_normal = relative_velocity.x * normal.x + relative_velocity.y * normal.y;
        
        // Don't resolve if bodies are separating
        if velocity_along_normal > 0.0 {
            return;
        }
        
        // Calculate restitution
        let restitution = body1.restitution.min(body2.restitution);
        
        // Calculate impulse scalar
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
        let impulse_scalar /= 1.0 / body1.mass + 1.0 / body2.mass;
        
        // Apply impulse
        let impulse = normal * impulse_scalar;
        
        if !body1.is_static {
            body1.velocity = body1.velocity - impulse * (1.0 / body1.mass);
        }
        
        if !body2.is_static {
            body2.velocity = body2.velocity + impulse * (1.0 / body2.mass);
        }
        
        // Separate bodies to prevent overlap
        let overlap = (body1.radius + body2.radius) - body1.position.distance_to(&body2.position);
        if overlap > 0.0 {
            let separation = normal * (overlap / 2.0);
            
            if !body1.is_static {
                body1.position = body1.position - separation;
            }
            
            if !body2.is_static {
                body2.position = body2.position + separation;
            }
        }
    }
}

// Physics world
pub struct PhysicsWorld {
    bodies: HashMap<String, PhysicsBody>,
    gravity: f32,
    friction: f32,
}

impl PhysicsWorld {
    pub fn new(gravity: f32, friction: f32) -> Self {
        PhysicsWorld {
            bodies: HashMap::new(),
            gravity,
            friction,
        }
    }
    
    pub fn add_body(&mut self, name: String, body: PhysicsBody) {
        self.bodies.insert(name, body);
    }
    
    pub fn get_body(&self, name: &str) -> Option<&PhysicsBody> {
        self.bodies.get(name)
    }
    
    pub fn get_body_mut(&mut self, name: &str) -> Option<&mut PhysicsBody> {
        self.bodies.get_mut(name)
    }
    
    pub fn update(&mut self, delta_time: f32) {
        // Apply forces
        for body in self.bodies.values_mut() {
            body.apply_gravity(self.gravity);
            body.apply_friction(self.friction);
        }
        
        // Update positions
        for body in self.bodies.values_mut() {
            body.update(delta_time);
        }
        
        // Check collisions
        let body_names: Vec<String> = self.bodies.keys().cloned().collect();
        
        for i in 0..body_names.len() {
            for j in i + 1..body_names.len() {
                let name1 = &body_names[i];
                let name2 = &body_names[j];
                
                if let (Some(body1), Some(body2)) = (
                    self.bodies.get(name1).cloned(),
                    self.bodies.get(name2).cloned(),
                ) {
                    if CollisionDetector::check_collision(&body1, &body2) {
                        // We need to borrow mutably, so we'll do this in a separate step
                        self.resolve_collision(name1, name2);
                    }
                }
            }
        }
    }
    
    fn resolve_collision(&mut self, name1: &str, name2: &str) {
        // This is a simplified approach - in practice you'd want to be more careful with borrowing
        if let (Some(body1), Some(body2)) = (
            self.bodies.get_mut(name1),
            self.bodies.get_mut(name2),
        ) {
            CollisionDetector::resolve_collision(body1, body2);
        }
    }
    
    pub fn apply_force_to_body(&mut self, name: &str, force: Vec2) {
        if let Some(body) = self.bodies.get_mut(name) {
            body.apply_force(force);
        }
    }
    
    pub fn set_body_velocity(&mut self, name: &str, velocity: Vec2) {
        if let Some(body) = self.bodies.get_mut(name) {
            body.velocity = velocity;
        }
    }
    
    pub fn get_body_count(&self) -> usize {
        self.bodies.len()
    }
    
    pub fn print_state(&self) {
        println!("=== PHYSICS WORLD STATE ===");
        println!("Gravity: {}, Friction: {}", self.gravity, self.friction);
        println!("Bodies: {}", self.bodies.len());
        
        for (name, body) in &self.bodies {
            println!("  {}: pos=({:.1}, {:.1}), vel=({:.1}, {:.1}), mass={:.1}",
                     name, body.position.x, body.position.y,
                     body.velocity.x, body.velocity.y, body.mass);
        }
    }
}
```

---

## Key Takeaways

- **ECS architecture** provides flexible game object management
- **Component-based design** enables code reuse
- **Physics simulation** requires careful collision detection
- **Input handling** needs to be responsive and accurate
- **Rendering optimization** is crucial for performance
- **Game loops** must handle timing consistently
- **State management** is essential for complex games

---

## Game Development Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Fixed timestep** | Consistent physics simulation | Delta time normalization |
| **Object pooling** | Reduce allocation overhead | Reuse game objects |
| **Spatial partitioning** | Optimize collision detection | Grid-based systems |
| **Asset management** | Efficient resource loading | Asset pipelines |
| **Frame rate limiting** | Consistent performance | FPS control |
| **Input buffering** | Responsive controls | Input queues |
| **State machines** | Manage game states | State patterns |
