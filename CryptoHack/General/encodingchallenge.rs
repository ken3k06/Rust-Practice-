use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use serde_json::Value;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use num_bigint::BigUint;
use std::str;

fn rot13(s: &str) -> String {
    s.chars()
     .map(|c| match c {
         'a'..='z' => (((c as u8 - b'a') + 13) % 26 + b'a') as char,
         'A'..='Z' => (((c as u8 - b'A') + 13) % 26 + b'A') as char,
         other => other,
     })
     .collect()
}

fn decode_bigint(s: &str) -> Option<String> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    match BigUint::parse_bytes(s.as_bytes(), 16) {
        Some(n) => {
            let bytes = n.to_bytes_be();
            match String::from_utf8(bytes) {
                Ok(st) => Some(st),
                Err(_) => None,
            }
        }
        None => None,
    }
}

fn main() -> std::io::Result<()> {
    const HOST: &str = "socket.cryptohack.org";
    const PORT: u16 = 13377;
    let addr = format!("{}:{}", HOST, PORT);

    let mut stream = TcpStream::connect(&addr)?;
    println!("Connected to {}", addr);

    let mut reader = BufReader::new(stream.try_clone()?);

    let mut line = String::new();
    loop {
        line.clear();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            println!("Server closed connection");
            break;
        }

        if line.trim().is_empty() {
            continue;
        }

        println!("<- {}", line.trim_end());

        let v: Value = match serde_json::from_str(line.trim_end()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
                continue;
            }
        };

        if let Some(flag) = v.get("flag") {
            println!("FLAG: {}", flag);
            break;
        }

        let typ = v.get("type").and_then(|t| t.as_str());
        let encoded = v.get("encoded");

        let decoded_string_opt: Option<String> = match typ {
            Some("base64") => {
                if let Some(s) = encoded.and_then(|e| e.as_str()) {
                    match STANDARD.decode(s) {
                        Ok(bytes) => String::from_utf8(bytes).ok(),
                        Err(_) => None,
                    }
                } else { None }
            }
            Some("hex") => {
                if let Some(s) = encoded.and_then(|e| e.as_str()) {
                    match hex::decode(s) {
                        Ok(bytes) => String::from_utf8(bytes).ok(),
                        Err(_) => None,
                    }
                } else { None }
            }
            Some("rot13") => {
                if let Some(s) = encoded.and_then(|e| e.as_str()) {
                    Some(rot13(s))
                } else { None }
            }
            Some("bigint") => {
                if let Some(s) = encoded.and_then(|e| e.as_str()) {
                    decode_bigint(s)
                } else { None }
            }
            Some("utf-8") => {
                if let Some(arr) = encoded.and_then(|e| e.as_array()) {
                    let mut bytes = Vec::with_capacity(arr.len());
                    let mut ok = true;
                    for v in arr {
                        if let Some(n) = v.as_u64() {
                            if n <= 255 {
                                bytes.push(n as u8);
                            } else { ok = false; break; }
                        } else { ok = false; break; }
                    }
                    if ok { String::from_utf8(bytes).ok() } else { None }
                } else { None }
            }
            _ => {
                eprintln!("Unknown/unsupported type: {:?}", typ);
                None
            }
        };

        if let Some(decoded) = decoded_string_opt {
            let reply = serde_json::json!({ "decoded": decoded });
            let mut s = reply.to_string();
            s.push('\n'); 
            stream.write_all(s.as_bytes())?;
            println!("-> {}", s.trim_end());
        } else {
            eprintln!("Failed to decode payload for type {:?}", typ);
            let reply = serde_json::json!({ "decoded": "" });
            let mut s = reply.to_string();
            s.push('\n');
            stream.write_all(s.as_bytes())?;
            println!("-> (sent empty decoded)");
        }
    }

    Ok(())
}
