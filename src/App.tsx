import { invoke } from '@tauri-apps/api/tauri';
import { useState } from 'react';
import { Device } from './models';
import DeviceComponent from './components/DeviceComponent';

export default function App() {

  const [devices, setDevices] = useState<Device[]>([]);

  async function s() {
    // (async () => {
    //   const devices = await discover();
    //   console.log(devices);
    // })();
    const res = await invoke('discover')
    setDevices(res as Device[])
    console.log(res)
  }

  return (
    <div className="App">
      <button onClick={() => s()}>discover</button>
      <div className="p-2">
        {devices.map(device => <DeviceComponent key={device.ip} device={device} />)}
      </div>
    </div>
  )
}
