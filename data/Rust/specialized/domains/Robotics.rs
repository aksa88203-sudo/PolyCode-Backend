// robotics.rs
// Robotics implementation examples in Rust

use std::collections::HashMap;
use std::cmp::Ordering;

// Simple 3D vector for robotics
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }
    
    pub fn zeros() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }
    
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Vector3 {
        let norm = self.norm();
        if norm > 0.0 {
            Vector3::new(self.x / norm, self.y / norm, self.z / norm)
        } else {
            Vector3::zeros()
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;
    
    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;
    
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    
    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Robot state
#[derive(Debug, Clone)]
pub struct RobotState {
    pub position: Vector3,
    pub velocity: Vector3,
    pub orientation: f64, // Yaw angle in radians
    pub timestamp: f64,
}

impl RobotState {
    pub fn new() -> Self {
        RobotState {
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            orientation: 0.0,
            timestamp: 0.0,
        }
    }
    
    pub fn predict_next(&self, dt: f64) -> RobotState {
        RobotState {
            position: self.position + self.velocity * dt,
            velocity: self.velocity,
            orientation: self.orientation,
            timestamp: self.timestamp + dt,
        }
    }
}

// Sensor interface
pub trait Sensor {
    type Data;
    type Error;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error>;
    fn calibrate(&mut self) -> Result<(), Self::Error>;
}

// Lidar sensor
#[derive(Debug)]
pub struct LidarSensor {
    angle_range: f64,
    resolution: usize,
    max_range: f64,
}

#[derive(Debug, Clone)]
pub struct LidarScan {
    pub ranges: Vec<f64>,
    pub angles: Vec<f64>,
    pub timestamp: f64,
}

impl LidarSensor {
    pub fn new(angle_range: f64, resolution: usize) -> Self {
        LidarSensor {
            angle_range,
            resolution,
            max_range: 10.0,
        }
    }
    
    fn generate_angles(&self) -> Vec<f64> {
        let start_angle = -self.angle_range / 2.0;
        let angle_step = self.angle_range / self.resolution as f64;
        (0..self.resolution)
            .map(|i| start_angle + i as f64 * angle_step)
            .collect()
    }
}

impl Sensor for LidarSensor {
    type Data = LidarScan;
    type Error = String;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error> {
        let angles = self.generate_angles();
        let ranges = vec![5.0; self.resolution]; // Simulated ranges
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        Ok(LidarScan {
            ranges,
            angles,
            timestamp,
        })
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating LiDAR sensor...");
        Ok(())
    }
}

// IMU sensor
#[derive(Debug)]
pub struct IMUSensor {
    bias: Vector3,
    noise_level: f64,
}

#[derive(Debug, Clone)]
pub struct IMUData {
    pub acceleration: Vector3,
    pub angular_velocity: Vector3,
    pub timestamp: f64,
}

impl IMUSensor {
    pub fn new() -> Self {
        IMUSensor {
            bias: Vector3::new(0.01, 0.01, 0.01),
            noise_level: 0.001,
        }
    }
}

impl Sensor for IMUSensor {
    type Data = IMUData;
    type Error = String;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // Simulate IMU reading
        let acceleration = Vector3::new(0.0, 0.0, -9.81); // Gravity
        let angular_velocity = Vector3::new(0.1, 0.0, 0.0);
        
        Ok(IMUData {
            acceleration,
            angular_velocity,
            timestamp,
        })
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating IMU sensor...");
        self.bias = Vector3::zeros();
        Ok(())
    }
}

// Grid position for path planning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPosition {
    pub x: isize,
    pub y: isize,
}

impl GridPosition {
    pub fn new(x: isize, y: isize) -> Self {
        GridPosition { x, y }
    }
    
    pub fn distance_to(&self, other: &GridPosition) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
    
    pub fn neighbors(&self) -> Vec<GridPosition> {
        vec![
            GridPosition::new(self.x - 1, self.y),
            GridPosition::new(self.x + 1, self.y),
            GridPosition::new(self.x, self.y - 1),
            GridPosition::new(self.x, self.y + 1),
        ]
    }
}

// Occupancy grid
#[derive(Debug)]
pub struct OccupancyGrid {
    grid: HashMap<GridPosition, bool>,
    resolution: f64,
}

