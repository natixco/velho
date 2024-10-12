import { ILight } from '../models';
import { Link } from 'react-router-dom';
import { useLights } from '../hooks/useLights';
import React from 'react';
import { Header } from '../components/Header';
import { RefreshLightsButton } from '../components/RefreshLightsButton';

export default function Lights() {
  const { lights } = useLights();

  return (
    <div className="flex flex-col gap-4">
      <Header>
        <Header.Title>
          <h1 className="text-xl font-black text-zinc-900">Lights</h1>
        </Header.Title>
        <Header.Buttons>
          <RefreshLightsButton/>
        </Header.Buttons>
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
            className="border-2 p-4 flex flex-col items-start justify-between gap-2 border-black hover:shadow-[0.25rem_0.25rem_0_black] shadow-none hover:-translate-x-1 hover:-translate-y-1">
        <p className="text-2xl font-bold transition-none text-zinc-900">
          {light.name}
        </p>
        <p className="text-sm font-bold transition-none text-zinc-700">
          {light.state.state ? 'On' : 'Off'}
        </p>
      </Link>
    );
  }

  return (
    <div className="border-2 border-zinc-900 p-4 flex flex-col items-start justify-between gap-2">
      <p className="text-2xl font-bold transition-none text-zinc-900">
        {light.name}
      </p>
      <p className="text font-medium transition-none text-zinc-700">
        Unavailable
      </p>
    </div>
  );
}
