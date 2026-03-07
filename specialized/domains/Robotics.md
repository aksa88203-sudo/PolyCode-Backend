# Robotics in Rust

## Overview

Rust's performance, memory safety, and real-time capabilities make it an excellent choice for robotics development. This guide covers robot control systems, sensor integration, motion planning, and building autonomous robots in Rust.

---

## Robotics Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `serialport` | Serial communication | Hardware interfaces |
| `rppal` | Raspberry Pi GPIO | GPIO, I2C, SPI |
| `embedded-hal` | Hardware abstraction | Microcontroller support |
| `cortex-m-rt` | ARM Cortex runtime | Embedded systems |
| `stm32f4xx-hal` | STM32 microcontrollers | HAL for STM32 |
| `nalgebra` | Linear algebra | Kinematics, transformations |
| `serde` | Serialization | Data exchange |
| `tokio` | Async runtime | Real-time control |

---

## Robot Architecture

### Robot Components

```rust
use std::collections::HashMap;
use nalgebra::{Vector3, Matrix3, Rotation3, Translation3};

#[derive(Debug, Clone)]
pub struct RobotConfig {
    pub name: String,
    pub max_velocity: f64,
    pub max_acceleration: f64,
    pub wheel_base: f64,
    pub wheel_radius: f64,
    pub sensors: HashMap<String, SensorConfig>,
    pub actuators: HashMap<String, ActuatorConfig>,
}

#[derive(Debug, Clone)]
pub struct SensorConfig {
    pub sensor_type: SensorType,
    pub position: Vector3<f64>,
    pub orientation: Rotation3<f64>,
    pub update_rate: f64,
    pub accuracy: f64,
}

#[derive(Debug, Clone)]
pub struct ActuatorConfig {
    pub actuator_type: ActuatorType,
    pub max_force: f64,
    pub position: Vector3<f64>,
    pub orientation: Rotation3<f64>,
}

#[derive(Debug, Clone)]
pub enum SensorType {
    Lidar,
    Camera,
    IMU,
    Ultrasonic,
    GPS,
    Encoder,
    Touch,
}

#[derive(Debug, Clone)]
pub enum ActuatorType {
    Motor,
    Servo,
    LinearActuator,
    Gripper,
    Wheel,
}

impl RobotConfig {
    pub fn new(name: String) -> Self {
        RobotConfig {
            name,
            max_velocity: 2.0, // m/s
            max_acceleration: 1.0, // m/s²
            wheel_base: 0.3, // meters
            wheel_radius: 0.05, // meters
            sensors: HashMap::new(),
            actuators: HashMap::new(),
        }
    }
    
    pub fn add_sensor(&mut self, name: String, config: SensorConfig) {
        self.sensors.insert(name, config);
    }
    
    pub fn add_actuator(&mut self, name: String, config: ActuatorConfig) {
        self.actuators.insert(name, config);
    }
}
```

### Robot State

```rust
#[derive(Debug, Clone)]
pub struct RobotState {
    pub position: Vector3<f64>,      // x, y, z position
    pub orientation: Rotation3<f64>, // Roll, pitch, yaw
    pub velocity: Vector3<f64>,      // Linear velocity
    pub angular_velocity: Vector3<f64>, // Angular velocity
    pub acceleration: Vector3<f64>,  // Linear acceleration
    pub timestamp: f64,              // Unix timestamp
}

impl RobotState {
    pub fn new() -> Self {
        RobotState {
            position: Vector3::zeros(),
            orientation: Rotation3::identity(),
            velocity: Vector3::zeros(),
            angular_velocity: Vector3::zeros(),
            acceleration: Vector3::zeros(),
            timestamp: 0.0,
        }
    }
    
    pub fn predict_next_state(&self, dt: f64) -> RobotState {
        RobotState {
            position: self.position + self.velocity * dt,
            orientation: self.orientation * 
                Rotation3::from_scaled_axis(self.angular_velocity * dt),
            velocity: self.velocity + self.acceleration * dt,
            angular_velocity: self.angular_velocity,
            acceleration: self.acceleration,
            timestamp: self.timestamp + dt,
        }
    }
    
    pub fn distance_to(&self, other: &RobotState) -> f64 {
        (self.position - other.position).norm()
    }
    
    pub fn heading_to(&self, target: &Vector3<f64>) -> f64 {
        let direction = target - self.position;
        direction[1].atan2(direction[0])
    }
}
```

