import { Device } from '../models';
import { emit } from '@tauri-apps/api/event';

export function setState(device: Device, state: boolean): void {
  emit('set_state', {
    deviceIp: device.ip,
    state,
  });
}

export function setPilot(device: Device, params: Partial<Device>): void {
  emit('set_pilot', {
    deviceIp: device.ip,
    params: params,
  });
}
