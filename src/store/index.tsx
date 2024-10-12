import { ILight } from '../models';
import { atom } from 'jotai';

export const lightsAtom = atom<ILight[]>([]);
