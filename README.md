## Embedded Projects 🚀

A collection of embedded systems projects built using different microcontrollers and programming languages.

This repository includes projects for:
- **Raspberry Pi Pico / Pico W / Pico 2 W**
- **BBC micro:bit**

---

## 📁 Repository Structure

```
embedded-projects/
│
├── pico/
│   ├── c/
│   │   └── project-name/
│   │       ├── src/
│   │       ├── CMakeLists.txt
│   │       └── README.md
│   ├── rust/
│   │   └── project-name/
│   │       ├── src/
│   │       ├── Cargo.toml
│   │       └── README.md
│   └── micropython/
│       └── project-name/
│           ├── main.py
│           └── README.md
│
├── microbit/
│   ├── micropython/
│   │   └── project-name/
│   │       ├── main.py
│   │       └── README.md
│   └── makecode/
│       └── project-name/
│           └── README.md
│
└── README.md
```

---

## 🧠 Platforms

### Raspberry Pi Pico Series
Low-level embedded development with powerful tools and multiple language support.

| Language | Framework | Build Tool |
|----------|-----------|-----------|
| **C/C++** | Pico SDK | CMake |
| **Rust** | Embassy, rp-hal | Cargo |
| **MicroPython** | MicroPython Firmware | `mpremote` / `ampy` |

### BBC micro:bit
Rapid prototyping and educational projects with beginner-friendly development environments.

| Language | Framework | Tool |
|----------|-----------|------|
| **MicroPython** | MicroPython Firmware | `uflash` / Web Editor |
| **MakeCode** | Visual Blocks / JavaScript | Microsoft MakeCode Editor |

---

## ⚙️ Requirements

### Pico (C/C++)
- ARM GCC Toolchain
- CMake (≥ 3.12)
- [Raspberry Pi Pico SDK](https://github.com/raspberrypi/pico-sdk)

### Pico (Rust)
- Rust toolchain ([install via `rustup`](https://rustup.rs/))
- Targets: `thumbv6m-none-eabi` or `thumbv8m.main-none-eabihf`
- Tools: `probe-rs` or `elf2uf2-rs` or `picotool`

### Pico & micro:bit (MicroPython)
- MicroPython firmware pre-installed on device
- Host tools: `mpremote` or `ampy`

### micro:bit (MakeCode)
- [Microsoft MakeCode Editor](https://makecode.microbit.org/) (web or offline)

---

## 🚀 Quick Start

### Clone the Repository

```bash
git clone https://github.com/judevector/embedded-projects.git
cd embedded-projects
```

### Run a Project

1. Navigate to your chosen project folder:
   ```bash
   cd pico/rust/project-name
   # or
   cd microbit/micropython/project-name
   ```

2. Follow the **README.md** inside that folder for build and flash instructions.

---

## ⚡ Flashing Devices

### Raspberry Pi Pico

| Method | Command |
|--------|---------|
| **Drag & Drop** | Copy `.uf2` file to Pico (bootloader mode) |
| **elf2uf2-rs** | `elf2uf2-rs input.elf output.uf2` |
| **picotool** | `picotool load .uf2` |
| **probe-rs** | `probe-rs run --chip RP2350 firmware.elf` |

### BBC micro:bit

| Method | Usage |
|--------|-------|
| **Drag & Drop** | Copy `.hex` file to micro:bit |
| **MakeCode Web** | Built-in flashing in editor |
| **uflash** | `uflash main.py /path/to/microbit` |

---

## 🧪 Project Ideas

**Pico:**
- GPIO control (LEDs, buttons)
- Sensors (temperature, motion, distance)
- I2C/SPI communication
- WiFi projects (Pico W / Pico 2 W)
- Display projects (OLED, LCD)
- Motor control (PWM)

**micro:bit:**
- LED animations and patterns
- Button interaction
- Accelerometer projects
- Simple games
- Radio communication (multi-board)
- Temperature sensing

---

## 🎯 Goals

- 📚 Learn embedded systems at different levels
- 🦀 Explore Rust on microcontrollers
- 🔄 Compare different hardware ecosystems
- 🔧 Build real-world hardware projects
- 📡 Experiment with wireless (Pico W / Pico 2 W)
- 🎓 Create a portfolio of practical examples

---

## 🤝 Contributing

This is a personal learning repository, but feel free to:
- Fork and adapt projects for your own learning
- Use as inspiration for your embedded projects
- Share improvements or new project ideas

---

## 📝 License

MIT License

---

## 📚 Useful Resources

**Raspberry Pi Pico:**
- [Official Pico Documentation](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html)
- [Pico SDK GitHub](https://github.com/raspberrypi/pico-sdk)
- [Embassy Rust Framework](https://embassy.dev/)
- [MicroPython on Pico](https://micropython.org/download/rp2-pico/)

**BBC micro:bit:**
- [micro:bit Official Site](https://microbit.org/)
- [MicroPython Documentation](https://microbit-micropython.readthedocs.io/)
- [MakeCode Editor](https://makecode.microbit.org/)

---

## **Happy coding! 🎉**
