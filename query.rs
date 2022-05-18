use std::io::prelude::*;

static P_VERSION: &[u8] = b"\x15\x00\xf6\x05\x0e";

fn main() -> std::io::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		println!("[-] usage: ./query <server_name>");
		std::process::exit(1)
	}
	let mut host: String;
	let port: i16;
	if args[1].find(':') == None {
		host = args[1].clone();
		port = 25565;
	} else {
		host = args[1].clone();
		let mut tmp = args[1].clone();
		tmp.replace_range(0..host.find(":").unwrap() + 1, "");
		port = tmp.parse::<i16>().unwrap();
		host.replace_range(host.find(":").unwrap()..host.len(), "");
	}
	println!("[*] Trying to connect to {}:{}", host, port);
	let mut stream = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
	println!("[+] Connected to {}:{}", host, port);

	// let packet = P_VERSION.clone() + host.clone() + port.to_string() + "\x01";a
	let mut v_packet = P_VERSION.to_vec();
	v_packet.append(&mut host.as_bytes().to_vec());
	v_packet.append(&mut port.to_be_bytes().to_vec());
	v_packet.append(&mut 1_i8.to_be_bytes().to_vec());
	let packet: &[u8] = &v_packet;
	stream.write(packet)?;
	stream.write(&mut b"\x01\x00".to_vec())?;
	// let mut buffer = String::new();
	// stream.read_to_string(&mut buffer)?;
	let mut buf = vec![0u8; 1024];
	stream.read_exact(&mut buf)?;
	// println!("{:?}", buf);
	println!("{}", String::from_utf8_lossy(&buf));
	Ok(())
}
