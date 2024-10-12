import { ILight } from '../models';
import React, { MouseEvent, ChangeEvent, useRef, useState, useEffect } from 'react';
import clsx from 'clsx';
import { Button } from './Button';
import { MinusIcon, PlusIcon } from '@heroicons/react/24/solid';

interface SliderProps {
  label: string;
  light: ILight;
  valueLabel: string;
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

  const ref = useRef<HTMLInputElement>(null);

  if (ref.current) {
    const progress = (props.input.value - props.input.min) / (props.input.max - props.input.min) * 100;
    ref.current.style.background = `linear-gradient(to right, var(--stone-900) ${progress}%, var(--stone-200) ${progress}%)`;
  }

  return (
    <div className="rounded-xl flex flex-col gap-2">
      <div className="flex flex-row items-center justify-between">
        <p className="font-bold text-stone-900">{props.label}</p>
        <span className="font-medium text-sm text-stone-900">{props.valueLabel}</span>
      </div>
      <div className="h-10 grid grid-rows-1 grid-cols-[2.5rem_1fr_2.5rem] gap-2 items-center">
        <Button theme="secondary" onClick={() => props.onDecrease()}>
          <MinusIcon className="h-6 text-stone-900"/>
        </Button>
        <input type="range" min={props.input.min} max={props.input.max} step={props.input.step}
               value={props.input.value}
               ref={ref}
               onChange={e => props.onChange(e)}
               className={clsx(
                 'webkit-slider-thumb appearance-none w-full h-2 bg-stone-200 rounded-full outline-none',
                 '[&::-webkit-slider-thumb]:appearance-none',
                 '[&::-webkit-slider-thumb]:w-4',
                 '[&::-webkit-slider-thumb]:h-4',
                 '[&::-webkit-slider-thumb]:rounded-full',
                 '[&::-webkit-slider-thumb]:bg-white',
                 'active:[&::-webkit-slider-thumb]:scale-[110%]',
               )}/>
        <Button theme="secondary" onClick={() => props.onIncrease()}>
          <PlusIcon className="h-6 text-stone-900"/>
        </Button>
      </div>
    </div>
  );
}
