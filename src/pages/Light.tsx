import { Link, useParams } from 'react-router-dom';
import clsx from 'clsx';
import React, { useState } from 'react';
import { useLights } from '../hooks/useLights';
import { useClickAway } from '@uidotdev/usehooks';
import { Slider } from '../components/Slider';
import { Header } from '../components/Header';
import { Button } from '../components/Button';
import { RefreshLightsButton } from '../components/RefreshLightsButton';

export default function Light() {
  const { lights, setPilot, updateLight, refreshLights } = useLights();
  const { mac } = useParams();
  const light = lights.find(x => x.state.mac === mac)!;
  const [rename, setRename] = useState({ enabled: false, name: '' });

  const nameRef = useClickAway<HTMLHeadingElement>(async () => {
    if (!rename.enabled) {
      return;
    }

    setRename({ enabled: false, name: '' });
    if (rename.name) {
      await updateLight(light, { name: rename.name });
    }
  });

  return (
    <div className="flex flex-col gap-4">
      <Header>
        <Header.Top>
          <Link to="/" className="flex flex-row items-center gap-0.5 group">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"
                 className="h-[20px] text-zinc-900 group-hover:text-indigo-500 transition-none">
              <path className="transition-none"
                    fillRule="evenodd"
                    d="M11.78 5.22a.75.75 0 0 1 0 1.06L8.06 10l3.72 3.72a.75.75 0 1 1-1.06 1.06l-4.25-4.25a.75.75 0 0 1 0-1.06l4.25-4.25a.75.75 0 0 1 1.06 0Z"
                    clipRule="evenodd"/>
            </svg>
            <span className="font-medium text-sm text-zinc-900 group-hover:text-indigo-500 transition-none">
              Back to lights
            </span>
          </Link>
        </Header.Top>
        <Header.Title>
          <h1 ref={nameRef}
              className={clsx(
                '-ml-1 border-2 p-0.5 rounded-md text-2xl font-black text-zinc-900 cursor-text break-all',
                rename.enabled ? 'border-indigo-500' : 'border-transparent'
              )}
              spellCheck={false}
              contentEditable={rename.enabled}
              onClick={() => setRename({ enabled: true, name: light.name })}
              onInput={(e: any) => setRename({ enabled: true, name: e.target.textContent })}
              dangerouslySetInnerHTML={{ __html: light.name }}>
          </h1>
        </Header.Title>
        <Header.Buttons>
          <RefreshLightsButton/>
          <Button variant="secondary"
                  onClick={() => setPilot(light, { state: !light.state.state })}
                  label={light.state.state ? 'Turn off' : 'Turn on'}/>
        </Header.Buttons>
      </Header>

      <div className="flex flex-col gap-4">

        <Slider title="Temperature"
                light={light}
                subtitle={`${light.state.temp}K`}
                onDecrease={() => {
                  if (light.state.temp > 2200) {
                    void setPilot(light, { temp: light.state.temp - 100 })
                  }
                }}
                onIncrease={() => {
                  if (light.state.temp < 6500) {
                    void setPilot(light, { temp: light.state.temp + 100 })
                  }
                }}
                onChange={e => setPilot(light, { temp: parseInt(e.target.value) })}
                input={{
                  min: 2200,
                  max: 6500,
                  step: 100,
                  value: light.state.temp,
                }}/>

        <Slider title="Brightness"
                light={light}
                subtitle={`${light.state.dimming}%`}
                onDecrease={() => {
                  if (light.state.dimming > 10) {
                    void setPilot(light, { dimming: light.state.dimming - 1 })
                  }
                }}
                onIncrease={() => {
                  if (light.state.dimming < 100) {
                    void setPilot(light, { dimming: light.state.dimming + 1 })
                  }
                }}
                onChange={e => setPilot(light, { dimming: parseInt(e.target.value) })}
                input={{
                  min: 10,
                  max: 100,
                  step: 1,
                  value: light.state.dimming,
                }}/>
      </div>

    </div>
  );
}
