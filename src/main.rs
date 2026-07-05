use tokio::{io::AsyncReadExt, net::TcpListener};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

const OK: &str = concat!("[", "\x1b[32m", "OK", "\x1b[0m", "]");
const WAIT: &str = concat!("[", "\x1b[33m", "WAIT", "\x1b[0m", "]");

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    println!("{WAIT} Initializing");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("{OK} Server started!");
    loop {
        let (mut socket, cli_addr) = listener.accept().await?;
        println!("{OK} Got connection from {}", cli_addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let _ = socket.read(&mut buf).await;
            
            println!("{}", String::from_utf8_lossy(&buf));
        });
    }
    Ok(())
}
