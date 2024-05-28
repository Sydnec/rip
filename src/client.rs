use std::net::UdpSocket;
use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.connect("127.0.0.1:8080")?;

    socket.send(b"Hello World!")?;

    let mut buf = [0; 1024];
    let (amt, _) = socket.recv_from(&mut buf)?;
    println!("Received {} bytes: {}", amt, str::from_utf8(&buf[..amt]).unwrap());

    Ok(())
}
