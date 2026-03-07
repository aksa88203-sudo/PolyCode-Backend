# Internet of Things (IoT) in Rust

## Overview

Rust's memory safety, performance, and low-level capabilities make it ideal for IoT development. This guide covers embedded programming, sensor networks, communication protocols, and building IoT devices in Rust.

---

## IoT Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `embedded-hal` | Hardware abstraction | Cross-platform embedded |
| `cortex-m-rt` | ARM Cortex runtime | Microcontroller support |
| `stm32f4xx-hal` | STM32 HAL | STM32 microcontrollers |
| `esp-idf-hal` | ESP32 support | WiFi/Bluetooth IoT |
| `rppal` | Raspberry Pi GPIO | GPIO, I2C, SPI |
| `tokio` | Async runtime | Network protocols |
| `serde` | Serialization | Data exchange |
| `mqtt` | MQTT client | IoT messaging |

---

## Embedded Programming

### Hardware Abstraction Layer

```rust
use embedded_hal::digital::v2::{InputPin, OutputPin, ToggleableOutputPin};
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::adc::{Channel, OneShot};

// Generic LED interface
pub trait LED {
    type Error;
    
    fn on(&mut self) -> Result<(), Self::Error>;
    fn off(&mut self) -> Result<(), Self::Error>;
    fn toggle(&mut self) -> Result<(), Self::Error>;
    fn is_on(&self) -> bool;
}

// Generic Button interface
pub trait Button {
    type Error;
    
    fn is_pressed(&self) -> Result<bool, Self::Error>;
    fn wait_for_press(&mut self) -> Result<(), Self::Error>;
}

// Generic Sensor interface
pub trait Sensor {
    type Reading;
    type Error;
    
    fn read(&mut self) -> Result<Self::Reading, Self::Error>;
    fn calibrate(&mut self) -> Result<(), Self::Error>;
}

// LED implementation using HAL
pub struct HalLED<PIN> 
where 
    PIN: OutputPin + ToggleableOutputPin,
{
    pin: PIN,
    is_on: bool,
}

impl<PIN> HalLED<PIN> 
where 
    PIN: OutputPin + ToggleableOutputPin,
{
    pub fn new(mut pin: PIN) -> Result<Self, PIN::Error> {
        pin.set_low()?;
        Ok(HalLED {
            pin,
            is_on: false,
        })
    }
}

impl<PIN> LED for HalLED<PIN> 
where 
    PIN: OutputPin + ToggleableOutputPin,
{
    type Error = PIN::Error;
    
    fn on(&mut self) -> Result<(), Self::Error> {
        self.pin.set_high()?;
        self.is_on = true;
        Ok(())
    }
    
    fn off(&mut self) -> Result<(), Self::Error> {
        self.pin.set_low()?;
        self.is_on = false;
        Ok(())
    }
    
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.pin.toggle()?;
        self.is_on = !self.is_on;
        Ok(())
    }
    
    fn is_on(&self) -> bool {
        self.is_on
    }
}

// Button implementation using HAL
pub struct HalButton<PIN> 
where 
    PIN: InputPin,
{
    pin: PIN,
    debounce_delay: u32,
}

impl<PIN> HalButton<PIN> 
where 
    PIN: InputPin,
{
    pub fn new(pin: PIN, debounce_delay: u32) -> Self {
        HalButton {
            pin,
            debounce_delay,
        }
    }
}

impl<PIN> Button for HalButton<PIN> 
where 
    PIN: InputPin,
{
    type Error = PIN::Error;
    
    fn is_pressed(&self) -> Result<bool, Self::Error> {
        self.pin.is_low()
    }
    
    fn wait_for_press(&mut self) -> Result<(), Self::Error> {
        while !self.is_pressed()? {
            // Wait for button press
        }
        
        // Simple debounce
        for _ in 0..self.debounce_delay {
            // Delay implementation would go here
        }
        
        Ok(())
    }
}
```

