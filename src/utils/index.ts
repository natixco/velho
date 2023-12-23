import { Device } from '../models';
import { invoke } from '@tauri-apps/api';

export function setState(device: Device, state: boolean) {
  return invoke('set_state', {
    deviceIp: device.ip,
    state,
  });
}

export function setPilot(device: Device, params: Partial<Device>) {
  return invoke('set_pilot', {
    deviceIp: device.ip,
    params: params,
  });
}
