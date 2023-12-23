import { BrowserRouter, Route, Routes, useLocation } from 'react-router-dom';
import Lights from './views/Lights';
import Light from './views/Light';
import { useEffect } from 'react';
import { setWindowSize } from './utils/window';
import { useDevices } from './hooks/useDevices';
import { useEvent } from './hooks/useEvent';
import { Device } from './models';

function Test() {
  const location = useLocation();
  const { devices, refreshDevices, setDevices } = useDevices();

  useEffect(() => {
    setWindowSize();
    refreshDevices();
  }, [location.pathname]);

  useEvent<Device>('upsert_device', device => {
    const index = devices.findIndex(x => x.mac === device.mac);
    const devicesCopy = [...devices];
    if (index === -1) {
      devicesCopy.push(device);
    } else {
      devicesCopy[index] = device;
    }
    setDevices(devicesCopy);
  });

  return <></>;
}

export default function App() {

  const { devices } = useDevices();

  useEffect(() => {
    setWindowSize();
  }, [devices]);

  useEffect(() => {
    window.addEventListener('contextmenu', e => e.preventDefault());
  }, []);

  return (
    <div data-tauri-drag-region
         className="container h-full w-full py-4 px-4 bg-zinc-100 rounded-xl flex flex-col gap-4 relative">
      <BrowserRouter>
        <Test/>
        <Routes>
          <Route path="/" element={<Lights/>}/>
          <Route path="/:mac" element={<Light/>}/>
        </Routes>
      </BrowserRouter>
    </div>
  );
}
