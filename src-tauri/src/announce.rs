use std::time::Duration;
use app::BeaconSender;

const BEACON_TEST_SERVICE_PORT : u16 = 9999;
const BEACON_TEST_SERVICE_NAME :&str = "BeaconTestService";

pub fn main() -> std::io::Result<()> {

  println!("\nHit Control-C to kill the process and stop beacon sending\n");

  let args : Vec<String> = std::env::args().collect();
  let service_name = match args.len() {
    0..=1 => BEACON_TEST_SERVICE_NAME,
    _ => &args[1]
  };

  println!("Beacon message set to: '{}'", service_name);

  if let Ok(beacon) = BeaconSender::new(BEACON_TEST_SERVICE_PORT,
                                        service_name.as_bytes(), 38899) {
    beacon.send_loop(Duration::from_secs(1))?;
  }

  Ok(())
}
