import { BrowserRouter, Route, Routes, useLocation } from 'react-router-dom';
import Lights from './views/Lights';
import { useEffect } from 'react';
import { setWindowSize } from './utils/window';
import { useLights } from './hooks/useLights';
import { useEvent } from './hooks/useEvent';
import { Light } from './models';
import LightView from './views/LightView';

function Test() {
  const location = useLocation();
  const { lights, refreshLights, setLights } = useLights();

  useEffect(() => {
    setWindowSize();
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

  return <></>;
}

export default function App() {

  const { lights } = useLights();

  useEffect(() => {
    setWindowSize();
  }, [lights]);

  useEffect(() => {
    window.addEventListener('contextmenu', e => e.preventDefault());
  }, []);

  return (
    <div data-tauri-drag-region
         className="container h-full w-full py-4 px-4 bg-zinc-100 rounded-xl flex flex-col gap-4 relative">
      <BrowserRouter>
        <Test/>
        <Routes>
          <Route path="/" element={<Lights/>}/>
          <Route path="/:mac" element={<LightView/>}/>
        </Routes>
      </BrowserRouter>
    </div>
  );
}
