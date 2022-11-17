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
  }, []);
}
