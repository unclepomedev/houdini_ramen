use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

/// Sends the generated Python script to the Houdini Live-Link server.
pub fn send_to_houdini(script: &str) {
    println!("🍜 Houdini Ramen: Sending script via Live-Link...");

    let token = std::env::var("HOUDINI_RAMEN_TOKEN").unwrap_or_else(|_| {
        eprintln!("⚠️ Warning: HOUDINI_RAMEN_TOKEN not set, using default.");
        "houdini_ramen_secret_2026".to_string()
    });
    let port = std::env::var("HOUDINI_RAMEN_PORT").unwrap_or_else(|_| "18080".to_string());
    let target_addr = format!("127.0.0.1:{}", port);

    let target = match target_addr.parse() {
        Ok(target) => target,
        Err(e) => {
            eprintln!("❌ Invalid HOUDINI_RAMEN_PORT '{}': {}", port, e);
            return;
        }
    };
    match TcpStream::connect_timeout(&target, Duration::from_secs(2)) {
        Ok(mut stream) => {
            let payload = format!("AUTH:{}\n{}", token, script);

            if let Err(e) = stream.write_all(payload.as_bytes()) {
                eprintln!("❌ Failed to transfer the script: {}", e);
                return;
            }
            let _ = stream.shutdown(Shutdown::Write);
            stream.set_read_timeout(Some(Duration::from_secs(10))).ok();

            let mut response = String::new();
            if stream.read_to_string(&mut response).is_ok() {
                match response.trim_end() {
                    "OK" => {
                        println!("✅ Live-Link successful! Transferred the node tree to Houdini!");
                    }
                    msg if msg.starts_with("ERROR") => {
                        eprintln!("❌ Python Execution Failed in Houdini:\n{}", msg);
                    }
                    "" => {
                        eprintln!("❌ Houdini closed the connection without a response.");
                    }
                    msg => {
                        eprintln!("❌ Unexpected response from Houdini:\n{}", msg);
                    }
                }
            } else {
                eprintln!("⚠️ Script sent, but failed to read response from Houdini.");
            }
        }
        Err(e) => {
            eprintln!("❌ Could not connect to Houdini: {}", e);
            eprintln!(
                "💡 Hint: Is the Live-Link server (tools/link_server.py) running in Houdini?"
            );
        }
    }
}
