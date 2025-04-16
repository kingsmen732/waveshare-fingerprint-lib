use serialport::SerialPort;
use std::io::{self, Write};
use std::time::Duration;

// Commands
pub const CMD_START: u8 = 0xF5;
pub const CMD_END: u8 = 0xF5;
pub const ACK_SUCCESS: u8 = 0x00;
pub const ACK_FAIL: u8 = 0x01;
pub const ACK_FULL: u8 = 0x04;
pub const ACK_NOUSER: u8 = 0x05;
pub const ACK_USER_OCCUPIED: u8 = 0x06;
pub const ACK_FINGER_OCCUPIED: u8 = 0x07;
pub const ACK_TIMEOUT: u8 = 0x08;

pub fn checksum(packet: &[u8]) -> u8 {
    packet[1] ^ packet[2] ^ packet[3] ^ packet[4] ^ packet[5]
}

pub fn build_command(cmd: u8, p1: u8, p2: u8, p3: u8) -> Vec<u8> {
    let mut packet = vec![CMD_START, cmd, p1, p2, p3, 0, 0, CMD_END];
    packet[6] = checksum(&packet);
    packet
}

pub fn send_command(port: &mut dyn SerialPort, cmd: u8, p1: u8, p2: u8, p3: u8) -> io::Result<[u8; 8]> {
    let cmd_bytes = build_command(cmd, p1, p2, p3);
    port.write_all(&cmd_bytes)?;
    let mut buf = [0u8; 8];
    port.read_exact(&mut buf)?;
    Ok(buf)
}

pub fn parse_ack(resp: &[u8]) -> String {
    if resp.len() != 8 || resp[0] != CMD_START || resp[7] != CMD_END {
        return "❌ Invalid or malformed response".to_string();
    }
    match resp[4] {
        ACK_SUCCESS => "✅ Success".to_string(),
        ACK_FAIL => "❌ Fail".to_string(),
        ACK_FULL => "⚠️ Database Full".to_string(),
        ACK_NOUSER => "❌ No User".to_string(),
        ACK_USER_OCCUPIED => "⚠️ User ID Exists".to_string(),
        ACK_FINGER_OCCUPIED => "⚠️ Fingerprint Exists".to_string(),
        ACK_TIMEOUT => "⌛ Timeout".to_string(),
        other => format!("❓ Unknown response: {:#X}", other),
    }
}

pub fn enroll(port: &mut dyn SerialPort, user_id: u16, permission: u8) {
    let (uid_h, uid_l) = ((user_id >> 8) as u8, user_id as u8);
    let cmds = [0x01, 0x02, 0x03];

    for (i, &cmd) in cmds.iter().enumerate() {
        println!(">> Step {}: Place your finger...", i + 1);
        match send_command(port, cmd, uid_h, uid_l, permission) {
            Ok(resp) => println!("{}", parse_ack(&resp)),
            Err(e) => println!("❌ Error: {}", e),
        }
        std::thread::sleep(Duration::from_millis(1500));
    }
}

pub fn verify_1n(port: &mut dyn SerialPort) {
    println!(">> Place finger to verify (1:N)...");
    match send_command(port, 0x0C, 0, 0, 0) {
        Ok(resp) => {
            if resp[4] == ACK_NOUSER {
                println!("❌ No match");
            } else if resp[4] == ACK_TIMEOUT {
                println!("⌛ Timeout");
            } else {
                let uid = ((resp[2] as u16) << 8) | resp[3] as u16;
                println!("✅ Match! User ID: {}, Permission: {}", uid, resp[4]);
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

pub fn verify_1_1(port: &mut dyn SerialPort, user_id: u16) {
    let (uid_h, uid_l) = ((user_id >> 8) as u8, user_id as u8);
    println!(">> Place finger to verify against specific ID...");
    match send_command(port, 0x0B, uid_h, uid_l, 0) {
        Ok(resp) => println!("{}", parse_ack(&resp)),
        Err(e) => println!("❌ Error: {}", e),
    }
}

pub fn delete_user(port: &mut dyn SerialPort, user_id: u16) {
    let (uid_h, uid_l) = ((user_id >> 8) as u8, user_id as u8);
    match send_command(port, 0x04, uid_h, uid_l, 0) {
        Ok(resp) => println!("{}", parse_ack(&resp)),
        Err(e) => println!("❌ Error: {}", e),
    }
}

pub fn delete_all_users(port: &mut dyn SerialPort) {
    match send_command(port, 0x05, 0, 0, 0) {
        Ok(resp) => println!("{}", parse_ack(&resp)),
        Err(e) => println!("❌ Error: {}", e),
    }
}

pub fn query_user_count(port: &mut dyn SerialPort) {
    match send_command(port, 0x09, 0, 0, 0) {
        Ok(resp) => {
            if resp[4] == ACK_SUCCESS {
                let count = ((resp[2] as u16) << 8) | resp[3] as u16;
                println!("👥 Total Users: {}", count);
            } else {
                println!("{}", parse_ack(&resp));
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}

pub fn query_permission(port: &mut dyn SerialPort, user_id: u16) {
    let (uid_h, uid_l) = ((user_id >> 8) as u8, user_id as u8);
    match send_command(port, 0x0A, uid_h, uid_l, 0) {
        Ok(resp) => {
            if resp[4] != ACK_NOUSER {
                println!("🧾 User {} Permission: {}", user_id, resp[4]);
            } else {
                println!("{}", parse_ack(&resp));
            }
        }
        Err(e) => println!("❌ Error: {}", e),
    }
}
