export interface Device {
  ip: string;
  mac: string;
  state: boolean;
  scene_id: number;
  temp: number;
  dimming: number;
}