---

## Sensor Integration

### Sensor Interface

```rust
pub trait Sensor {
    type Data;
    type Error;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error>;
    fn calibrate(&mut self) -> Result<(), Self::Error>;
    fn get_config(&self) -> &SensorConfig;
}

// Lidar sensor implementation
#[derive(Debug)]
pub struct LidarSensor {
    config: SensorConfig,
    angle_range: f64,        // Total angle range in radians
    angle_step: f64,         // Angle step in radians
    max_range: f64,          // Maximum range in meters
    resolution: usize,       // Number of measurements
    last_scan: Option<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct LidarScan {
    pub ranges: Vec<f64>,
    pub angles: Vec<f64>,
    pub timestamp: f64,
}

impl LidarSensor {
    pub fn new(config: SensorConfig, angle_range: f64, resolution: usize) -> Self {
        let angle_step = angle_range / resolution as f64;
        
        LidarSensor {
            config,
            angle_range,
            angle_step,
            max_range: 10.0,
            resolution,
            last_scan: None,
        }
    }
    
    fn generate_angles(&self) -> Vec<f64> {
        let start_angle = -self.angle_range / 2.0;
        (0..self.resolution)
            .map(|i| start_angle + i as f64 * self.angle_step)
            .collect()
    }
    
    fn simulate_scan(&self, robot_state: &RobotState, obstacles: &[Obstacle]) -> Vec<f64> {
        let angles = self.generate_angles();
        let mut ranges = Vec::with_capacity(angles.len());
        
        for &angle in &angles {
            let ray_direction = Vector3::new(
                robot_state.orientation * Vector3::x() * angle.cos() - 
                robot_state.orientation * Vector3::y() * angle.sin(),
                robot_state.orientation * Vector3::x() * angle.sin() + 
                robot_state.orientation * Vector3::y() * angle.cos(),
                0.0,
            );
            
            let mut min_distance = self.max_range;
            
            for obstacle in obstacles {
                let distance = obstacle.ray_intersection(
                    &robot_state.position,
                    &ray_direction,
                    self.max_range,
                );
                
                if distance < min_distance {
                    min_distance = distance;
                }
            }
            
            ranges.push(min_distance);
        }
        
        ranges
    }
}

impl Sensor for LidarSensor {
    type Data = LidarScan;
    type Error = String;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error> {
        // In a real implementation, this would read from actual hardware
        // For simulation, we'll generate sample data
        let ranges = vec![5.0; self.resolution]; // Simulated ranges
        let angles = self.generate_angles();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        let scan = LidarScan {
            ranges,
            angles,
            timestamp,
        };
        
        self.last_scan = Some(scan.ranges.clone());
        Ok(scan)
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating LiDAR sensor...");
        // Simulate calibration process
        Ok(())
    }
    
    fn get_config(&self) -> &SensorConfig {
        &self.config
    }
}

// IMU sensor implementation
#[derive(Debug)]
pub struct IMUSensor {
    config: SensorConfig,
    bias: Vector3<f64>,
    noise_level: f64,
}

#[derive(Debug, Clone)]
pub struct IMUData {
    pub acceleration: Vector3<f64>,
    pub angular_velocity: Vector3<f64>,
    pub magnetic_field: Vector3<f64>,
    pub timestamp: f64,
}

impl IMUSensor {
    pub fn new(config: SensorConfig) -> Self {
        IMUSensor {
            config,
            bias: Vector3::new(0.01, 0.01, 0.01), // Small bias
            noise_level: 0.001,
        }
    }
    
    fn simulate_reading(&self, true_acceleration: &Vector3<f64>, 
                       true_angular_velocity: &Vector3<f64>) -> IMUData {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        
        // Add noise and bias
        let noise = Vector3::new(
            (rand::random::<f64>() - 0.5) * self.noise_level,
            (rand::random::<f64>() - 0.5) * self.noise_level,
            (rand::random::<f64>() - 0.5) * self.noise_level,
        );
        
        IMUData {
            acceleration: true_acceleration + self.bias + noise,
            angular_velocity: true_angular_velocity + self.bias + noise,
            magnetic_field: Vector3::new(0.2, 0.1, 0.4), // Simulated magnetic field
            timestamp,
        }
    }
}

impl Sensor for IMUSensor {
    type Data = IMUData;
    type Error = String;
    
    fn read(&mut self) -> Result<Self::Data, Self::Error> {
        // Simulate IMU reading
        let acceleration = Vector3::new(0.0, 0.0, -9.81); // Gravity
        let angular_velocity = Vector3::new(0.1, 0.0, 0.0); // Small rotation
        
        Ok(self.simulate_reading(&acceleration, &angular_velocity))
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating IMU sensor...");
        // Calibration would involve measuring bias
        self.bias = Vector3::zeros();
        Ok(())
    }
    
    fn get_config(&self) -> &SensorConfig {
        &self.config
    }
}
```

