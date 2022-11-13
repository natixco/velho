import { Device } from '../models';
import Toggle from './Toggle';
import { setState } from '../utils';
import { useState } from 'react';
import clsx from 'clsx';

interface Props {
  device: Device;
}

export default function DeviceComponent(props: Props) {

  const [device, setDevice] = useState(props.device);
  const [open, setOpen] = useState(false);

  async function toggle(device: Device): Promise<void> {
    const res = setState(device, {
      state: !device.state,
    });
    setDevice({...device, state: !device.state});
  }

  return (
    <div className="flex flex-row justify-between items-center bg-gray-200/60 rounded-2xl p-4 cursor-pointer">
      <p className="text-sm text-zinc-900 font-medium">{device.mac}</p>
      <Toggle onChange={() => toggle(device)} enabled={device.state} />
    </div>
  );
}
