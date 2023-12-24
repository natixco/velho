import { LightState } from '../models';
import Toggle from './Toggle';
import { setPilot, setState } from '../utils';
import { useEffect, useState } from 'react';
import clsx from 'clsx';
import ContextMenuComponent from './ContextMenuComponent';

interface Props {
  device: LightState;
}

export default function DeviceComponent(props: Props) {

  const [device, setDevice] = useState(props.device);
  const [open, setOpen] = useState(false);

  useEffect(() => {
    setDevice(props.device);
  }, [props.device])

  return (
    <ContextMenuComponent items={[
      {
        label: 'Action'
      }
    ]}>
      <div className="bg-zinc-300/10 hover:bg-zinc-400/20 rounded-md p-4" onContextMenu={() => console.log('ctx')}>
        <div className="flex flex-row justify-between items-center" onClick={() => setOpen(!open)}>
          <p className="text-sm text-white">{device.mac}</p>
          <Toggle enabled={device.state} onChange={() => {
            setState(device, !device.state);
            setDevice({ ...device, state: !device.state });
          }}/>
        </div>
        <div className={clsx(open ? '' : 'hidden')}>
          <div>
            <label htmlFor="temp">Temp</label>
            <input type="range" name="temp" min={2200} max={6500} step={100} value={device.temp} onChange={v => {
              const params = { temp: v.target.valueAsNumber };
              setPilot(device, params);
              setDevice({ ...device, ...params })
            }}/>
          </div>
          <div>
            <label htmlFor="dimming">Dimming</label>
            <input type="range" name="dimming" min={10} max={100} step={1} value={device.dimming} onChange={v => {
              const params = { dimming: v.target.valueAsNumber };
              setPilot(device, params);
              setDevice({ ...device, ...params });
            }}/>
          </div>
        </div>
      </div>
    </ContextMenuComponent>
  );
}
