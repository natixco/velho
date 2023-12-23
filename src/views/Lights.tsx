import { Device } from '../models';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { useDevices } from '../hooks/useDevices';

export default function Lights() {

  const { devices, setState } = useDevices();

  async function onContextMenu(device: Device) {
    const state = !device.state;
    await setState(device, state);
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-2xl font-black text-zinc-900">Lights</h1>
      <div className="grid grid-flow-row grid-cols-2 gap-2">
        {devices.map((device, i) => (
          <Link key={device.mac}
                to={`/${device.mac}`}
                onContextMenu={() => onContextMenu(device)}
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