### Sensor Fusion

```rust
pub struct SensorFusion {
    lidar: LidarSensor,
    imu: IMUSensor,
    kalman_filter: KalmanFilter,
}

impl SensorFusion {
    pub fn new(lidar: LidarSensor, imu: IMUSensor) -> Self {
        SensorFusion {
            lidar,
            imu,
            kalman_filter: KalmanFilter::new(),
        }
    }
    
    pub fn update(&mut self, robot_state: &RobotState) -> Result<RobotState, Box<dyn std::error::Error>> {
        // Read sensors
        let lidar_scan = self.lidar.read()?;
        let imu_data = self.imu.read()?;
        
        // Process LiDAR data for obstacle detection
        let obstacles = self.detect_obstacles(&lidar_scan, robot_state);
        
        // Update state with sensor fusion
        let updated_state = self.kalman_filter.update(
            robot_state,
            &imu_data,
            &obstacles,
        );
        
        Ok(updated_state)
    }
    
    fn detect_obstacles(&self, scan: &LidarScan, robot_state: &RobotState) -> Vec<Obstacle> {
        let mut obstacles = Vec::new();
        
        for (i, &range) in scan.ranges.iter().enumerate() {
            if range < 10.0 { // Valid range
                let angle = scan.angles[i];
                let obstacle_x = robot_state.position[0] + range * angle.cos();
                let obstacle_y = robot_state.position[1] + range * angle.sin();
                
                let obstacle = Obstacle {
                    position: Vector3::new(obstacle_x, obstacle_y, 0.0),
                    radius: 0.1, // Assume small obstacles
                };
                
                obstacles.push(obstacle);
            }
        }
        
        obstacles
    }
}

// Kalman filter for sensor fusion
pub struct KalmanFilter {
    state: Vector3<f64>,
    covariance: Matrix3<f64>,
    process_noise: Matrix3<f64>,
    measurement_noise: Matrix3<f64>,
}

impl KalmanFilter {
    pub fn new() -> Self {
        KalmanFilter {
            state: Vector3::zeros(),
            covariance: Matrix3::identity(),
            process_noise: Matrix3::identity() * 0.01,
            measurement_noise: Matrix3::identity() * 0.1,
        }
    }
    
    pub fn update(&mut self, robot_state: &RobotState, imu_data: &IMUData, 
                  obstacles: &[Obstacle]) -> RobotState {
        // Predict step
        let predicted_state = self.predict(robot_state);
        
        // Update step with sensor measurements
        let corrected_state = self.correct(&predicted_state, imu_data, obstacles);
        
        corrected_state
    }
    
    fn predict(&mut self, current_state: &RobotState) -> RobotState {
        // Simple prediction using current velocity
        let dt = 0.1; // 10ms time step
        current_state.predict_next_state(dt)
    }
    
    fn correct(&mut self, predicted_state: &RobotState, imu_data: &IMUData, 
                _obstacles: &[Obstacle]) -> RobotState {
        // Correct state using IMU measurements
        let mut corrected_state = predicted_state.clone();
        
        // Update acceleration from IMU
        corrected_state.acceleration = imu_data.acceleration;
        
        // Update angular velocity from IMU
        corrected_state.angular_velocity = imu_data.angular_velocity;
        
        corrected_state
    }
}
```

