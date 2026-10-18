use serde::Deserialize;
use std::env;
use std::io::{self, Read, Write};
use std::process::Command;

#[derive(Deserialize)]
struct ExtensionMessage {
    url: String,
}

fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let mut length_buf = [0u8; 4];

    // Native messaging messages begin with a 4-byte length prefix (native endian)
    if stdin.read_exact(&mut length_buf).is_err() {
        return Ok(());
    }

    let length = u32::from_ne_bytes(length_buf) as usize;
    if length == 0 || length > 1024 * 1024 {
        return Ok(());
    }

    let mut message_buf = vec![0u8; length];
    stdin.read_exact(&mut message_buf)?;

    if let Ok(msg) = serde_json::from_slice::<ExtensionMessage>(&message_buf) {
        let current_exe = env::current_exe()?;
        let app_exe = current_exe
            .parent()
            .map(|p| p.join("open-in"))
            .unwrap_or_else(|| "open-in".into());

        // Launch OpenIn with the received URL
        let _ = Command::new(app_exe).arg(&msg.url).spawn();

        // Write a success response back to the extension
        let response = b"{\"status\":\"success\"}";
        let res_len = (response.len() as u32).to_ne_bytes();
        let mut stdout = io::stdout();
        stdout.write_all(&res_len)?;
        stdout.write_all(response)?;
        stdout.flush()?;
    }

    Ok(())
}
