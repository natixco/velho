import { ILight } from '../models';
import { Link } from 'react-router-dom';
import { useLights } from '../hooks/useLights';
import React from 'react';
import { Header } from '../components/Header';
import { Button } from '../components/Button';

export default function Lights() {
  const { lights, refreshLights } = useLights();

  return (
    <div className="flex flex-col gap-4">
      <Header title={<h1 className="text-xl font-black text-zinc-900">Lights</h1>}
              buttons={<>
                <Button theme="secondary" onClick={() => refreshLights()}>
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
                       stroke="currentColor" className="h-[18px] group-hover:text-white transition-none">
                    <path strokeLinecap="round" strokeLinejoin="round" className="transition-none"
                          d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
                  </svg>
                </Button>
              </>}>
      </Header>

      <div className="h-[calc(100vh-1.5rem)] overflow-x-hidden overflow-y-scroll">
        <div className="grid grid-flow-row grid-cols-2 gap-2 p-1">
          {lights.map((light, i) => (
            <LightItem key={light.state.mac} light={light}/>
          ))}
        </div>
      </div>
    </div>
  );
}

function LightItem({ light }: { light: ILight }) {
  const { setPilot } = useLights();

  async function onContextMenu(light: ILight) {
    const state = !light.state.state;
    await setPilot(light, { state });
  }

  if (light.available) {
    return (
      <Link key={light.state.mac}
            to={`/${light.state.mac}`}
            onContextMenu={() => onContextMenu(light)}
            className="border border-stone-400 rounded-md p-4 flex flex-col items-start justify-between gap-2 hover:bg-stone-100 cursor-pointer ring ring-transparent active:ring-stone-200">
        <p className="text-2xl font-bold transition-none text-zinc-900">
          {light.name}
        </p>
        <p className="text-sm font-bold transition-none text-stone-900">
          {light.state.state ? 'On' : 'Off'}
        </p>
      </Link>
    );
  }

  return (
    <div className="border border-stone-200 rounded-md p-4 flex flex-col items-start justify-between gap-2">
      <p className="text-2xl font-bold transition-none text-zinc-900">
        {light.name}
      </p>
      <p className="text font-medium transition-none text-zinc-700">
        Unavailable
      </p>
    </div>
  );
}
