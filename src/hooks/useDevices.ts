import { useAtom } from 'jotai';
import { devicesAtom } from '../store';
import { Device } from '../models';
import { invoke } from '@tauri-apps/api';

export function useDevices() {
  const [devices, setDevices] = useAtom(devicesAtom);

  function updateDevice(device: Device, params: Partial<Device>) {
    const index = devices.findIndex(d => d.ip === device.ip);
    if (index === -1) {
      return;
    }

    const newDevices = [...devices];
    newDevices[index] = {
      ...device,
      ...params,
    };

    setDevices(newDevices);
  }

  async function refreshDevices() {
    const devices = await invoke<Device[]>('get_devices');
    setDevices(devices);
  }

  async function setPilot(device: Device, params: Partial<Device>) {
    const success = await invoke<boolean>('set_pilot', {
      deviceIp: device.ip,
      params: params,
    });

    if (success) {
      updateDevice(device, params);
    }

    return success;
  }

  return {
    devices,
    setDevices,
    refreshDevices,
    setPilot,
  };
}
