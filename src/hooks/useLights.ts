import { useAtom } from 'jotai';
import { lightsAtom } from '../store';
import { Light, LightState } from '../models';
import { invoke } from '@tauri-apps/api';

export function useLights() {
  const [lights, setLights] = useAtom(lightsAtom);

  function setLightState(light: Light, params: Partial<LightState>) {
    const index = lights.findIndex(d => d.state.ip === light.state.ip);
    if (index === -1) {
      return;
    }

    const newLights = [...lights];
    newLights[index] = {
      ...light,
      state: {
        ...light.state,
        ...params,
      },
    };

    setLights(newLights);
  }

  async function refreshLights() {
    const lights = await invoke<Light[]>('get_lights');
    setLights(lights);
  }

  async function setPilot(light: Light, params: Partial<LightState>) {
    const success = await invoke<boolean>('set_pilot', {
      ip: light.state.ip,
      params: params,
    });

    if (success) {
      setLightState(light, params);
    }

    return success;
  }

  async function updateLight(light: Light, params: {name?: string}) {
    const success = await invoke<boolean>('update_light', {
      mac: light.state.mac,
      params: params,
    });

    if (success) {
      refreshLights();
    }

    return success;
  }

  return {
    lights,
    setLights,
    refreshLights,
    setPilot,
    updateLight,
  };
}