impl OccupancyGrid {
    pub fn new(width: isize, height: isize, resolution: f64) -> Self {
        let mut grid = HashMap::new();
        
        for x in 0..width {
            for y in 0..height {
                grid.insert(GridPosition::new(x, y), false);
            }
        }
        
        OccupancyGrid { grid, resolution }
    }
    
    pub fn set_occupied(&mut self, pos: GridPosition, occupied: bool) {
        self.grid.insert(pos, occupied);
    }
    
    pub fn is_occupied(&self, pos: &GridPosition) -> bool {
        self.grid.get(pos).copied().unwrap_or(true)
    }
    
    pub fn world_to_grid(&self, world_pos: &Vector3) -> GridPosition {
        GridPosition::new(
            (world_pos.x / self.resolution).round() as isize,
            (world_pos.y / self.resolution).round() as isize,
        )
    }
    
    pub fn grid_to_world(&self, grid_pos: &GridPosition) -> Vector3 {
        Vector3::new(
            grid_pos.x as f64 * self.resolution,
            grid_pos.y as f64 * self.resolution,
            0.0,
        )
    }
}

// A* pathfinding
#[derive(Debug, Eq, PartialEq)]
struct PathNode {
    position: GridPosition,
    g_score: f64,
    f_score: f64,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct PathPlanner {
    grid: OccupancyGrid,
}

impl PathPlanner {
    pub fn new(grid: OccupancyGrid) -> Self {
        PathPlanner { grid }
    }
    
    pub fn find_path(&self, start: GridPosition, goal: GridPosition) -> Option<Vec<GridPosition>> {
        use std::collections::{HashMap, BinaryHeap};
        
        if self.grid.is_occupied(&start) || self.grid.is_occupied(&goal) {
            return None;
        }
        
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();
        
        open_set.push(PathNode {
            position: start,
            g_score: 0.0,
            f_score: start.distance_to(&goal),
        });
        
        g_score.insert(start, 0.0);
        f_score.insert(start, start.distance_to(&goal));
        
        while let Some(current) = open_set.pop() {
            if current.position == goal {
                return Some(self.reconstruct_path(&came_from, current.position));
            }
            
            for neighbor in current.position.neighbors() {
                if self.grid.is_occupied(&neighbor) {
                    continue;
                }
                
                let tentative_g_score = g_score[&current.position] + current.position.distance_to(&neighbor);
                
                if tentative_g_score < g_score.get(&neighbor).copied().unwrap_or(f64::INFINITY) {
                    came_from.insert(neighbor, current.position);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + neighbor.distance_to(&goal));
                    
                    open_set.push(PathNode {
                        position: neighbor,
                        g_score: tentative_g_score,
                        f_score: tentative_g_score + neighbor.distance_to(&goal),
                    });
                }
            }
        }
        
        None
    }
    
    fn reconstruct_path(&self, came_from: &HashMap<GridPosition, GridPosition>, 
                       current: GridPosition) -> Vec<GridPosition> {
        let mut path = vec![current];
        let mut current_pos = current;
        
        while let Some(&prev) = came_from.get(&current_pos) {
            path.push(prev);
            current_pos = prev;
        }
        
        path.reverse();
        path
    }
}

// PID Controller
#[derive(Debug)]
pub struct PIDController {
    kp: f64,
    ki: f64,
    kd: f64,
    integral: f64,
    previous_error: f64,
}

impl PIDController {
    pub fn new(kp: f64, ki: f64, kd: f64) -> Self {
        PIDController {
            kp,
            ki,
            kd,
            integral: 0.0,
            previous_error: 0.0,
        }
    }
    
    pub fn update(&mut self, current_value: f64, setpoint: f64, dt: f64) -> f64 {
        let error = setpoint - current_value;
        
        // Proportional term
        let p_term = self.kp * error;
        
        // Integral term
        self.integral += error * dt;
        let i_term = self.ki * self.integral;
        
        // Derivative term
        let derivative = (error - self.previous_error) / dt;
        let d_term = self.kd * derivative;
        
        self.previous_error = error;
        
        p_term + i_term + d_term
    }
    
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.previous_error = 0.0;
    }
}

// Motion controller
pub struct MotionController {
    current_state: RobotState,
    target_state: Option<RobotState>,
    pid_x: PIDController,
    pid_y: PIDController,
    pid_theta: PIDController,
}

