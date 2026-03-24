use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::time::Duration;

const LIVE_LINK_ADDR: &str = "127.0.0.1:18080";
const AUTH_TOKEN: &str = "houdini_ramen_secret_2026";

/// Sends the generated Python script to the Houdini Live-Link server.
pub fn send_to_houdini(script: &str) {
    println!("🍜 Houdini Ramen: Sending script via Live-Link...");

    let target = LIVE_LINK_ADDR.parse().unwrap();
    match TcpStream::connect_timeout(&target, Duration::from_secs(2)) {
        Ok(mut stream) => {
            let payload = format!("AUTH:{}\n{}", AUTH_TOKEN, script);

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
