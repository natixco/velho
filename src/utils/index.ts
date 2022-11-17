import { invoke } from '@tauri-apps/api/tauri';
import { Device } from '../models';

export async function setState(device: Device, params: Record<string, any>) {
  return await invoke('set_state', {
    deviceIp: device.ip,
    params: JSON.stringify(params),
  });
}

export async function setPilot(device: Device, params: Record<string, any>) {
  console.log('setPilot', {
    deviceIp: device.ip,
    params: JSON.stringify(params),
  })
  return await invoke('set_pilot', {
    deviceIp: device.ip,
    params: JSON.stringify(params),
  });
}
