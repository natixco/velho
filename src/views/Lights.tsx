import { Device } from '../models';
import clsx from 'clsx';
import { devicesAtom } from '../store';
import { useEvent } from '../hooks/useEvent';
import { Link } from 'react-router-dom';
import { useAtom } from 'jotai';
import { setState } from '../utils';
import { MouseEvent, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';
import { setWindowSize } from '../utils/window';

export default function Lights() {

  const [devices, setDevices] = useAtom(devicesAtom);

  useEvent<Device>('device_discovery', device => {
    const existingDevice = devices.findIndex(d => d.ip === device.ip);
    if (existingDevice > -1) {
      // const clonedDevices = [...devices];
      // clonedDevices[existingDevice] = device;
      // setDevices(clonedDevices);
    } else {
      setDevices([...devices, device]);
    }
  }, []);

  useEffect(() => {
    invoke<Device[]>('get_devices').then(devices => {
      setDevices(devices);
    });
  }, []);

  useEffect(() => {
    setWindowSize();
  }, [devices]);

  async function onContextMenu(device: Device, event: MouseEvent<HTMLAnchorElement>) {
    event.preventDefault();
    const state = !device.state;
    await setState(device, state);
    device.state = state;
    setDevices([...devices]);
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-2xl font-black text-zinc-900">Lights</h1>
      <div className="grid grid-flow-row grid-cols-2 gap-2">
        {devices.map((device, i) => (
          <Link key={device.mac}
                to={`/${device.mac}`}
                onContextMenu={(e) => onContextMenu(device, e)}
                className={clsx(
                  'rounded-xl p-4 flex flex-col items-start justify-between gap-2 hover:scale-105',
                  device.state ? 'bg-indigo-500' : 'bg-zinc-900'
                )}>
            <p className={clsx(
              'text-xl font-bold transition-none text-zinc-100',
            )}>{device.mac}</p>
            <p className={clsx(
              'text-sm font-medium transition-none text-zinc-200',
            )}>
              {device.state ? 'On' : 'Off'}
            </p>
          </Link>
        ))}
      </div>
    </div>
  );
}
