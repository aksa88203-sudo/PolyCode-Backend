// game_development.rs
// Game development examples in Rust

use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::time::Instant;

// 2D vector for game math
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

// Component trait for ECS
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// Position component
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
    
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
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

// Velocity component
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
    
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.dx, self.dy)
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

// Health component
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

// Render component
#[derive(Debug, Clone)]
pub struct Renderable {
    pub color: (u8, u8, u8),
    pub size: f32,
    pub visible: bool,
}

impl Renderable {
    pub fn new(color: (u8, u8, u8), size: f32) -> Self {
        Renderable {
            color,
            size,
            visible: true,
        }
    }
}

impl Component for Renderable {
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
    
    pub fn entities_with_component<C: Component + 'static>(&self) -> Vec<Entity> {
        let type_id = TypeId::of::<C>();
        if let Some(component_map) = self.components.get(&type_id) {
            component_map.keys().copied().collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn remove_component<C: Component + 'static>(&mut self, entity: Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        self.components.get_mut(&type_id)
            .and_then(|map| map.remove(&entity))
            .and_then(|component| component.into_any().downcast::<C>().ok())
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

// Rendering system
pub struct RenderingSystem {
    screen_width: f32,
    screen_height: f32,
}

impl RenderingSystem {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        RenderingSystem {
            screen_width,
            screen_height,
        }
    }
}

impl System for RenderingSystem {
    fn update(&mut self, storage: &mut ComponentStorage, _delta_time: f32) {
        println!("=== RENDERING FRAME ===");
        println!("Screen size: {}x{}", self.screen_width, self.screen_height);
        
        let entities_to_render: Vec<Entity> = storage.entities_with_component::<Position>()
            .into_iter()
            .filter(|entity| storage.get_component::<Renderable>(*entity).is_some())
            .collect();
        
        for entity in entities_to_render {
            if let (Some(position), Some(renderable)) = (
                storage.get_component::<Position>(entity),
                storage.get_component::<Renderable>(entity),
            ) {
                if renderable.visible {
                    println!("Entity {:?} at ({:.1}, {:.1}, {:.1}) size {:.1} color ({}, {}, {})",
                             entity,
                             position.x, position.y, position.z,
                             renderable.size,
                             renderable.color.0, renderable.color.1, renderable.color.2);
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
    
    pub fn get_entity_count(&self) -> usize {
        self.next_entity_id as usize
    }
}

// Physics body
#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub restitution: f32,
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
            self.acceleration = Vec2::zero();
        }
    }
    
    pub fn apply_gravity(&mut self, gravity: f32) {
        if !self.is_static {
            self.acceleration.y += gravity;
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
            return;
        }
        
        let normal = (body2.position - body1.position).normalize();
        let relative_velocity = body2.velocity - body1.velocity;
        let velocity_along_normal = relative_velocity.x * normal.x + relative_velocity.y * normal.y;
        
        if velocity_along_normal > 0.0 {
            return;
        }
        
        let restitution = body1.restitution.min(body2.restitution);
        let impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
        let impulse_scalar /= 1.0 / body1.mass + 1.0 / body2.mass;
        
        let impulse = normal * impulse_scalar;
        
        if !body1.is_static {
            body1.velocity = body1.velocity - impulse * (1.0 / body1.mass);
        }
        
        if !body2.is_static {
            body2.velocity = body2.velocity + impulse * (1.0 / body2.mass);
        }
        
        // Separate bodies
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
}

impl PhysicsWorld {
    pub fn new(gravity: f32) -> Self {
        PhysicsWorld {
            bodies: HashMap::new(),
            gravity,
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
        // Apply gravity
        for body in self.bodies.values_mut() {
            body.apply_gravity(self.gravity);
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
                        self.resolve_collision(name1, name2);
                    }
                }
            }
        }
    }
    
    fn resolve_collision(&mut self, name1: &str, name2: &str) {
        if let (Some(body1), Some(body2)) = (
            self.bodies.get_mut(name1),
            self.bodies.get_mut(name2),
        ) {
            CollisionDetector::resolve_collision(body1, body2);
        }
    }
    
