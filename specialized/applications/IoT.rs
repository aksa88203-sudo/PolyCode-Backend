// iot.rs
// IoT implementation examples in Rust

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: u64,
}

impl SensorReading {
    pub fn new(sensor_id: String, value: f64, unit: String) -> Self {
        SensorReading {
            sensor_id,
            value,
            unit,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

// Generic sensor trait
pub trait Sensor {
    type Error;
    
    fn read(&mut self) -> Result<SensorReading, Self::Error>;
    fn calibrate(&mut self) -> Result<(), Self::Error>;
}

// Temperature sensor implementation
#[derive(Debug)]
pub struct TemperatureSensor {
    sensor_id: String,
    calibration_offset: f32,
    last_reading: Option<SensorReading>,
}

impl TemperatureSensor {
    pub fn new(sensor_id: String) -> Self {
        TemperatureSensor {
            sensor_id,
            calibration_offset: 0.0,
            last_reading: None,
        }
    }
    
    pub fn set_calibration_offset(&mut self, offset: f32) {
        self.calibration_offset = offset;
    }
    
    fn simulate_reading(&self) -> f32 {
        // Simulate temperature between 15-25°C
        20.0 + (rand::random::<f32>() - 0.5) * 10.0 + self.calibration_offset
    }
}

impl Sensor for TemperatureSensor {
    type Error = String;
    
    fn read(&mut self) -> Result<SensorReading, Self::Error> {
        let temperature = self.simulate_reading();
        let reading = SensorReading::new(
            self.sensor_id.clone(),
            temperature as f64,
            "°C".to_string(),
        );
        
        self.last_reading = Some(reading.clone());
        Ok(reading)
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating temperature sensor {}", self.sensor_id);
        self.calibration_offset = 0.0;
        Ok(())
    }
}

// Humidity sensor implementation
#[derive(Debug)]
pub struct HumiditySensor {
    sensor_id: String,
    calibration_factor: f32,
}

impl HumiditySensor {
    pub fn new(sensor_id: String) -> Self {
        HumiditySensor {
            sensor_id,
            calibration_factor: 1.0,
        }
    }
    
    fn simulate_reading(&self) -> f32 {
        // Simulate humidity between 30-80%
        (55.0 + (rand::random::<f32>() - 0.5) * 50.0) * self.calibration_factor
    }
}

impl Sensor for HumiditySensor {
    type Error = String;
    
    fn read(&mut self) -> Result<SensorReading, Self::Error> {
        let humidity = self.simulate_reading();
        let reading = SensorReading::new(
            self.sensor_id.clone(),
            humidity as f64,
            "%".to_string(),
        );
        
        Ok(reading)
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating humidity sensor {}", self.sensor_id);
        self.calibration_factor = 1.0;
        Ok(())
    }
}

// Actuator trait
pub trait Actuator {
    type Error;
    
    fn execute(&mut self, command: &str, params: &serde_json::Value) -> Result<serde_json::Value, Self::Error>;
    fn get_status(&self) -> ActuatorStatus;
}

#[derive(Debug, Clone)]
pub struct ActuatorStatus {
    pub actuator_id: String,
    pub state: String,
    pub last_updated: u64,
}

// LED actuator
#[derive(Debug)]
pub struct LEDActuator {
    actuator_id: String,
    is_on: bool,
    brightness: u8,
}

impl LEDActuator {
    pub fn new(actuator_id: String) -> Self {
        LEDActuator {
            actuator_id,
            is_on: false,
            brightness: 0,
        }
    }
}

impl Actuator for LEDActuator {
    type Error = String;
    
    fn execute(&mut self, command: &str, params: &serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        match command {
            "turn_on" => {
                self.is_on = true;
                if let Some(brightness) = params.get("brightness").and_then(|v| v.as_u64()) {
                    self.brightness = brightness as u8;
                } else {
                    self.brightness = 255;
                }
                println!("LED {} turned on with brightness {}", self.actuator_id, self.brightness);
            },
            "turn_off" => {
                self.is_on = false;
                self.brightness = 0;
                println!("LED {} turned off", self.actuator_id);
            },
            "toggle" => {
                self.is_on = !self.is_on;
                self.brightness = if self.is_on { 255 } else { 0 };
                println!("LED {} toggled, now {}", self.actuator_id, if self.is_on { "on" } else { "off" });
            },
            "set_brightness" => {
                if let Some(brightness) = params.get("brightness").and_then(|v| v.as_u64()) {
                    self.brightness = brightness as u8;
                    self.is_on = self.brightness > 0;
                    println!("LED {} brightness set to {}", self.actuator_id, self.brightness);
                } else {
                    return Err("Missing brightness parameter".to_string());
                }
            },
            _ => {
                return Err(format!("Unknown command: {}", command));
            }
        }
        
        Ok(serde_json::json!({
            "actuator_id": self.actuator_id,
            "command": command,
            "status": "success",
            "state": self.get_status().state
        }))
    }
    
    fn get_status(&self) -> ActuatorStatus {
        ActuatorStatus {
            actuator_id: self.actuator_id.clone(),
            state: if self.is_on {
                format!("on (brightness: {})", self.brightness)
            } else {
                "off".to_string()
            },
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

// Motor actuator
#[derive(Debug)]
pub struct MotorActuator {
    actuator_id: String,
    speed: i16, // -1000 to 1000
    direction: String,
}

impl MotorActuator {
    pub fn new(actuator_id: String) -> Self {
        MotorActuator {
            actuator_id,
            speed: 0,
            direction: "stopped".to_string(),
        }
    }
}

impl Actuator for MotorActuator {
    type Error = String;
    
    fn execute(&mut self, command: &str, params: &serde_json::Value) -> Result<serde_json::Value, Self::Error> {
        match command {
            "set_speed" => {
                if let Some(speed) = params.get("speed").and_then(|v| v.as_i64()) {
                    self.speed = speed.clamp(-1000, 1000) as i16;
                    if self.speed > 0 {
                        self.direction = "forward".to_string();
                    } else if self.speed < 0 {
                        self.direction = "backward".to_string();
                    } else {
                        self.direction = "stopped".to_string();
                    }
                    println!("Motor {} speed set to {} ({})", self.actuator_id, self.speed, self.direction);
                } else {
                    return Err("Missing speed parameter".to_string());
                }
            },
            "stop" => {
                self.speed = 0;
                self.direction = "stopped".to_string();
                println!("Motor {} stopped", self.actuator_id);
            },
            _ => {
                return Err(format!("Unknown command: {}", command));
            }
        }
        
        Ok(serde_json::json!({
            "actuator_id": self.actuator_id,
            "command": command,
            "status": "success",
            "speed": self.speed,
            "direction": self.direction
        }))
    }
    
    fn get_status(&self) -> ActuatorStatus {
        ActuatorStatus {
            actuator_id: self.actuator_id.clone(),
            state: format!("speed: {}, direction: {}", self.speed, self.direction),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

// MQTT message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTMessage {
    pub device_id: String,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    SensorData,
    Command,
    Status,
    Alert,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QoS {
    AtMostOnce,    // QoS 0
    AtLeastOnce,   // QoS 1
    ExactlyOnce,   // QoS 2
}

// Simulated MQTT client
pub struct MQTTClient {
    client_id: String,
    broker_address: String,
    port: u16,
    subscriptions: HashMap<String, QoS>,
}

impl MQTTClient {
    pub fn new(client_id: String, broker_address: String, port: u16) -> Self {
        MQTTClient {
            client_id,
            broker_address,
            port,
            subscriptions: HashMap::new(),
        }
    }
    
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting to MQTT broker at {}:{}", self.broker_address, self.port);
        
        // Simulate connection
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        println!("Connected to MQTT broker as {}", self.client_id);
        
        Ok(())
    }
    
    pub async fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Box<dyn std::error::Error>> {
        println!("Subscribing to topic: {} with QoS: {:?}", topic, qos);
        
        self.subscriptions.insert(topic.to_string(), qos);
        
        // Simulate subscription
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        println!("Successfully subscribed to: {}", topic);
        
        Ok(())
    }
    
    pub async fn publish(&mut self, message: IoTMessage) -> Result<(), Box<dyn std::error::Error>> {
        let topic = self.format_topic(&message);
        let payload = serde_json::to_string(&message.payload)?;
        
        println!("Publishing to topic: {}", topic);
        println!("Payload: {}", payload);
        
        // Simulate publishing
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
        
        match message.message_type {
            MessageType::SensorData => {
                println!("Published sensor data");
            },
            MessageType::Command => {
                println!("Published command");
            },
            MessageType::Status => {
                println!("Published status");
            },
            MessageType::Alert => {
                println!("Published alert");
            },
            MessageType::Heartbeat => {
                println!("Published heartbeat");
            },
        }
        
        Ok(())
    }
    
    pub async fn receive_message(&mut self) -> Result<Option<IoTMessage>, Box<dyn std::error::Error>> {
        // Simulate receiving a message
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Simulate a command message
        let simulated_message = IoTMessage {
            device_id: "server".to_string(),
            message_type: MessageType::Command,
            payload: serde_json::json!({
                "command": "set_brightness",
                "params": {
                    "brightness": 128
                }
            }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        println!("Received message from broker");
        Ok(Some(simulated_message))
    }
    
    fn format_topic(&self, message: &IoTMessage) -> String {
        match message.message_type {
            MessageType::SensorData => format!("iot/{}/sensors", message.device_id),
            MessageType::Command => format!("iot/{}/commands", message.device_id),
            MessageType::Status => format!("iot/{}/status", message.device_id),
            MessageType::Alert => format!("iot/{}/alerts", message.device_id),
            MessageType::Heartbeat => format!("iot/{}/heartbeat", message.device_id),
        }
    }
}

// IoT Device
pub struct IoTDevice {
    device_id: String,
    device_type: String,
    sensors: HashMap<String, Box<dyn Sensor<Error = String>>>,
    actuators: HashMap<String, Box<dyn Actuator<Error = String>>>,
    mqtt_client: Option<MQTTClient>,
}

impl IoTDevice {
    pub fn new(device_id: String, device_type: String) -> Self {
        IoTDevice {
            device_id,
            device_type,
            sensors: HashMap::new(),
            actuators: HashMap::new(),
            mqtt_client: None,
        }
    }
    
    pub fn add_sensor<S>(&mut self, sensor_id: String, sensor: S) 
    where 
        S: Sensor<Error = String> + 'static,
    {
        self.sensors.insert(sensor_id, Box::new(sensor));
    }
    
    pub fn add_actuator<A>(&mut self, actuator_id: String, actuator: A) 
    where 
        A: Actuator<Error = String> + 'static,
    {
        self.actuators.insert(actuator_id, Box::new(actuator));
    }
    
    pub fn set_mqtt_client(&mut self, mqtt_client: MQTTClient) {
        self.mqtt_client = Some(mqtt_client);
    }
    
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing IoT device: {}", self.device_id);
        
        // Initialize MQTT client
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            mqtt_client.connect().await?;
            
            // Subscribe to command topic
            mqtt_client.subscribe(&format!("iot/{}/commands", self.device_id), QoS::AtLeastOnce).await?;
        }
        
        // Calibrate all sensors
        for (sensor_id, sensor) in self.sensors.iter_mut() {
            println!("Calibrating sensor: {}", sensor_id);
            sensor.calibrate()?;
        }
        
        println!("Device initialized successfully");
        Ok(())
    }
    
    pub async fn read_all_sensors(&mut self) -> Result<Vec<SensorReading>, Box<dyn std::error::Error>> {
        let mut readings = Vec::new();
        
        for (sensor_id, sensor) in self.sensors.iter_mut() {
            match sensor.read() {
                Ok(reading) => {
                    readings.push(reading);
                    println!("Sensor {} reading: {:.2} {}", sensor_id, reading.value, reading.unit);
                },
                Err(e) => {
                    eprintln!("Error reading sensor {}: {}", sensor_id, e);
                }
            }
        }
        
        Ok(readings)
    }
    
    pub async fn send_sensor_data(&mut self, readings: Vec<SensorReading>) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            for reading in readings {
                let message = IoTMessage {
                    device_id: self.device_id.clone(),
                    message_type: MessageType::SensorData,
                    payload: serde_json::json!({
                        "sensor_id": reading.sensor_id,
                        "value": reading.value,
                        "unit": reading.unit,
                        "timestamp": reading.timestamp
                    }),
                    timestamp: reading.timestamp,
                };
                
                mqtt_client.publish(message).await?;
            }
        }
        
        Ok(())
    }
    
    pub async fn process_commands(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            while let Some(message) = mqtt_client.receive_message().await? {
                self.handle_command(message).await?;
            }
        }
        
        Ok(())
    }
    
    async fn handle_command(&mut self, message: IoTMessage) -> Result<(), Box<dyn std::error::Error>> {
        match message.message_type {
            MessageType::Command => {
                if let Some(command) = message.payload.get("command") {
                    if let Some(command_str) = command.as_str() {
                        let params = message.payload.get("params").unwrap_or(&serde_json::Value::Null);
                        
                        println!("Executing command: {}", command_str);
                        
                        // Execute command on all actuators
                        for (actuator_id, actuator) in self.actuators.iter_mut() {
                            if let Ok(result) = actuator.execute(command_str, params) {
                                println!("Actuator {} executed command successfully", actuator_id);
                                
                                // Send response
                                self.send_command_response(&command_str, &result).await?;
                            }
                        }
                    }
                }
            },
            _ => {
                println!("Received non-command message type: {:?}", message.message_type);
            }
        }
        
        Ok(())
    }
    
    async fn send_command_response(&mut self, command: &str, result: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            let response = IoTMessage {
                device_id: self.device_id.clone(),
                message_type: MessageType::Status,
                payload: serde_json::json!({
                    "command": command,
                    "result": result,
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                }),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            mqtt_client.publish(response).await?;
        }
        
        Ok(())
    }
    
    pub async fn run_main_loop(&mut self, iterations: usize) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting main loop for device: {} ({} iterations)", self.device_id, iterations);
        
        for i in 0..iterations {
            println!("=== Loop iteration {} ===", i + 1);
            
            // Read sensors
            let readings = self.read_all_sensors().await?;
            
            // Send sensor data
            self.send_sensor_data(readings).await?;
            
            // Process commands
            self.process_commands().await?;
            
            // Send heartbeat
            self.send_heartbeat().await?;
            
            // Sleep for a short period
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        
        Ok(())
    }
    
    async fn send_heartbeat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            let heartbeat = IoTMessage {
                device_id: self.device_id.clone(),
                message_type: MessageType::Heartbeat,
                payload: serde_json::json!({
                    "status": "online",
                    "uptime": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    "memory_usage": 45.2,
                    "cpu_usage": 12.8
                }),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            
            mqtt_client.publish(heartbeat).await?;
        }
        
        Ok(())
    }
}

// Main demonstration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== IOT DEMONSTRATIONS ===\n");
    
    // Create sensors
    let mut temp_sensor = TemperatureSensor::new("temp_001".to_string());
    let mut humidity_sensor = HumiditySensor::new("humid_001".to_string());
    
    // Test sensors
    println!("=== SENSOR TESTING ===");
    let temp_reading = temp_sensor.read()?;
    println!("Temperature reading: {:.2} {}", temp_reading.value, temp_reading.unit);
    
    let humidity_reading = humidity_sensor.read()?;
    println!("Humidity reading: {:.2} {}", humidity_reading.value, humidity_reading.unit);
    
    // Create actuators
    let led = LEDActuator::new("led_001".to_string());
    let motor = MotorActuator::new("motor_001".to_string());
    
    // Test actuators
    println!("\n=== ACTUATOR TESTING ===");
    let mut led_clone = led;
    let result = led_clone.execute("turn_on", &serde_json::json!({"brightness": 200}))?;
    println!("LED result: {}", result);
    
    let mut motor_clone = motor;
    let result = motor_clone.execute("set_speed", &serde_json::json!({"speed": 500}))?;
    println!("Motor result: {}", result);
    
    // Create IoT device
    println!("\n=== IOT DEVICE TESTING ===");
    let mut device = IoTDevice::new("device_001".to_string(), "environmental_monitor".to_string());
    
    // Add sensors to device
    device.add_sensor("temp_001".to_string(), temp_sensor);
    device.add_sensor("humid_001".to_string(), humidity_sensor);
    
    // Add actuators to device
    device.add_actuator("led_001".to_string(), led_clone);
    device.add_actuator("motor_001".to_string(), motor_clone);
    
    // Set up MQTT client
    let mut mqtt_client = MQTTClient::new("device_001".to_string(), "localhost".to_string(), 1883);
    device.set_mqtt_client(mqtt_client);
    
    // Initialize device
    device.initialize().await?;
    
    // Run main loop
    device.run_main_loop(5).await?;
    
    println!("\n=== IOT DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Sensor abstraction and implementation");
    println!("- Actuator control and command execution");
    println!("- MQTT communication protocol");
    println!("- IoT device management");
    println!("- Real-time sensor data collection");
    println!("- Command processing and response");
    println!("- Heartbeat and status reporting");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_temperature_sensor() {
        let mut sensor = TemperatureSensor::new("test_temp".to_string());
        
        let reading = sensor.read().unwrap();
        assert_eq!(reading.sensor_id, "test_temp");
        assert_eq!(reading.unit, "°C");
        assert!(reading.value >= 15.0 && reading.value <= 25.0);
    }
    
    #[tokio::test]
    async fn test_humidity_sensor() {
        let mut sensor = HumiditySensor::new("test_humid".to_string());
        
        let reading = sensor.read().unwrap();
        assert_eq!(reading.sensor_id, "test_humid");
        assert_eq!(reading.unit, "%");
        assert!(reading.value >= 30.0 && reading.value <= 80.0);
    }
    
    #[tokio::test]
    async fn test_led_actuator() {
        let mut led = LEDActuator::new("test_led".to_string());
        
        let result = led.execute("turn_on", &serde_json::json!({"brightness": 128})).unwrap();
        assert!(result["status"] == "success");
        
        let status = led.get_status();
        assert!(status.state.contains("on"));
    }
    
    #[tokio::test]
    async fn test_motor_actuator() {
        let mut motor = MotorActuator::new("test_motor".to_string());
        
        let result = motor.execute("set_speed", &serde_json::json!({"speed": 750})).unwrap();
        assert!(result["status"] == "success");
        
        let status = motor.get_status();
        assert!(status.state.contains("forward"));
    }
    
    #[tokio::test]
    async fn test_mqtt_client() {
        let mut client = MQTTClient::new("test_client".to_string(), "localhost".to_string(), 1883);
        
        client.connect().await.unwrap();
        client.subscribe("test/topic", QoS::AtLeastOnce).await.unwrap();
        
        let message = IoTMessage {
            device_id: "test_device".to_string(),
            message_type: MessageType::SensorData,
            payload: serde_json::json!({"value": 42}),
            timestamp: 12345,
        };
        
        client.publish(message).await.unwrap();
    }
    
    #[tokio::test]
    async fn test_iot_device() {
        let mut device = IoTDevice::new("test_device".to_string(), "test_type".to_string());
        
        let temp_sensor = TemperatureSensor::new("temp_001".to_string());
        device.add_sensor("temp_001".to_string(), temp_sensor);
        
        let led = LEDActuator::new("led_001".to_string());
        device.add_actuator("led_001".to_string(), led);
        
        device.initialize().await.unwrap();
        
        let readings = device.read_all_sensors().await.unwrap();
        assert!(!readings.is_empty());
        assert_eq!(readings[0].sensor_id, "temp_001");
    }
}