impl MotionController {
    pub fn new() -> Self {
        MotionController {
            current_state: RobotState::new(),
            target_state: None,
            pid_x: PIDController::new(1.0, 0.1, 0.01),
            pid_y: PIDController::new(1.0, 0.1, 0.01),
            pid_theta: PIDController::new(2.0, 0.2, 0.02),
        }
    }
    
    pub fn set_target(&mut self, target: RobotState) {
        self.target_state = Some(target);
    }
    
    pub fn update(&mut self, dt: f64) -> Vector3 {
        if let Some(target) = &self.target_state {
            let x_command = self.pid_x.update(
                self.current_state.position.x, 
                target.position.x, 
                dt
            );
            
            let y_command = self.pid_y.update(
                self.current_state.position.y, 
                target.position.y, 
                dt
            );
            
            let theta_command = self.pid_theta.update(
                self.current_state.orientation, 
                target.orientation, 
                dt
            );
            
            Vector3::new(x_command, y_command, theta_command)
        } else {
            Vector3::zeros()
        }
    }
    
    pub fn update_state(&mut self, new_state: RobotState) {
        self.current_state = new_state;
    }
}

// Obstacle
#[derive(Debug, Clone)]
pub struct Obstacle {
    pub position: Vector3,
    pub radius: f64,
}

impl Obstacle {
    pub fn new(position: Vector3, radius: f64) -> Self {
        Obstacle { position, radius }
    }
    
    pub fn is_colliding(&self, point: &Vector3) -> bool {
        (point - &self.position).norm() < self.radius
    }
}

// Simple robot
pub struct SimpleRobot {
    state: RobotState,
    motion_controller: MotionController,
    lidar: LidarSensor,
    imu: IMUSensor,
    obstacles: Vec<Obstacle>,
}

impl SimpleRobot {
    pub fn new() -> Self {
        SimpleRobot {
            state: RobotState::new(),
            motion_controller: MotionController::new(),
            lidar: LidarSensor::new(std::f64::consts::PI, 360),
            imu: IMUSensor::new(),
            obstacles: Vec::new(),
        }
    }
    
    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }
    
    pub fn navigate_to(&mut self, target: Vector3) -> Result<(), Box<dyn std::error::Error>> {
        let target_state = RobotState {
            position: target,
            velocity: Vector3::zeros(),
            orientation: 0.0,
            timestamp: self.state.timestamp,
        };
        
        self.motion_controller.set_target(target_state);
        
        // Simulate navigation
        for i in 0..100 {
            let dt = 0.1;
            let control = self.motion_controller.update(dt);
            
            // Update robot state
            self.state.position.x += control.x * dt;
            self.state.position.y += control.y * dt;
            self.state.orientation += control.z * dt;
            self.state.timestamp += dt;
            
            self.motion_controller.update_state(self.state.clone());
            
            // Check for obstacles
            for obstacle in &self.obstacles {
                if obstacle.is_colliding(&self.state.position) {
                    println!("Collision detected! Stopping navigation.");
                    return Ok(());
                }
            }
            
            // Check if reached target
            let distance = (target - self.state.position).norm();
            if distance < 0.1 {
                println!("Target reached!");
                break;
            }
        }
        
        Ok(())
    }
    
    pub fn sense(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let lidar_scan = self.lidar.read()?;
        let imu_data = self.imu.read()?;
        
        println!("LiDAR scan: {} measurements", lidar_scan.ranges.len());
        println!("IMU acceleration: ({:.2}, {:.2}, {:.2})", 
                imu_data.acceleration.x, imu_data.acceleration.y, imu_data.acceleration.z);
        
        Ok(())
    }
    
    pub fn get_state(&self) -> &RobotState {
        &self.state
    }
}

