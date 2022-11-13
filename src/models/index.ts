export interface Device {
  ip: string;
  mac: string;
  state: boolean;
}

export interface Bulb extends Device {
  scene_id: number;
  temp: number;
  dimming: number;
}