    pub fn print_state(&self) {
        println!("=== PHYSICS WORLD STATE ===");
        println!("Gravity: {}", self.gravity);
        println!("Bodies: {}", self.bodies.len());
        
        for (name, body) in &self.bodies {
            println!("  {}: pos=({:.1}, {:.1}), vel=({:.1}, {:.1}), mass={:.1}",
                     name, body.position.x, body.position.y,
                     body.velocity.x, body.velocity.y, body.mass);
        }
    }
}

// Main demonstration
fn main() {
    println!("=== GAME DEVELOPMENT DEMONSTRATIONS ===\n");
    
    // ECS demonstration
    println!("=== ENTITY COMPONENT SYSTEM ===");
    let mut world = GameWorld::new();
    
    // Add systems
    world.add_system(MovementSystem);
    world.add_system(HealthSystem);
    world.add_system(RenderingSystem::new(800.0, 600.0));
    
    // Create player entity
    let player = world.create_entity();
    world.add_component(player, Position::new(400.0, 300.0, 0.0));
    world.add_component(player, Velocity::new(100.0, 50.0, 0.0));
    world.add_component(player, Health::new(100.0));
    world.add_component(player, Renderable::new((0, 255, 0), 32.0));
    
    // Create enemy entity
    let enemy = world.create_entity();
    world.add_component(enemy, Position::new(200.0, 100.0, 0.0));
    world.add_component(enemy, Velocity::new(-50.0, 25.0, 0.0));
    world.add_component(enemy, Health::new(50.0));
    world.add_component(enemy, Renderable::new((255, 0, 0), 24.0));
    
    // Create static obstacle
    let obstacle = world.create_entity();
    world.add_component(obstacle, Position::new(300.0, 200.0, 0.0));
    world.add_component(obstacle, Renderable::new((128, 128, 128), 40.0));
    
    println!("Created {} entities", world.get_entity_count());
    
    // Update world
    for i in 0..5 {
        println!("\n--- Frame {} ---", i + 1);
        world.update(0.016); // 60 FPS
        
        // Simulate some damage to enemy
        if i == 2 {
            if let Some(health) = world.get_component_mut::<Health>(enemy) {
                health.take_damage(25.0);
                println!("Enemy took 25 damage! Health: {:.1}", health.current);
            }
        }
    }
    
    // Physics demonstration
    println!("\n=== PHYSICS SIMULATION ===");
    let mut physics_world = PhysicsWorld::new(9.81); // Earth gravity
    
    // Add ground (static)
    let ground = PhysicsBody::static_body(Vec2::new(400.0, 580.0), 1000.0, 400.0);
    physics_world.add_body("ground".to_string(), ground);
    
    // Add falling ball
    let ball = PhysicsBody::new(Vec2::new(400.0, 100.0), 1.0, 20.0);
    physics_world.add_body("ball".to_string(), ball);
    
    // Add another ball
    let ball2 = PhysicsBody::new(Vec2::new(350.0, 50.0), 1.5, 25.0);
    physics_world.add_body("ball2".to_string(), ball2);
    
    println!("Initial state:");
    physics_world.print_state();
    
    // Simulate physics
    for i in 0..10 {
        println!("\n--- Physics Step {} ---", i + 1);
        physics_world.update(0.016); // 60 FPS
        physics_world.print_state();
        
        // Check if ball hit the ground
        if let Some(ball) = physics_world.get_body("ball") {
            if ball.position.y >= 560.0 {
                println!("Ball hit the ground!");
                break;
            }
        }
    }
    
    // Vector math demonstration
    println!("\n=== VECTOR MATH ===");
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(1.0, 2.0);
    
    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
    println!("v1 + v2 = {:?}", v1 + v2);
    println!("v1 - v2 = {:?}", v1 - v2);
    println!("v1 * 2.0 = {:?}", v1 * 2.0);
    println!("v1 magnitude = {:.2}", v1.magnitude());
    println!("v1 normalized = {:?}", v1.normalize());
    println!("distance between v1 and v2 = {:.2}", v1.distance_to(&v2));
    
    // Component demonstration
    println!("\n=== COMPONENT SYSTEMS ===");
    let position = Position::new(10.0, 20.0, 30.0);
    let velocity = Velocity::new(5.0, -3.0, 2.0);
    let health = Health::new(100.0);
    let renderable = Renderable::new((255, 255, 255), 1.0);
    
    println!("Position: {:?}", position);
    println!("Velocity: {:?}", velocity);
    println!("Health: {:.1}/{:.1}", health.current, health.maximum);
    println!("Renderable: color={:?}, size={:.1}", renderable.color, renderable.size);
    
    // Health component operations
    let mut health = Health::new(100.0);
    health.take_damage(30.0);
    println!("After taking 30 damage: {:.1}", health.current);
    health.heal(20.0);
    println!("After healing 20: {:.1}", health.current);
    println!("Is alive: {}", health.is_alive());
    
    println!("\n=== GAME DEVELOPMENT DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Entity Component System (ECS) architecture");
    println!("- Component-based game object design");
    println!("- System-based game logic processing");
    println!("- Physics simulation with collision detection");
    println!("- Vector mathematics for 2D games");
    println!("- Component storage and management");
    println!("- Game loop and update cycles");
    println!("- Rendering system integration");
    println!("- Health and damage systems");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_math() {
        let v1 = Vec2::new(3.0, 4.0);
        let v2 = Vec2::new(1.0, 2.0);
        
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v1 - v2, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(6.0, 8.0));
        assert!((v1.magnitude() - 5.0).abs() < 0.001);
        assert_eq!(v1.distance_to(&v2), (5.0_f64).sqrt() as f32);
    }
    
