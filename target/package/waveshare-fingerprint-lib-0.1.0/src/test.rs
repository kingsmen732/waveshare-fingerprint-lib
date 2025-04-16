use fingerprint_sensor_lib::*;
use serialport::SerialPort;
use std::time::Duration;
use std::io::{self, Write};

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let port = serialport::new("COM2", 19200)
        .timeout(Duration::from_secs(2))
        .open();

    if let Ok(mut port) = port {
        loop {
            println!("\n=== Fingerprint Sensor Menu ===");
            println!("1. Register Fingerprint");
            println!("2. Verify Fingerprint (1:N)");
            println!("3. Verify against User ID (1:1)");
            println!("4. Delete User");
            println!("5. Delete All Users");
            println!("6. Query User Count");
            println!("7. Query User Permission");
            println!("8. Exit");

            let choice = read_input("Select an option (1–8): ");

            match choice.as_str() {
                "1" => {
                    let uid = read_input("Enter user ID (1–4095): ").parse().unwrap_or(0);
                    let perm = read_input("Enter permission (1–3): ").parse().unwrap_or(1);
                    enroll(&mut *port, uid, perm);
                }
                "2" => verify_1n(&mut *port),
                "3" => {
                    let uid = read_input("Enter user ID to verify: ").parse().unwrap_or(0);
                    verify_1_1(&mut *port, uid);
                }
                "4" => {
                    let uid = read_input("Enter user ID to delete: ").parse().unwrap_or(0);
                    delete_user(&mut *port, uid);
                }
                "5" => delete_all_users(&mut *port),
                "6" => query_user_count(&mut *port),
                "7" => {
                    let uid = read_input("Enter user ID to query: ").parse().unwrap_or(0);
                    query_permission(&mut *port, uid);
                }
                "8" => {
                    println!("👋 Goodbye!");
                    break;
                }
                _ => println!("❌ Invalid selection!"),
            }
        }
    } else {
        eprintln!("❌ Could not open serial port.");
    }
}
