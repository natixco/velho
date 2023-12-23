import { BrowserRouter, Route, Routes, useLocation } from 'react-router-dom';
import Lights from './views/Lights';
import Light from './views/Light';
import { useEffect } from 'react';
import { setWindowSize } from './utils/window';

function Test() {
  const location = useLocation();

  useEffect(() => {
    setWindowSize();
  }, [location.pathname]);

  return <></>;
}

export default function App() {

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
          <Route path="/:mac" element={<Light/>}/>
        </Routes>
      </BrowserRouter>
    </div>
  );
}