---

## Motion Planning

### Path Planning

```rust
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

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
            GridPosition::new(self.x - 1, self.y - 1),
            GridPosition::new(self.x + 1, self.y - 1),
            GridPosition::new(self.x - 1, self.y + 1),
            GridPosition::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Debug)]
pub struct OccupancyGrid {
    grid: HashMap<GridPosition, bool>, // true = occupied
    resolution: f64,                    // meters per grid cell
    origin: GridPosition,
}

impl OccupancyGrid {
    pub fn new(resolution: f64, width: isize, height: isize) -> Self {
        let mut grid = HashMap::new();
        
        for x in 0..width {
            for y in 0..height {
                grid.insert(GridPosition::new(x, y), false);
            }
        }
        
        OccupancyGrid {
            grid,
            resolution,
            origin: GridPosition::new(0, 0),
        }
    }
    
    pub fn set_occupied(&mut self, pos: GridPosition, occupied: bool) {
        self.grid.insert(pos, occupied);
    }
    
    pub fn is_occupied(&self, pos: &GridPosition) -> bool {
        self.grid.get(pos).copied().unwrap_or(true)
    }
    
    pub fn world_to_grid(&self, world_pos: &Vector3<f64>) -> GridPosition {
        GridPosition::new(
            (world_pos[0] / self.resolution).round() as isize,
            (world_pos[1] / self.resolution).round() as isize,
        )
    }
    
    pub fn grid_to_world(&self, grid_pos: &GridPosition) -> Vector3<f64> {
        Vector3::new(
            grid_pos.x as f64 * self.resolution,
            grid_pos.y as f64 * self.resolution,
            0.0,
        )
    }
}

// A* pathfinding algorithm
#[derive(Debug)]
pub struct PathPlanner {
    grid: OccupancyGrid,
}

impl PathPlanner {
    pub fn new(grid: OccupancyGrid) -> Self {
        PathPlanner { grid }
    }
    
    pub fn find_path(&self, start: GridPosition, goal: GridPosition) -> Option<Vec<GridPosition>> {
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

#[derive(Debug, Eq, PartialEq)]
struct PathNode {
    position: GridPosition,
    g_score: f64,
    f_score: f64,
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap behavior
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
```

### Trajectory Planning

