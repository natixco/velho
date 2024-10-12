import { Button } from './Button';
import React from 'react';
import { useLights } from '../hooks/useLights';

export function RefreshLightsButton() {
  const { refreshLights } = useLights();

  return (
    <Button label="Refresh" variant="secondary" onClick={() => refreshLights()}>
      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5}
           stroke="currentColor" className="h-[18px] group-hover:text-white transition-none">
        <path strokeLinecap="round" strokeLinejoin="round" className="transition-none"
              d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99"/>
      </svg>
    </Button>
  );
}
