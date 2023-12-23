import { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';

export function useEvent<T>(eventName: string, onEvent: (cb: T) => void): void {
  const unlisten = useRef<any>(undefined);
  const isUnlistenSet = useRef(false);

  useEffect(() => {
    async function setListen() {
      unlisten.current = await listen<T>(eventName, event => onEvent(event.payload));
    }

    if (!isUnlistenSet.current) {
      setListen();
      isUnlistenSet.current = true;
    }

    return () => {
      if (unlisten.current) {
        unlisten.current();
        isUnlistenSet.current = false;
      }
    }
  });
}

// useEffect(() => {
//   const unlisten = listen<Device>('device_discovery', (event) => {
//     console.log(event.payload, devices)
//     const existingDevice = devices.findIndex(d => d.ip === event.payload.ip);
//     if (existingDevice === -1) {
//       addDevice(event.payload);
//     }
//   });
//
//   return () => {
//     unlisten.then(f => f());
//   };
//
// }, [devices]);
