use std::net::UdpSocket;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};

// Function to send raw JSON string to WIZ LED
fn send_wiz_command(bulb_ip: &str, json_payload: &str) -> std::io::Result<()> {
    // 1. Create a UDP socket on any available local port 
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 2. Prepare the destination address (bulb IP + Wiz Port 38899)
    let destination = format!("{}:38899", bulb_ip);

    // 3. Send the JSON bytes
    socket.send_to(json_payload.as_bytes(), destination)?;

    println!("Sent command to {}: {}", bulb_ip, json_payload);
    Ok(())
}

fn color_json(r: u8, g: u8, b: u8) -> String {
    format!(
        r#"{{"method":"setPilot","params":{{"r":{},"g":{},"b":{}}}}}"#,
        r, g, b
    )
}

fn off_json() -> String {
    r#"{"method":"setPilot","params":{"state":false}}"#.to_string()
}

#[tokio::main]
async fn main() {
    let bulb_ip = "10.226.209.74"; // IP — real LED IP goes here later
    // --- MQTT SETUP ---
    // Create connection settings: who am I + where is the broker
    let mut mqtt_opts = MqttOptions::new("wiz-controller", "localhost", 1883);
    mqtt_opts.set_keep_alive(std::time::Duration::from_secs(30));
    // Create client + event loop
    let (client, mut eventloop) = AsyncClient::new(mqtt_opts, 64);
    // Subscribe to the topic we listen on
    client.subscribe("led/color", QoS::AtMostOnce).await.unwrap();
    println!("✅ Connected to MQTT. Listening on 'led/color'...");
    // --- MAIN LOOP: wait for messages forever ---
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::Publish(msg))) => {
                // A message arrived! Convert bytes → text
                let color = String::from_utf8_lossy(&msg.payload).to_string();
                println!("📨 Received: '{}'", color);
                // Match the color text → send correct WiZ command
                let payload = match color.as_str() {
                    "red"   => color_json(255, 0, 0),
                    "green" => color_json(0, 255, 0),
                    "blue"  => color_json(0, 0, 255),
                    "yellow" => color_json(255, 255, 0),
                    "cyan"  => color_json(0, 255, 255),
                    "pink"  => color_json(255, 105, 180),
                    "warm"  => color_json(255, 200, 125), // warm white
                    "cool"  => color_json(180, 220, 255), // cool white
                    "white" => color_json(255, 255, 255), // pure white
                    "off"   => off_json(),
                    _       => {
                        println!("❓ Unknown color: {}", color);
                        continue; // skip unknown commands
                    }
                };
                // Send the UDP command to WiZ LED
                match send_wiz_command(bulb_ip, &payload) {
                    Ok(()) => println!("💡 LED updated!"),
                    Err(e) => println!("❌ UDP error: {}", e),
                }
            }
            Ok(_) => {} // ignore other MQTT events (connect ack, ping, etc.)
            Err(e) => {
                println!("❌ MQTT error: {}", e);
                break;
            }
        }
    }
}