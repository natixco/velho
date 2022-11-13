#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::{net, thread, time};
use std::sync::Mutex;
use std::time::Duration;
use crate::socket::{Device, SocketHandler};

mod socket;

// static MULTI_CAST_ADDR: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);
//
// fn listen(socket: &UdpSocket) {
//   loop {
//     println!("LISTEN");
//     let mut buf: [u8; 1024] = [0; 1024];
//     let result = socket.recv_from(&mut buf);
//     match result {
//       Ok((amt, src)) => {
//         println!("Received data from {} - {}", src, String::from_utf8(buf[0..amt].to_vec()).unwrap());
//       },
//       Err(err) => panic!("Read error: {}", err)
//     }
//   }
// }
//
// fn send(socket: &UdpSocket) {
//   loop {
//     println!("SEND");
//     socket.send_to(&"{\"method\": \"getPilot\", \"params\": {}}".as_bytes().to_vec(), "255.255.255.255:38899").unwrap();
//     thread::sleep(Duration::from_secs(2));
//   }
// }

struct WizState(Mutex<InnerState>);

#[derive(Default)]
struct InnerState {
  devices: Vec<Device>,
}

fn main() {

  // thread::spawn(|| {
  //   let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 38899);
  //   let bind_addr = Ipv4Addr::new(0, 0, 0, 0);
  //   let socket = UdpSocket::bind(socket_address).unwrap();
  //   println!("Listening on: {}", socket.local_addr().unwrap());
  //   socket.join_multicast_v4(&MULTI_CAST_ADDR, &bind_addr).unwrap();
  //   loop {
  //     // set up message buffer with size of 120 bytes
  //     let mut buf = [0; 120];
  //     let (data, origin) = socket.recv_from(&mut buf).unwrap();
  //     let buf = &mut buf[..data];
  //     let message = String::from_utf8(buf.to_vec()).unwrap();
  //     println!("server got: {} from {}", message, origin);
  //   }
  // });
  //
  // thread::spawn(|| {
  //   let socket_address: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
  //   let socket = UdpSocket::bind(socket_address).unwrap();
  //   socket.connect(SocketAddrV4::new(MULTI_CAST_ADDR, 38899)).unwrap();
  //   // Don't send messages to yourself.
  //   // In this case self discovery is for human developers, not machines.
  //   socket.set_multicast_loop_v4(false).unwrap();
  //   let data = String::from("{\"method\": \"getPilot\", \"params\": {}}");
  //   loop {
  //     socket.send(data.as_bytes()).unwrap();
  //     thread::sleep(time::Duration::from_secs(2));
  //   }
  // });

  // let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();
  // let result = socket.send_to(&"{\"method\":\"setState\",\"params\":{\"state\":true}}".as_bytes().to_vec(), "192.168.1.106:38899");
  // println!("Result: {}", result.unwrap());

  // let socket = UdpSocket::bind("0.0.0.0:38899").unwrap();
  // let cloned = socket.try_clone().unwrap();
  // socket.set_broadcast(true);
  // thread::spawn(move || {
  //   send(&socket);
  // });
  // listen(&cloned);
  let socket: SocketHandler = SocketHandler::new().unwrap();
  match socket.discover() {
    Ok(devices) => {
      for device in devices.into_iter() {
        print!("{}", device);
      }
    },
    Err(err) => panic!("Discovery error: {}", err),
  }

  tauri::Builder::default()
    .manage(WizState(Default::default()))
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command(state: tauri::State<WizState>) -> usize {
  0
}
