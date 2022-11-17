import { useEffect, useState } from 'react';
import { Device } from './models';
import DeviceComponent from './components/DeviceComponent';
import { ArrowPathIcon } from '@heroicons/react/24/outline';
import { emit } from '@tauri-apps/api/event';
import { useEvent } from './hooks/useEvent';

export default function App() {

  const [devices, setDevices] = useState<Device[]>([]);
  useEvent<Device>('device_discovery', device => {
    const existingDevice = devices.findIndex(d => d.ip === device.ip);
    if (existingDevice > -1) {
      const clonedDevices = [...devices];
      clonedDevices[existingDevice] = device;
      setDevices(clonedDevices);
    } else {
      setDevices([...devices, device]);
    }
  });

  useEffect(() => {
    window.addEventListener("contextmenu", e => e.preventDefault());
  }, []);

  return (
    <div className="App py-4 px-6">
      <div className="">
        <div className="flex flex-row justify-between items-center mb-4">
          <h1 className="text-sm text-white font-medium">Devices</h1>
        </div>
        {devices.map((device, i) => <DeviceComponent key={i} device={device} />)}
      </div>
    </div>
  )
}
