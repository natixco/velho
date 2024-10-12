import { useParams } from 'react-router-dom';
import clsx from 'clsx';
import React, { useState } from 'react';
import { useLights } from '../hooks/useLights';
import { useClickAway } from '@uidotdev/usehooks';
import { Slider } from '../components/Slider';
import { Header } from '../components/Header';
import { Button } from '../components/Button';

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
    <div className="flex flex-col gap-8">
      <Header backLink="/"
              title={<h1 ref={nameRef}
                         className={clsx(
                           '-ml-1 border-2 p-0.5 rounded-md text-2xl font-black text-zinc-900 cursor-text break-all',
                           rename.enabled ? 'border-indigo-500' : 'border-transparent'
                         )}
                         spellCheck={false}
                         contentEditable={rename.enabled}
                         onClick={() => setRename({ enabled: true, name: light.name })}
                         onInput={(e: any) => setRename({ enabled: true, name: e.target.textContent })}
                         dangerouslySetInnerHTML={{ __html: light.name }}>
              </h1>}
              buttons={<>
                <Button theme="secondary" onClick={() => refreshLights()}>
                  <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
                       stroke="currentColor" className="h-[18px] group-hover:text-white transition-none">
                    <path strokeLinecap="round" strokeLinejoin="round" className="transition-none"
                          d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
                  </svg>
                </Button>
                <Button theme="secondary"
                        onClick={() => setPilot(light, { state: !light.state.state })}
                        label={light.state.state ? 'Turn off' : 'Turn on'}/>
              </>}>
      </Header>

      <div className="flex flex-col gap-4">

        <Slider light={light}
                label="Temperature"
                valueLabel={`${light.state.temp}K`}
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

        <Slider light={light}
                label="Brightness"
                valueLabel={`${light.state.dimming}%`}
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
