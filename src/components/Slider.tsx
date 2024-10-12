import { ILight } from '../models';
import React, { ChangeEvent } from 'react';
import clsx from 'clsx';
import { Button } from './Button';

interface SliderProps {
  title: string;
  light: ILight;
  subtitle: string;
  onDecrease: () => void;
  onIncrease: () => void;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
  input: {
    min: number;
    max: number;
    step: number;
    value: number;
  };
}

export function Slider(props: SliderProps) {
  return (
    <div className="rounded-xl flex flex-col gap-2">
      <div className="flex flex-row items-center gap-2">
        <p className="font-bold text-zinc-900">{props.title}</p>
        <span className="font-medium text-sm text-zinc-500">{props.subtitle}</span>
      </div>
      <div className="h-10 grid grid-rows-1 grid-cols-[2.5rem_1fr_2.5rem] gap-2 items-center">
        <Button variant="secondary" onClick={() => props.onDecrease()}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5"
               stroke="currentColor" className="min-w-[16px] group-hover:text-white transition-none">
            <path strokeLinecap="round" strokeLinejoin="round" d="M5 12h14" className="transition-none"/>
          </svg>
        </Button>
        <input type="range" min={props.input.min} max={props.input.max} step={props.input.step}
               value={props.input.value}
               onChange={e => props.onChange(e)}
               className={clsx(
                 'appearance-none w-full h-2 bg-zinc-300 rounded-full outline-none',
                 '[&::-webkit-slider-thumb]:appearance-none',
                 '[&::-webkit-slider-thumb]:w-4',
                 '[&::-webkit-slider-thumb]:h-4',
                 '[&::-webkit-slider-thumb]:rounded-full',
                 '[&::-webkit-slider-thumb]:bg-indigo-500',
                 'active:[&::-webkit-slider-thumb]:scale-[120%]',
               )}/>
        <Button variant="secondary" onClick={() => props.onIncrease()}>
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth="1.5"
               stroke="currentColor" className="min-w-[16px] group-hover:text-white transition-none">
            <path strokeLinecap="round" strokeLinejoin="round" d="M12 4.5v15m7.5-7.5h-15" className="transition-none"/>
          </svg>
        </Button>
      </div>
    </div>
  );
}
