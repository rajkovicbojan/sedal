use std::time::Duration;
use std::io::{self, Read};
use serialport::SerialPort;
use std::env;

const DEFAULT_PORT: &str = "/dev/ttyUSB0";
const DEFAULT_BAUD_RATE: u32 = 9600;
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut port_name = DEFAULT_PORT.to_string();

    if let Some(arg)= env::args().nth(1) {
        // Ako je argument proslijeđen, koristi ga kao naziv porta
        println!("Using port: {}", arg);
        port_name = arg;
    } else {
        println!("No port specified, using default: {}", port_name);
    }

    let mut baud_rate = DEFAULT_BAUD_RATE;
    if let Some(arg) = env::args().nth(2) {
        // Ako je argument proslijeđen, koristi ga kao baud rate
        baud_rate = arg.parse::<u32>().unwrap_or(9600);
        println!("Using baud rate: {}", baud_rate);
    } else {
        println!("No baud rate specified, using default: {}", baud_rate);
    }
    
    let timeout = Duration::from_secs(1);
    
    let mut port = serialport::new(&port_name, baud_rate)
        .timeout(timeout)
        .open()?;

    println!("Reading from port {} at {} baud:", port_name, baud_rate);

    let mut buffer: Vec<u8> = vec![0; 1024];

    loop {
        match port.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let received = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", received);
            },
            Ok(_) => {}, // ništa pročitano, timeout
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}, // timeout
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
