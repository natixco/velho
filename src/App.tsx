import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { Device } from './models';
import DeviceComponent from './components/DeviceComponent';
import { ArrowPathIcon } from '@heroicons/react/24/outline';
import { listen } from '@tauri-apps/api/event';

export default function App() {

  const [devices, setDevices] = useState<Device[]>([]);

  async function refresh() {
    // (async () => {
    //   const devices = await discover();
    //   console.log(devices);
    // })();
    const res = await invoke('discover')
    setDevices(res as Device[])
    console.log(res)
  }

  useEffect(() => {
    window.addEventListener("contextmenu", e => e.preventDefault());
    listen('test_event', e => {
      console.log(e)
    })
  }, []);

  return (
    <div className="App py-4 px-6">
      <div className="">
        <div className="flex flex-row justify-between items-center mb-4">
          <h1 className="text-sm text-white font-medium">Devices</h1>
          <button onClick={() => refresh()} className="flex flex-row items-center gap-2">
            <span className="text-sm text-zinc-400">Refresh</span>
            <ArrowPathIcon className="h-4 w-4 text-zinc-400" />
          </button>
        </div>
        {devices.map(device => <DeviceComponent key={device.ip} device={device} />)}
      </div>
    </div>
  )
}
