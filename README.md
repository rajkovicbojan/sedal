# SeDaL - Serial Data Logger (WORK IN PROGRESS !)

**SeDaL** (Serial Data Logger) is a simple and efficient command-line tool written in **Rust**. It opens a serial port, listens for incoming UART data, filters lines that begin with specific keywords, and stores those lines into a log file.

---

## 🧩 Key Features

- Written in fast and safe **Rust**
- Opens a serial port and reads incoming UART data
- Filters messages that start with:
  - `ERROR`
  - `ALARM`
  - `LOG`
- Stores filtered messages in a log file
- Lightweight, fast, and reliable

---

## ⚙️ How It Works

1. The user specifies the serial port, baud rate, and output file.
2. SeDaL opens the port and continuously listens for incoming UART data.
3. Each line is checked—if it begins with `ERROR`, `ALARM`, or `LOG`, it's written to the log file.
4. Lines that don’t match the pattern are ignored.

---

## 🖥️ Example UART Input

