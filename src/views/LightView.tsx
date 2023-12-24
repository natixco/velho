import { Link, useParams } from 'react-router-dom';
import clsx from 'clsx';
import { ChangeEvent } from 'react';
import { useLights } from '../hooks/useLights';

export default function LightView() {

  const { lights, setPilot } = useLights();
  const { mac } = useParams();
  const light = lights.find(x => x.state.mac === mac)!;

  async function toggle() {
    const state = !light.state.state;
    await setPilot(light, { state });
  }

  async function setDimming(e: ChangeEvent<HTMLInputElement>) {
    const dimming = parseInt(e.target.value);
    await setPilot(light, { dimming });
  }

  return (
    <div className="flex flex-col justify-end gap-2">
      <Link to="/" className="flex flex-row items-center gap-0.5 group pb-2">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
             className="h-[20px] text-zinc-900 group-hover:text-indigo-500 transition-none">
          <path className="transition-none"
                fillRule="evenodd"
                d="M11.78 5.22a.75.75 0 0 1 0 1.06L8.06 10l3.72 3.72a.75.75 0 1 1-1.06 1.06l-4.25-4.25a.75.75 0 0 1 0-1.06l4.25-4.25a.75.75 0 0 1 1.06 0Z"
                clipRule="evenodd"/>
        </svg>
        <span
          className="font-medium text-sm text-zinc-900 group-hover:text-indigo-500 transition-none">Back to lights</span>
      </Link>

      <div className="flex flex-row items-start justify-between gap-4">
        <h1 className="text-2xl font-black text-zinc-900">{light.state.mac}</h1>
        <button onClick={() => toggle()}
                className={clsx(
                  'h-12 w-12 min-w-12 flex flex-col items-center justify-center group border cursor-default rounded-xl hover:scale-105',
                  light.state.state ? 'bg-indigo-500' : 'bg-zinc-900'
                )}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5"
               stroke="currentColor"
               className={clsx(
                 'h-[26px] pointer-events-none transition-none',
                 light.state.state ? 'text-zinc-100' : 'text-zinc-100'
               )}>
            <path strokeLinecap="round" strokeLinejoin="round"
                  d="M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18"/>
          </svg>
        </button>
      </div>

      <div className="rounded-xl p-4 bg-zinc-300 flex flex-col gap-2">
        <p className="font-bold text-zinc-900">Brightness</p>
        <input type="range" min={10} max={100} step={1} value={light.state.dimming} onChange={e => setDimming(e)}/>
      </div>

    </div>
  );
}
