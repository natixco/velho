import { Light } from '../models';
import { atom } from 'jotai';

export const lightsAtom = atom<Light[]>([]);