### Temperature Sensor

```rust
use embedded_hal::adc::{Channel, OneShot};

pub struct TemperatureSensor<ADC, PIN> 
where 
    ADC: OneShot<PIN, u16, u16>,
    PIN: Channel<ADC, ID = u8>,
{
    adc: ADC,
    pin: PIN,
    calibration_offset: f32,
}

#[derive(Debug, Clone)]
pub struct TemperatureReading {
    pub celsius: f32,
    pub fahrenheit: f32,
    pub kelvin: f32,
    pub timestamp: u32,
}

impl<ADC, PIN> TemperatureSensor<ADC, PIN> 
where 
    ADC: OneShot<PIN, u16, u16>,
    PIN: Channel<ADC, ID = u8>,
{
    pub fn new(adc: ADC, pin: PIN) -> Self {
        TemperatureSensor {
            adc,
            pin,
            calibration_offset: 0.0,
        }
    }
    
    pub fn set_calibration_offset(&mut self, offset: f32) {
        self.calibration_offset = offset;
    }
    
    fn adc_to_temperature(&self, adc_value: u16) -> f32 {
        // Convert ADC value to voltage (assuming 3.3V reference and 12-bit ADC)
        let voltage = adc_value as f32 * 3.3 / 4095.0;
        
        // Convert voltage to temperature (for LM35 sensor: 10mV/°C)
        let temperature_c = voltage * 100.0 + self.calibration_offset;
        
        temperature_c
    }
}

impl<ADC, PIN> Sensor for TemperatureSensor<ADC, PIN> 
where 
    ADC: OneShot<PIN, u16, u16>,
    PIN: Channel<ADC, ID = u8>,
{
    type Reading = TemperatureReading;
    type Error = ADC::Error;
    
    fn read(&mut self) -> Result<Self::Reading, Self::Error> {
        let adc_value = self.adc.read(&mut self.pin)?;
        let celsius = self.adc_to_temperature(adc_value);
        
        let reading = TemperatureReading {
            celsius,
            fahrenheit: celsius * 9.0 / 5.0 + 32.0,
            kelvin: celsius + 273.15,
            timestamp: self.get_timestamp(),
        };
        
        Ok(reading)
    }
    
    fn calibrate(&mut self) -> Result<(), Self::Error> {
        println!("Calibrating temperature sensor...");
        // Calibration logic would go here
        self.calibration_offset = 0.0;
        Ok(())
    }
}

impl<ADC, PIN> TemperatureSensor<ADC, PIN> 
where 
    ADC: OneShot<PIN, u16, u16>,
    PIN: Channel<ADC, ID = u8>,
{
    fn get_timestamp(&self) -> u32 {
        // Simple timestamp implementation
        // In real embedded systems, this would use RTC or system timer
        0
    }
}
```

---

## Communication Protocols

