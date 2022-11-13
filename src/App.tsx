import { useEffect, useState } from 'react';
import { discover } from './utils';
import { invoke } from '@tauri-apps/api/tauri';

export default function App() {

  async function s() {
    // (async () => {
    //   const devices = await discover();
    //   console.log(devices);
    // })();
    const res = await invoke('my_custom_command')
    console.log(res)
  }

  return (
    <div className="App">
      <button onClick={() => s()}>discover</button>
    </div>
  )
}
