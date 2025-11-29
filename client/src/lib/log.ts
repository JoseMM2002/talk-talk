import { error as TauriError, info as TauriInfo } from '@tauri-apps/plugin-log';

import { isWindowTauri } from './utils';

const isTauri = isWindowTauri();

export const error = (msg: string) => {
  if (isTauri) TauriError(msg);
  else console.error(msg);
};

export const info = (msg: string) => {
  if (isTauri) TauriInfo(msg);
  else console.log(msg);
};
