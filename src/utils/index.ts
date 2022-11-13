// import { createSocket } from 'dgram';
// import Discovery from 'udp-discovery';

const WIZ_PORT = 38899;

// export function udpRequest(ipAddress: string, method: 'getPilot' | 'setState', params: Record<string, any> = {}): Promise<any> {
//   return new Promise((resolve, reject) => {
//     const client = createSocket('udp4');
//     const message = JSON.stringify({
//       method,
//       params
//     });
//     client.send(message, 0, message.length, WIZ_PORT, ipAddress);
//     client.on('message', (msg, info) => {
//       // client.close();
//       return resolve(JSON.parse(Buffer.from(msg).toString()));
//     });
//   })
// }

const WIZ_HANDSHAKE = {
  method: 'getPilot',
  params: {}
};

export function discover() {
  // return new Promise((resolve, reject) => {
  //   Discovery.discover({
  //     port: WIZ_PORT,
  //     handshake: WIZ_HANDSHAKE
  //   }, async (err: any, msg: any, info: any) => {
  //     if (err) {
  //       return console.error(err);
  //     }
  //     if (msg && info) {
  //       console.log(`Message from ${info.address}: ${msg.toString()}`);
  //       const obj = JSON.parse(Buffer.from(msg).toString());
  //       return resolve(obj);
  //     }
  //   });
  // })
}

