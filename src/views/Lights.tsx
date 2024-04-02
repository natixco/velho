import { Light } from '../models';
import clsx from 'clsx';
import { Link } from 'react-router-dom';
import { useLights } from '../hooks/useLights';
import { Fragment } from 'react';

export default function Lights() {

  const { lights, setPilot, refreshLights } = useLights();

  async function onContextMenu(light: Light) {
    const state = !light.state.state;
    await setPilot(light, { state });
  }

  function getLightElement(light: Light) {
    if (light.available) {
      return (
        <Link key={light.state.mac}
              to={`/${light.state.mac}`}
              onContextMenu={() => onContextMenu(light)}
              className={clsx(
                'rounded-xl p-4 flex flex-col items-start justify-between gap-2 hover:scale-105',
                light.state.state ? 'bg-indigo-500' : 'bg-zinc-900'
              )}>
          <p className="text-xl font-bold transition-none text-zinc-100">
            {light.name}
          </p>
          <p className="text-sm font-medium transition-none text-zinc-200">
            {light.state.state ? 'On' : 'Off'}
          </p>
        </Link>
      );
    }

    return (
      <div className="rounded-xl p-4 flex flex-col items-start justify-between gap-2 bg-zinc-500">
        <p className="text-xl font-bold transition-none text-zinc-100">
          {light.name}
        </p>
        <p className="text-sm font-medium transition-none text-zinc-200">
          Unavailable
        </p>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <div className="h-6 flex flex-row items-center justify-between">
        <h1 className="text-xl font-black text-zinc-900">Lights</h1>
        <button className="flex flex-row items-center gap-2 group active:scale-95" onClick={() => refreshLights()}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
               stroke="currentColor"
               className="h-[18px] text-zinc-900 group-hover:text-indigo-500 transition-transform">
            <path strokeLinecap="round" strokeLinejoin="round"
                  d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"
                  className="transition-none"/>
          </svg>
          <p className="font-bold text-sm text-zinc-900 group-hover:text-indigo-500 transition-none">Refresh</p>
        </button>
      </div>

      <div className="h-[280px] overflow-x-hidden overflow-y-scroll py-4 px-2">
        <div className="grid grid-flow-row grid-cols-2 gap-2">
          {lights.map((light, i) => (
            <Fragment key={light.state.mac}>
              {getLightElement(light)}
            </Fragment>
          ))}
        </div>
      </div>
    </div>
  );
}
