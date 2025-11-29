import { fetch } from '@tauri-apps/plugin-http';

export const useApi = async () => {
  // Send a GET request
  const response = await fetch('http://test.tauri.app/data.json', {
    method: 'GET',
  });
  console.log(response.status); // e.g. 200
  console.log(response.statusText); // e.g. "OK"
  return { response };
};
