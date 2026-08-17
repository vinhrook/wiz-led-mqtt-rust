Live Hardware Demo

[![Watch the demo].     https://youtu.be/ikOKeTT2Fxw  https://drive.google.com/file/d/1ugoRTyFJCauZB6zu2xwPddAHOznhOMTu/view?usp=sharing
(Color reflection on the laptop in this video) 
*(Click above to watch the LED react in real-time to MQTT terminal commands)* 


# Smart LED Controller & MQTT IoT Bridge (Rust)

A high-performance, asynchronous IoT control service built with **Rust**, **Tokio**, and **rumqttc** to bridge MQTT messaging with local Smart Light UDP datagram protocols.

---

## 🌟 Overview

This project allows local, real-time control of **WiZ Smart Connected Lighting** devices without relying on third-party cloud services or external hubs.

```
[ MQTT Publisher ]  ──►  [ Mosquitto Broker ]  ──►  [ Rust Controller ]  ──►  [ WiZ Smart LED ]
(Terminal / Web UI)          (Port 1883)              (rumqttc + tokio)         (UDP Port 38899)
```

- **Zero-Cloud Latency:** Direct UDP socket communication over the local network.
- **Asynchronous & Non-Blocking:** Powered by Tokio and `rumqttc::AsyncClient`.
- **Pure Memory Safety:** Idiomatic Rust with strict ownership semantics and error propagation.

---

## 🛠️ Tech Stack

- **Language:** Rust (2024 Edition)
- **Async Engine:** [Tokio](https://tokio.rs/)
- **MQTT Client:** [rumqttc](https://crates.io/crates/rumqttc)
- **Networking:** Standard Library `std::net::UdpSocket`
- **Error Handling:** `thiserror`

---

## 🚀 Getting Started

### 1. Prerequisites

- **Rust toolchain:** `rustup` / `cargo`
- **MQTT Broker:** `mosquitto` running locally or on your network
- **Smart LED Bulb:** Connected to the same local WiFi subnet

### 2. Configuration

Set your bulb's local IP in `src/main.rs`:
```rust
let bulb_ip = "10.226.000.000"; // Replace with real life device ip
```

### 3. Build and Run

```bash
# Start the controller service
cargo run
```

---

## 📡 Usage

Publish MQTT commands to topic `led/color` using any MQTT client (such as `mosquitto_pub`):

```bash
# Colors
mosquitto_pub -h localhost -t "led/color" -m "red"
mosquitto_pub -h localhost -t "led/color" -m "green"
mosquitto_pub -h localhost -t "led/color" -m "blue"
mosquitto_pub -h localhost -t "led/color" -m "pink"
mosquitto_pub -h localhost -t "led/color" -m "yellow"
mosquitto_pub -h localhost -t "led/color" -m "cyan"

# White Temperatures
mosquitto_pub -h localhost -t "led/color" -m "warm"
mosquitto_pub -h localhost -t "led/color" -m "cool"
mosquitto_pub -h localhost -t "led/color" -m "white"

# Power Off
mosquitto_pub -h localhost -t "led/color" -m "off"
```

---

## 📄 License

MIT / Apache-2.0