```rust
#[derive(Debug, Clone)]
pub struct Trajectory {
    pub waypoints: Vec<Vector3<f64>>,
    pub timestamps: Vec<f64>,
    pub velocities: Vec<Vector3<f64>>,
    pub accelerations: Vec<Vector3<f64>>,
}

impl Trajectory {
    pub fn new() -> Self {
        Trajectory {
            waypoints: Vec::new(),
            timestamps: Vec::new(),
            velocities: Vec::new(),
            accelerations: Vec::new(),
        }
    }
    
    pub fn add_waypoint(&mut self, position: Vector3<f64>, time: f64) {
        self.waypoints.push(position);
        self.timestamps.push(time);
        
        // Calculate velocity and acceleration (simplified)
        if self.waypoints.len() > 1 {
            let dt = self.timestamps[self.timestamps.len() - 1] - self.timestamps[self.timestamps.len() - 2];
            let velocity = (position - self.waypoints[self.waypoints.len() - 2]) / dt;
            self.velocities.push(velocity);
            
            if self.velocities.len() > 1 {
                let acceleration = (velocity - self.velocities[self.velocities.len() - 2]) / dt;
                self.accelerations.push(acceleration);
            }
        }
    }
    
    pub fn interpolate(&self, time: f64) -> Option<Vector3<f64>> {
        if self.waypoints.is_empty() || time < self.timestamps[0] {
            return None;
        }
        
        if time >= *self.timestamps.last().unwrap() {
            return Some(*self.waypoints.last().unwrap());
        }
        
        // Find the segment containing the time
        for i in 0..self.timestamps.len() - 1 {
            if time >= self.timestamps[i] && time <= self.timestamps[i + 1] {
                let t = (time - self.timestamps[i]) / (self.timestamps[i + 1] - self.timestamps[i]);
                let position = self.waypoints[i] * (1.0 - t) + self.waypoints[i + 1] * t;
                return Some(position);
            }
        }
        
        None
    }
}

pub struct TrajectoryPlanner {
    max_velocity: f64,
    max_acceleration: f64,
}

impl TrajectoryPlanner {
    pub fn new(max_velocity: f64, max_acceleration: f64) -> Self {
        TrajectoryPlanner {
            max_velocity,
            max_acceleration,
        }
    }
    
    pub fn generate_trajectory(&self, path: &[GridPosition], grid: &OccupancyGrid) -> Trajectory {
        let mut trajectory = Trajectory::new();
        let mut current_time = 0.0;
        
        for (i, &grid_pos) in path.iter().enumerate() {
            let world_pos = grid.grid_to_world(&grid_pos);
            
            if i == 0 {
                trajectory.add_waypoint(world_pos, current_time);
            } else {
                let prev_world_pos = grid.grid_to_world(&path[i - 1]);
                let distance = (world_pos - prev_world_pos).norm();
                
                // Calculate time based on velocity constraints
                let travel_time = distance / self.max_velocity;
                current_time += travel_time;
                
                trajectory.add_waypoint(world_pos, current_time);
            }
        }
        
        trajectory
    }
    
    pub fn smooth_trajectory(&self, trajectory: &Trajectory) -> Trajectory {
        // Apply smoothing algorithm (e.g., spline interpolation)
        // For simplicity, return the original trajectory
        trajectory.clone()
    }
}
```

---

## Robot Control

### Motion Control

```rust
pub struct MotionController {
    config: RobotConfig,
    current_state: RobotState,
    target_state: Option<RobotState>,
    trajectory: Option<Trajectory>,
    pid_controllers: HashMap<String, PIDController>,
}

impl MotionController {
    pub fn new(config: RobotConfig) -> Self {
        let mut pid_controllers = HashMap::new();
        
        // Create PID controllers for different axes
        pid_controllers.insert("x".to_string(), PIDController::new(1.0, 0.1, 0.01));
        pid_controllers.insert("y".to_string(), PIDController::new(1.0, 0.1, 0.01));
        pid_controllers.insert("theta".to_string(), PIDController::new(2.0, 0.2, 0.02));
        
        MotionController {
            config,
            current_state: RobotState::new(),
            target_state: None,
            trajectory: None,
            pid_controllers,
        }
    }
    
    pub fn set_target(&mut self, target: RobotState) {
        self.target_state = Some(target);
    }
    
    pub fn set_trajectory(&mut self, trajectory: Trajectory) {
        self.trajectory = Some(trajectory);
    }
    
    pub fn update(&mut self, dt: f64) -> Vector3<f64> {
        let target = if let Some(trajectory) = &self.trajectory {
            let current_time = self.current_state.timestamp;
            if let Some(position) = trajectory.interpolate(current_time) {
                RobotState {
                    position,
                    orientation: self.current_state.orientation,
                    velocity: Vector3::zeros(),
                    angular_velocity: Vector3::zeros(),
                    acceleration: Vector3::zeros(),
                    timestamp: current_time,
                }
            } else {
                self.current_state.clone()
            }
        } else if let Some(target) = &self.target_state {
            target.clone()
        } else {
            return Vector3::zeros();
        };
        
        // Calculate control commands using PID controllers
        let x_command = self.pid_controllers["x"].update(
            self.current_state.position[0], 
            target.position[0], 
            dt
        );
        
        let y_command = self.pid_controllers["y"].update(
            self.current_state.position[1], 
            target.position[1], 
            dt
        );
        
        Vector3::new(x_command, y_command, 0.0)
    }
    
    pub fn update_state(&mut self, new_state: RobotState) {
        self.current_state = new_state;
    }
}

#[derive(Debug)]
pub struct PIDController {
    kp: f64,  // Proportional gain
    ki: f64,  // Integral gain
    kd: f64,  // Derivative gain
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
```

