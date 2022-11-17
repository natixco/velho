import { Device } from '../models';
import Toggle from './Toggle';
import { setPilot, setState } from '../utils';
import { useState } from 'react';
import clsx from 'clsx';
import ContextMenuComponent from './ContextMenuComponent';

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

  async function onRangeChange(v: any) {
    console.log(v)
    const res = setPilot(device, {
      // dimming: v,
      // sceneId: 0,
      temp: v - 1000
      // "r":0,"g":0,"b":255
    });
    console.log(res)
  }

  return (
    <ContextMenuComponent items={[
      {
        label: 'Action'
      }
    ]}>
      <div className="bg-zinc-300/10 hover:bg-zinc-400/20 rounded-md p-4" onContextMenu={() => console.log('ctx')}>
        <div className="flex flex-row justify-between items-center" onClick={() => setOpen(!open)}>
          <p className="text-sm text-white">{device.mac}</p>
          <Toggle onChange={() => toggle(device)} enabled={device.state} />
        </div>
        <div className={clsx(open ? '' : 'hidden')}>
          <input type="range" min={2200} max={6500} step={100} onChange={(v) => onRangeChange(v.target.value)}/>
        </div>
      </div>
    </ContextMenuComponent>
  );
}
