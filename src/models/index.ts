export interface LightState {
  ip: string;
  mac: string;
  state: boolean;
  scene_id: number;
  temp: number;
  dimming: number;
}

export interface Light {
  name: string;
  state: LightState;
}