---

## Obstacle Avoidance

### Obstacle Representation

```rust
#[derive(Debug, Clone)]
pub struct Obstacle {
    pub position: Vector3<f64>,
    pub radius: f64,
    pub velocity: Option<Vector3<f64>>, // For moving obstacles
}

impl Obstacle {
    pub fn new(position: Vector3<f64>, radius: f64) -> Self {
        Obstacle {
            position,
            radius,
            velocity: None,
        }
    }
    
    pub fn is_colliding(&self, point: &Vector3<f64>) -> bool {
        (point - &self.position).norm() < self.radius
    }
    
    pub fn ray_intersection(&self, origin: &Vector3<f64>, direction: &Vector3<f64>, 
                          max_distance: f64) -> f64 {
        let oc = origin - &self.position;
        let a = direction.dot(direction);
        let b = 2.0 * oc.dot(direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        
        let discriminant = b * b - 4.0 * a * c;
        
        if discriminant < 0.0 {
            return max_distance;
        }
        
        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        
        if t1 > 0.0 && t1 < max_distance {
            t1
        } else if t2 > 0.0 && t2 < max_distance {
            t2
        } else {
            max_distance
        }
    }
    
    pub fn update(&mut self, dt: f64) {
        if let Some(velocity) = self.velocity {
            self.position += velocity * dt;
        }
    }
}

pub struct ObstacleAvoidance {
    obstacles: Vec<Obstacle>,
    safety_margin: f64,
    look_ahead_distance: f64,
}

impl ObstacleAvoidance {
    pub fn new(safety_margin: f64, look_ahead_distance: f64) -> Self {
        ObstacleAvoidance {
            obstacles: Vec::new(),
            safety_margin,
            look_ahead_distance,
        }
    }
    
    pub fn add_obstacle(&mut self, obstacle: Obstacle) {
        self.obstacles.push(obstacle);
    }
    
    pub fn update_obstacles(&mut self, dt: f64) {
        for obstacle in &mut self.obstacles {
            obstacle.update(dt);
        }
    }
    
    pub fn compute_avoidance_force(&self, robot_state: &RobotState) -> Vector3<f64> {
        let mut avoidance_force = Vector3::zeros();
        
        for obstacle in &self.obstacles {
            let distance = (obstacle.position - robot_state.position).norm();
            
            if distance < self.look_ahead_distance {
                let repulsion_magnitude = (1.0 / distance - 1.0 / self.look_ahead_distance).max(0.0);
                let repulsion_direction = (robot_state.position - obstacle.position).normalize();
                
                avoidance_force += repulsion_direction * repulsion_magnitude;
            }
        }
        
        avoidance_force
    }
    
    pub fn is_path_clear(&self, start: &Vector3<f64>, end: &Vector3<f64>) -> bool {
        let direction = (end - start).normalize();
        let distance = (end - start).norm();
        
        for obstacle in &self.obstacles {
            let intersection = obstacle.ray_intersection(start, &direction, distance);
            if intersection < distance {
                return false;
            }
        }
        
        true
    }
}
```

---

## Key Takeaways

- **Robot architecture** requires modular design
- **Sensor integration** needs proper abstraction
- **Sensor fusion** combines multiple sensor inputs
- **Path planning** enables autonomous navigation
- **Motion control** uses feedback controllers
- **Obstacle avoidance** ensures safe operation
- **Real-time constraints** require efficient algorithms

---

## Robotics Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Modular design** | Separate concerns | Use traits and modules |
| **Safety margins** | Account for uncertainties | Add buffer zones |
| **Real-time performance** | Meet timing constraints | Use efficient algorithms |
| **Error handling** | Handle sensor failures | Result types and fallbacks |
| **Simulation testing** | Test before deployment | Use simulation environments |
| **Hardware abstraction** | Support multiple platforms | Use HAL traits |
| **State estimation** | Track robot state accurately | Kalman filtering |