    #[test]
    fn test_health_component() {
        let mut health = Health::new(100.0);
        
        assert!(health.is_alive());
        assert_eq!(health.current, 100.0);
        
        health.take_damage(50.0);
        assert_eq!(health.current, 50.0);
        assert!(health.is_alive());
        
        health.take_damage(60.0);
        assert_eq!(health.current, 0.0);
        assert!(!health.is_alive());
        
        health.heal(30.0);
        assert_eq!(health.current, 30.0);
        assert!(health.is_alive());
    }
    
    #[test]
    fn test_physics_body() {
        let mut body = PhysicsBody::new(Vec2::new(0.0, 0.0), 1.0, 10.0);
        
        body.apply_force(Vec2::new(10.0, 0.0));
        body.update(1.0);
        
        assert!(body.velocity.x > 0.0);
        assert!(body.position.x > 0.0);
        
        let static_body = PhysicsBody::static_body(Vec2::new(100.0, 0.0), 1000.0, 50.0);
        assert!(static_body.is_static);
    }
    
    #[test]
    fn test_collision_detection() {
        let body1 = PhysicsBody::new(Vec2::new(0.0, 0.0), 1.0, 10.0);
        let body2 = PhysicsBody::new(Vec2::new(15.0, 0.0), 1.0, 10.0);
        let body3 = PhysicsBody::new(Vec2::new(25.0, 0.0), 1.0, 10.0);
        
        assert!(CollisionDetector::check_collision(&body1, &body2)); // Overlapping
        assert!(!CollisionDetector::check_collision(&body1, &body3)); // Not overlapping
    }
    
    #[test]
    fn test_ecs() {
        let mut world = GameWorld::new();
        
        let entity = world.create_entity();
        world.add_component(entity, Position::new(10.0, 20.0, 30.0));
        world.add_component(entity, Velocity::new(5.0, 3.0, 1.0));
        
        assert!(world.get_component::<Position>(entity).is_some());
        assert!(world.get_component::<Velocity>(entity).is_some());
        assert!(world.get_component::<Health>(entity).is_none());
        
        let position = world.get_component::<Position>(entity).unwrap();
        assert_eq!(position.x, 10.0);
        assert_eq!(position.y, 20.0);
        assert_eq!(position.z, 30.0);
    }
}
