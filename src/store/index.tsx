import { Device } from '../models';
import { atom } from 'jotai';

export const devicesAtom = atom<Device[]>([]);
