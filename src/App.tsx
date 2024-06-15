import { Route, Routes, useLocation } from 'react-router-dom';
import Lights from './views/Lights';
import React, { useEffect } from 'react';
import { useLights } from './hooks/useLights';
import { useEvent } from './hooks/useEvent';
import { Light } from './models';
import LightView from './views/LightView';

export default function App() {

  const location = useLocation();
  const { lights, refreshLights, setLights } = useLights();

  useEffect(() => {
    refreshLights();
  }, [location.pathname]);

  useEvent<Light>('upsert_light', light => {
    const index = lights.findIndex(x => x.state.mac === light.state.mac);
    const devicesCopy = [...lights];
    if (index === -1) {
      devicesCopy.push(light);
    } else {
      devicesCopy[index] = light;
    }
    setLights(devicesCopy);
  });

  useEffect(() => {
    window.addEventListener('contextmenu', e => e.preventDefault());
  }, []);

  return (
    <div data-tauri-drag-region className="w-full py-4 px-4 bg-zinc-100 rounded-xl flex flex-col gap-4 relative">
      <Routes>
        <Route path="/" element={<Lights/>}/>
        <Route path="/:mac" element={<LightView/>}/>
      </Routes>
    </div>
  );
}
