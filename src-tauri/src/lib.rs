use std::fmt::Formatter;
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

const BROADCAST_ADDRESS : &str = "255.255.255.255";
const LISTENING_ADDRESS : &str = "0.0.0.0";
const MAX_INCOMING_BEACON_SIZE : usize = 1024;
const MAGIC_NUMBER: u16 = 0xbeef;

fn u16_to_array_of_u8(x:u16) -> [u8;2] {
  let b1 : u8 = ((x >> 8) & 0xff) as u8;
  let b2 : u8 = (x & 0xff) as u8;
  [b1, b2]
}

fn array_of_u8_to_u16(array: &[u8]) -> u16 {
  let upper : u16 = (array[0] as u16) << 8;
  let lower : u16 = array[1] as u16;
  upper + lower
}

pub struct BeaconSender {
  socket: UdpSocket,
  beacon_payload: Vec<u8>,
  broadcast_address: String,
}

impl BeaconSender {
  pub fn new(service_port: u16, service_name: &[u8], broadcast_port: u16) -> std::io::Result<Self> {
    // Setting the port to non-zero (or at least the same port used in listener) causes
    // this to fail. I am not sure of the correct value to use. Docs on UDP says '0' is
    // permitted, if you do not expect a response from the UDP Datagram sent.
    let bind_address = "0.0.0.0:0";
    let socket:UdpSocket = UdpSocket::bind(bind_address)?;
    println!("Socket bound to: {}", bind_address);

    socket.set_multicast_loop_v4(false).unwrap();

    socket.set_broadcast(true)?;
    println!("Broadcast mode set to ON");

    // Create payload with magic number, service_port number and service_name
    // let mut beacon_payload: Vec<u8> = u16_to_array_of_u8(MAGIC_NUMBER).to_vec();
    // beacon_payload.append(&mut u16_to_array_of_u8(service_port).to_vec());
    // beacon_payload.append(&mut service_name.to_vec());
    let mut beacon_payload: Vec<u8> = "{\"method\": \"getPilot\", \"params\": {}}".as_bytes().to_vec();

    Ok(Self {
      socket,
      beacon_payload,
      broadcast_address: format!("{}:{}", BROADCAST_ADDRESS, broadcast_port)
    })
  }

  pub fn send_loop(&self, period: Duration) -> std::io::Result<()> {
    loop {
      self.send_one_beacon()?;
      std::thread::sleep(period);
    }
  }

  pub fn send_one_beacon(&self) -> std::io::Result<usize> {
    println!("Sending Beacon '{}' to: '{}'", String::from_utf8_lossy(&self.beacon_payload[..]),
            self.broadcast_address);
    self.socket.send_to(&self.beacon_payload, &self.broadcast_address)
  }
}

pub struct Beacon {
  pub service_ip: String,
  pub service_port: u16,
  pub service_name: Vec<u8>
}

impl std::fmt::Display for Beacon {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    let service_name = String::from_utf8(self.service_name.clone()).unwrap_or_else(|_| "Invalid UTF-8 String".into());
    println!("ServiceName: '{0}', Service IP: {1}, Service Port: {2}", service_name, self.service_ip, self.service_port);
    Ok(())
  }
}

pub struct BeaconListener {
  socket: UdpSocket,
}

impl BeaconListener {
  pub fn new(listening_port: u16) -> std::io::Result<Self> {
    let listening_address = format!("{}:{}", LISTENING_ADDRESS, listening_port);
    let socket = UdpSocket::bind(&listening_address)?;
    println!("Socket bound to: {}", listening_address);

    Ok(Self {
      socket
    })
  }

  pub fn wait(&self, timeout: Option<Duration>) -> () {
    self.socket.set_read_timeout(timeout);
    println!("Read timeout set to: {:?}", timeout);

    println!("Waiting for beacon");
    loop {
      let beacon = self.receive_one_beacon().unwrap();
      println!("Received beacon!!! {}", beacon);
      // return Ok(beacon);
    }
  }

  fn receive_one_beacon(&self) -> std::io::Result<Beacon> {
    let mut buffer = [0; MAX_INCOMING_BEACON_SIZE];

    loop {
      let (number_of_bytes, source_address) = self.socket.recv_from(&mut buffer)?;
      let service_port = array_of_u8_to_u16(&buffer[2..4]);
      let filled_buf = &buffer[..number_of_bytes];
      println!("asd: {}", service_port);
      // let service_name = buffer[0..number_of_bytes].to_vec();
      return Ok(Beacon {
        service_ip: source_address.ip().to_string(),
        service_port,
        service_name: filled_buf.to_vec(),
      });
    }
  }
}