// Main demonstration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ROBOTICS DEMONSTRATIONS ===\n");
    
    // Create a grid for path planning
    let mut grid = OccupancyGrid::new(20, 20, 0.5);
    
    // Add some obstacles
    grid.set_occupied(GridPosition::new(5, 5), true);
    grid.set_occupied(GridPosition::new(6, 5), true);
    grid.set_occupied(GridPosition::new(7, 5), true);
    grid.set_occupied(GridPosition::new(5, 6), true);
    grid.set_occupied(GridPosition::new(5, 7), true);
    
    // Create path planner
    let planner = PathPlanner::new(grid);
    
    // Find path from start to goal
    let start = GridPosition::new(2, 2);
    let goal = GridPosition::new(18, 18);
    
    if let Some(path) = planner.find_path(start, goal) {
        println!("Found path with {} waypoints:", path.len());
        for (i, waypoint) in path.iter().take(5).enumerate() {
            let world_pos = planner.grid.grid_to_world(waypoint);
            println!("  Waypoint {}: ({:.1}, {:.1})", i + 1, world_pos.x, world_pos.y);
        }
        if path.len() > 5 {
            println!("  ... and {} more waypoints", path.len() - 5);
        }
    } else {
        println!("No path found!");
    }
    
    // Create a robot
    let mut robot = SimpleRobot::new();
    
    // Add obstacles to robot
    robot.add_obstacle(Obstacle::new(Vector3::new(2.5, 2.5, 0.0), 0.3));
    robot.add_obstacle(Obstacle::new(Vector3::new(3.5, 3.5, 0.0), 0.3));
    
    // Test sensors
    println!("\n=== SENSOR TESTING ===");
    robot.sense()?;
    
    // Test navigation
    println!("\n=== NAVIGATION TESTING ===");
    let target = Vector3::new(1.0, 1.0, 0.0);
    println!("Navigating to target: ({:.1}, {:.1})", target.x, target.y);
    robot.navigate_to(target)?;
    
    println!("Final robot state: ({:.2}, {:.2})", 
            robot.get_state().position.x, robot.get_state().position.y);
    
    // Test PID controller
    println!("\n=== PID CONTROLLER TESTING ===");
    let mut pid = PIDController::new(1.0, 0.1, 0.01);
    
    let setpoint = 10.0;
    let mut current_value = 0.0;
    
    for i in 0..20 {
        let control = pid.update(current_value, setpoint, 0.1);
        current_value += control * 0.1;
        println!("Step {}: value = {:.3}, control = {:.3}", i + 1, current_value, control);
        
        if (current_value - setpoint).abs() < 0.01 {
            println!("Setpoint reached!");
            break;
        }
    }
    
    println!("\n=== ROBOTICS DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- 3D vector mathematics");
    println!("- Robot state representation");
    println!("- Sensor abstraction (LiDAR, IMU)");
    println!("- Path planning with A* algorithm");
    println!("- PID control for motion");
    println!("- Obstacle detection and avoidance");
    println!("- Basic robot navigation");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vector_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
        
        let diff = v2 - v1;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);
        
        let scaled = v1 * 2.0;
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
    }
    
    #[test]
    fn test_grid_operations() {
        let pos1 = GridPosition::new(0, 0);
        let pos2 = GridPosition::new(3, 4);
        
        let distance = pos1.distance_to(&pos2);
        assert!((distance - 5.0).abs() < 0.001);
        
        let neighbors = pos1.neighbors();
        assert_eq!(neighbors.len(), 4);
    }
    
    #[test]
    fn test_occupancy_grid() {
        let grid = OccupancyGrid::new(10, 10, 1.0);
        
        let world_pos = Vector3::new(2.5, 3.5, 0.0);
        let grid_pos = grid.world_to_grid(&world_pos);
        assert_eq!(grid_pos.x, 3);
        assert_eq!(grid_pos.y, 4);
        
        let back_to_world = grid.grid_to_world(&grid_pos);
        assert!((back_to_world.x - 3.0).abs() < 0.001);
        assert!((back_to_world.y - 4.0).abs() < 0.001);
    }
    
    #[test]
    fn test_pid_controller() {
        let mut pid = PIDController::new(1.0, 0.0, 0.0);
        
        let control = pid.update(0.0, 10.0, 1.0);
        assert_eq!(control, 10.0);
        
        let control = pid.update(5.0, 10.0, 1.0);
        assert_eq!(control, 5.0);
    }
    
    #[test]
    fn test_obstacle() {
        let obstacle = Obstacle::new(Vector3::new(0.0, 0.0, 0.0), 1.0);
        
        assert!(obstacle.is_colliding(&Vector3::new(0.5, 0.5, 0.0)));
        assert!(!obstacle.is_colliding(&Vector3::new(2.0, 2.0, 0.0)));
    }
}
