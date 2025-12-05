---
agent: coordinator-planner
name: migrate-esp32-bus-pirate-to-rust
description: Plan and initiate the migration of the esp32‑bus‑pirate firmware from C/ESP‑IDF to Rust (no_std) on the Waveshare ESP32‑S3 Touch LCD 2.8 board.
---
Use the official Waveshare documentation for the **ESP32‑S3‑Touch‑LCD‑2.8** board (<https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8>) to capture key hardware features and pin mappings. The board has a 2.8 inch TFT display (ST7789) and a CST328 capacitive touch controller, plus QMI8658 IMU, PCF85063 RTC, PCM5101 audio decoder, TF card slot, speaker and other peripherals:contentReference[oaicite:7]{index=7}:contentReference[oaicite:8]{index=8}.  The LCD uses MOSI GPIO45, SCLK GPIO40, CS GPIO42, DC GPIO41, RESET GPIO39 and BL GPIO5, while the touch uses I²C SDA GPIO1, SCL GPIO3, INT GPIO4 and RST GPIO2:contentReference[oaicite:9]{index=9}.  You must port the entire firmware to Rust using **no_std**, without depending on ESP‑IDF.

**Steps to carry out:**

1. **Analyze the existing codebase.**  Clone the `esp32-bus-pirate` repository and explore its PlatformIO configuration (`platformio.ini`) and source files to understand each Bus Pirate mode (I²C, SPI, UART, 1‑Wire, etc.), CLI handling, and peripheral initialization.
2. **Design the Rust project structure.**  Create a top‑level workspace or set of crates to separate the hardware abstraction layer (HAL), display & touch drivers, binary protocol implementation and application logic.  Target `xtensa-esp32s3-none-elf` with `no_std` enabled.  Select appropriate crates (`esp32-s3-hal`, `embedded-hal`, `embedded-graphics`, `st7789`, `heapless`, `prost-lite`/`postcard`, etc.).
3. **Define the binary protocol.**  With the Protocol & CLI Developer, specify the command messages (mode selection, bus operations, configuration, file access) and framing (start byte, length, version, checksum).  Choose between Protocol Buffers and simpler schemes like `postcard`, prioritizing compactness and compatibility with `no_std` Rust.
4. **Draft a high-level design document.**  Summarize how each C component will be replaced by a Rust module or crate.  Include diagrams of module dependencies and data flow.  Describe how the HAL will abstract peripheral access, how the display and touch drivers will integrate with the GUI layer, and how the protocol will interact with bus‑mode modules.
5. **Create an initial project skeleton.**  Set up the Rust workspace with `Cargo.toml` files and stub modules for the HAL, display/touch drivers, protocol and bus modes.  Include `build.rs` if necessary for Xtensa cross‑compilation.  Ensure the project builds (even if functionality is stubbed) without `std`.
6. **Coordinate subsequent tasks.**  Use GitHub issues or tasks to assign specific modules to the appropriate agents: HAL‑Peripherals Developer, Display & Touch Developer, Protocol & CLI Developer, Bus‑Mode Engineers, and Testing & CI Engineer.  Provide clear definitions of done, deadlines, and deliverables for each.

**Deliverables:**  A comprehensive design document committed to the repository (e.g., `docs/design.md`), an initial Rust project scaffold that compiles for the ESP32‑S3 with `no_std`, and a set of issues or tasks filed to delegate detailed implementation work to the other agents.

Ensure that your plan strictly avoids using ESP‑IDF or any crates that depend on `std`.  Emphasize accurate pin mappings and hardware details from the Waveshare documentation.  When uncertain about a peripheral, consult datasheets and update the design accordingly.  Your work will serve as the blueprint for the entire migration.