### MQTT Client

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTMessage {
    pub device_id: String,
    pub message_type: MessageType,
    pub payload: serde_json::Value,
    pub timestamp: u64,
    pub qos: QoS,
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

pub struct MQTTClient {
    client_id: String,
    broker_address: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    subscriptions: HashMap<String, QoS>,
    last_will: Option<IoTMessage>,
}

impl MQTTClient {
    pub fn new(client_id: String, broker_address: String, port: u16) -> Self {
        MQTTClient {
            client_id,
            broker_address,
            port,
            username: None,
            password: None,
            subscriptions: HashMap::new(),
            last_will: None,
        }
    }
    
    pub fn set_credentials(&mut self, username: String, password: String) {
        self.username = Some(username);
        self.password = Some(password);
    }
    
    pub fn set_last_will(&mut self, message: IoTMessage) {
        self.last_will = Some(message);
    }
    
    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting to MQTT broker at {}:{}", self.broker_address, self.port);
        
        // Simulate MQTT connection
        // In real implementation, use mqtt crate or similar
        
        // Send CONNECT packet
        // Wait for CONNACK
        
        println!("Connected to MQTT broker as {}", self.client_id);
        
        // Send last will if set
        if let Some(ref last_will) = self.last_will {
            println!("Last will message set");
        }
        
        Ok(())
    }
    
    pub async fn subscribe(&mut self, topic: &str, qos: QoS) -> Result<(), Box<dyn std::error::Error>> {
        println!("Subscribing to topic: {} with QoS: {:?}", topic, qos);
        
        // Simulate subscription
        self.subscriptions.insert(topic.to_string(), qos);
        
        // Send SUBSCRIBE packet
        // Wait for SUBACK
        
        println!("Successfully subscribed to: {}", topic);
        Ok(())
    }
    
    pub async fn publish(&mut self, message: IoTMessage) -> Result<(), Box<dyn std::error::Error>> {
        let topic = self.format_topic(&message);
        let payload = serde_json::to_string(&message.payload)?;
        
        println!("Publishing to topic: {}", topic);
        println!("Payload: {}", payload);
        
        // Simulate publishing
        // Send PUBLISH packet
        // Handle QoS levels
        
        match message.qos {
            QoS::AtMostOnce => {
                println!("Published with QoS 0 (Fire and forget)");
            },
            QoS::AtLeastOnce => {
                println!("Published with QoS 1 (At least once delivery)");
                // Wait for PUBACK
            },
            QoS::ExactlyOnce => {
                println!("Published with QoS 2 (Exactly once delivery)");
                // Wait for PUBREC, send PUBREL, wait for PUBCOMP
            },
        }
        
        Ok(())
    }
    
    pub async fn receive_message(&mut self) -> Result<Option<IoTMessage>, Box<dyn std::error::Error>> {
        // Simulate receiving a message
        // In real implementation, this would wait for PUBLISH packet
        
        let simulated_message = IoTMessage {
            device_id: "server".to_string(),
            message_type: MessageType::Command,
            payload: serde_json::json!({
                "command": "toggle_led",
                "params": {
                    "led_id": 1,
                    "state": "on"
                }
            }),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            qos: QoS::AtLeastOnce,
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
    
    pub async fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Disconnecting from MQTT broker");
        
        // Send DISCONNECT packet
        // Clean up connections
        
        println!("Disconnected from MQTT broker");
        Ok(())
    }
}
```

### HTTP REST API

```rust
use serde::{Serialize, Deserialize};
use reqwest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub sensor_id: String,
    pub sensor_type: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceStatus {
    pub device_id: String,
    pub online: bool,
    pub battery_level: Option<f64>,
    pub last_seen: u64,
    pub firmware_version: String,
}

pub struct HTTPClient {
    base_url: String,
    api_key: Option<String>,
    client: reqwest::Client,
}

impl HTTPClient {
    pub fn new(base_url: String) -> Self {
        HTTPClient {
            base_url,
            api_key: None,
            client: reqwest::Client::new(),
        }
    }
    
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
    
    pub async fn send_sensor_data(&self, data: SensorData) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/sensors/data", self.base_url);
        
        let mut request = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&data);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            println!("Sensor data sent successfully");
            Ok(())
        } else {
            Err(format!("Failed to send sensor data: {}", response.status()).into())
        }
    }
    
    pub async fn update_device_status(&self, status: DeviceStatus) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/devices/{}", self.base_url, status.device_id);
        
        let mut request = self.client
            .put(&url)
            .header("Content-Type", "application/json")
            .json(&status);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            println!("Device status updated successfully");
            Ok(())
        } else {
            Err(format!("Failed to update device status: {}", response.status()).into())
        }
    }
    
    pub async fn get_commands(&self, device_id: &str) -> Result<Vec<CommandMessage>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/devices/{}/commands", self.base_url, device_id);
        
        let mut request = self.client.get(&url);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            let commands: Vec<CommandMessage> = response.json().await?;
            println!("Retrieved {} commands", commands.len());
            Ok(commands)
        } else {
            Err(format!("Failed to get commands: {}", response.status()).into())
        }
    }
    
    pub async fn acknowledge_command(&self, command_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/api/commands/{}/acknowledge", self.base_url, command_id);
        
        let mut request = self.client.post(&url);
        
        if let Some(ref api_key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await?;
        
        if response.status().is_success() {
            println!("Command acknowledged successfully");
            Ok(())
        } else {
            Err(format!("Failed to acknowledge command: {}", response.status()).into())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandMessage {
    pub id: String,
    pub device_id: String,
    pub command: String,
    pub parameters: serde_json::Value,
    pub timestamp: u64,
    pub acknowledged: bool,
}
```

---

## Device Management

### IoT Device

```rust
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct IoTDevice {
    device_id: String,
    device_type: String,
    firmware_version: String,
    sensors: HashMap<String, Box<dyn Sensor<Reading = SensorReading>>>,
    actuators: HashMap<String, Box<dyn Actuator>>,
    mqtt_client: Option<MQTTClient>,
    http_client: Option<HTTPClient>,
    status: DeviceStatus,
}

#[derive(Debug, Clone)]
pub struct SensorReading {
    pub sensor_id: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: u64,
}

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

#[derive(Debug, Clone)]
pub struct DeviceStatus {
    pub device_id: String,
    pub online: bool,
    pub battery_level: Option<f64>,
    pub last_seen: u64,
    pub firmware_version: String,
    pub uptime: u64,
    pub memory_usage: f64,
    pub cpu_usage: f64,
}

impl IoTDevice {
    pub fn new(device_id: String, device_type: String, firmware_version: String) -> Self {
        IoTDevice {
            device_id,
            device_type,
            firmware_version,
            sensors: HashMap::new(),
            actuators: HashMap::new(),
            mqtt_client: None,
            http_client: None,
            status: DeviceStatus {
                device_id: device_id.clone(),
                online: false,
                battery_level: None,
                last_seen: 0,
                firmware_version,
                uptime: 0,
                memory_usage: 0.0,
                cpu_usage: 0.0,
            },
        }
    }
    
    pub fn add_sensor<S>(&mut self, sensor_id: String, sensor: S) 
    where 
        S: Sensor<Reading = SensorReading> + 'static,
    {
        self.sensors.insert(sensor_id, Box::new(sensor));
    }
    
    pub fn add_actuator<A>(&mut self, actuator_id: String, actuator: A) 
    where 
        A: Actuator + 'static,
    {
        self.actuators.insert(actuator_id, Box::new(actuator));
    }
    
    pub fn set_mqtt_client(&mut self, mqtt_client: MQTTClient) {
        self.mqtt_client = Some(mqtt_client);
    }
    
    pub fn set_http_client(&mut self, http_client: HTTPClient) {
        self.http_client = Some(http_client);
    }
    
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing IoT device: {}", self.device_id);
        
        // Initialize MQTT client
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            mqtt_client.connect().await?;
            
            // Subscribe to command topic
            mqtt_client.subscribe(&format!("iot/{}/commands", self.device_id), QoS::AtLeastOnce).await?;
            
            // Subscribe to configuration topic
            mqtt_client.subscribe(&format!("iot/{}/config", self.device_id), QoS::AtLeastOnce).await?;
        }
        
        // Calibrate all sensors
        for (sensor_id, sensor) in self.sensors.iter_mut() {
            println!("Calibrating sensor: {}", sensor_id);
            sensor.calibrate()?;
        }
        
        // Set device online
        self.status.online = true;
        self.status.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Send status update
        self.send_status_update().await?;
        
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
        // Send via MQTT
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
                    qos: QoS::AtLeastOnce,
                };
                
                mqtt_client.publish(message).await?;
            }
        }
        
        // Send via HTTP
        if let Some(ref http_client) = self.http_client {
            for reading in readings {
                let sensor_data = SensorData {
                    sensor_id: reading.sensor_id,
                    sensor_type: "generic".to_string(),
                    value: reading.value,
                    unit: reading.unit,
                    timestamp: reading.timestamp,
                };
                
                http_client.send_sensor_data(sensor_data).await?;
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
        
        if let Some(ref http_client) = self.http_client {
            let commands = http_client.get_commands(&self.device_id).await?;
            
            for command in commands {
                if !command.acknowledged {
                    self.handle_http_command(&command, http_client).await?;
                }
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
                        
                        // Execute command on actuators
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
    
    async fn handle_http_command(&mut self, command: &CommandMessage, 
                                http_client: &HTTPClient) -> Result<(), Box<dyn std::error::Error>> {
        println!("Executing HTTP command: {}", command.command);
        
        let params = &command.parameters;
        
        // Execute command on actuators
        for (actuator_id, actuator) in self.actuators.iter_mut() {
            if let Ok(result) = actuator.execute(&command.command, params) {
                println!("Actuator {} executed command successfully", actuator_id);
                
                // Acknowledge command
                http_client.acknowledge_command(&command.id).await?;
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
                    "timestamp": SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                }),
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                qos: QoS::AtLeastOnce,
            };
            
            mqtt_client.publish(response).await?;
        }
        
        Ok(())
    }
    
    async fn send_status_update(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status_message = IoTMessage {
            device_id: self.device_id.clone(),
            message_type: MessageType::Status,
            payload: serde_json::json!({
                "online": self.status.online,
                "battery_level": self.status.battery_level,
                "uptime": self.status.uptime,
                "memory_usage": self.status.memory_usage,
                "cpu_usage": self.status.cpu_usage,
                "firmware_version": self.status.firmware_version
            }),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            qos: QoS::AtLeastOnce,
        };
        
        if let Some(ref mut mqtt_client) = self.mqtt_client {
            mqtt_client.publish(status_message).await?;
        }
        
        Ok(())
    }
    
    pub async fn run_main_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting main loop for device: {}", self.device_id);
        
        loop {
            // Update device status
            self.update_system_status();
            
            // Read sensors
            let readings = self.read_all_sensors().await?;
            
            // Send sensor data
            self.send_sensor_data(readings).await?;
            
            // Process commands
            self.process_commands().await?;
            
            // Send periodic status update
            self.send_status_update().await?;
            
            // Sleep for a short period
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }
    
    fn update_system_status(&mut self) {
        self.status.last_seen = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simulate system metrics
        self.status.uptime += 10; // Assuming 10-second loop
        self.status.memory_usage = (self.status.uptime as f64 * 0.001) % 80.0;
        self.status.cpu_usage = (self.status.uptime as f64 * 0.1) % 100.0;
        
        // Simulate battery drain
        if let Some(ref mut battery) = self.status.battery_level {
            *battery = (*battery - 0.01).max(0.0);
        }
    }
}
```

---

## Key Takeaways

- **Embedded HAL** provides hardware abstraction
- **Communication protocols** enable IoT connectivity
- **Device management** handles sensor/actuator coordination
- **MQTT** is ideal for lightweight IoT messaging
- **HTTP** provides REST API integration
- **Real-time constraints** require efficient code
- **Power management** is critical for battery devices

---

## IoT Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Power efficiency** | Minimize power consumption | Sleep modes, low-power states |
| **Error handling** | Handle network failures gracefully | Retry mechanisms, fallbacks |
| **Security** | Encrypt communications | TLS, authentication |
| **OTA updates** | Enable over-the-air updates | Secure firmware updates |
| **Data validation** | Validate sensor readings | Range checks, calibration |
| **Resource management** | Manage limited memory | Stack/heap optimization |
| **Network resilience** | Handle intermittent connectivity | Buffering, reconnection |
