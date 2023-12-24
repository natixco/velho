import { Light } from '../models';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { useLights } from '../hooks/useLights';

export default function Lights() {

  const { lights, setPilot } = useLights();

  async function onContextMenu(light: Light) {
    const state = !light.state.state;
    await setPilot(light, { state });
  }

  return (
    <div className="flex flex-col gap-6">
      <h1 className="text-2xl font-black text-zinc-900">Lights</h1>
      <div className="grid grid-flow-row grid-cols-2 gap-2">
        {lights.map((light, i) => (
          <Link key={light.state.mac}
                to={`/${light.state.mac}`}
                onContextMenu={() => onContextMenu(light)}
                className={clsx(
                  'rounded-xl p-4 flex flex-col items-start justify-between gap-2 hover:scale-105',
                  light.state.state ? 'bg-indigo-500' : 'bg-zinc-900'
                )}>
            <p className={clsx(
              'text-xl font-bold transition-none text-zinc-100',
            )}>{light.state.mac}</p>
            <p className={clsx(
              'text-sm font-medium transition-none text-zinc-200',
            )}>
              {light.state.state ? 'On' : 'Off'}
            </p>
          </Link>
        ))}
      </div>
    </div>
  );
}
