# Waveshare Fingerprint (C)

A Rust library to interface with the **Waveshare UART Fingerprint Sensor (C)** via serial communication. This library supports full functionality, including enrollment, verification, deletion, and querying of fingerprint data.

RAW implementation of this project is available in:
- RUST
- PYTHON


[ ⭐ Link to the Project](https://github.com/kingsmen732/waveshare_fingerprint_-c-) 

If link not redirecting use :
https://github.com/kingsmen732/waveshare_fingerprint_-c-

This crate enables enrolling, verifying, listing, and deleting fingerprints using the sensor's built-in flash memory.

## ✨ Features

- Register fingerprints with user ID and permission level.
- Verify fingerprints in 1:N and 1:1 mode.
- Delete individual or all users.
- Query total number of enrolled users.
- Retrieve permission level of specific user IDs.
- Easy-to-use API for seamless integration.

---

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fingerprint-lib = "0.1.0"
serialport = "4.2"  # Required for serial port communication
```

## 🚀 Usage

Here's an example of how to use the library:

```rust
use fingerprint_lib::fingerprint;
use std::time::Duration;

fn main() {
    let port_name = "/dev/ttyUSB0"; // Replace with your serial port
    let mut port = serialport::new(port_name, 57600)
        .timeout(Duration::from_secs(2))
        .open()
        .expect("Failed to open serial port");

    // Enroll a new fingerprint with ID 5
    fingerprint::enroll_fingerprint(&mut *port, 5).unwrap();

    // Verify a fingerprint
    fingerprint::verify_fingerprint(&mut *port).unwrap();

    // List all stored fingerprint IDs
    fingerprint::list_fingerprints(&mut *port).unwrap();

    // Delete a fingerprint with ID 5
    fingerprint::delete_fingerprint(&mut *port, 5).unwrap();
}
```
## 🌟 Acknowledgements

This project acknowledges the invaluable guidance and support of <b> Prof. Sibi Chakkaravarthy Sethuraman (VIT-AP University) </b> and <b>Prof. Chester Rebeiro (Indian Institute of Technology Madras) </b>in the development of this tool. Their expertise and contributions have been instrumental in bringing this project to fruition.

## 📜 Research colab in AIR CENTER 

Research outcome supported by <b> Indominus labs Private Limited </b > and <b> Digital Fortress Private Limited </b>



## 💡 Sensor Compatibility

This crate is designed for fingerprint modules that communicate over UART and adhere to the Waveshare fingerprint (C), (E) or any other version packet protocol matters.

### Tested Modules

- **Waveshare fingerprint (C)**
- Other compatible modules using Adafruit's fingerprint protocol.

---

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.

---

## 👋 Contributions

Contributions are welcome! Feel free to open pull requests or suggest new features.

---

Let me know if you'd like me to auto-generate a `Cargo.toml` for publishing!
