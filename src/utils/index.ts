import { Device } from '../models';
import { emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api';

export function setState(device: Device, state: boolean) {
  return emit('set_state', {
    device_ip: device.ip,
    state,
  });
}

export function setPilot(device: Device, params: Partial<Device>) {
  return emit('set_pilot', {
    device_ip: device.ip,
    params: params,
  });
}
