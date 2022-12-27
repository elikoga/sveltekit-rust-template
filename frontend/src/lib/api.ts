import { env } from '$env/dynamic/public';

const BACKEND_URL_PREFIX = env.PUBLIC_BACKEND_URL_PREFIX;
export const health = async (fetch: typeof window.fetch = window.fetch) => {
  const res = await fetch(`${BACKEND_URL_PREFIX}/health`);
  const json = await res.json();
  return json;
}

export const random = async (fetch: typeof window.fetch = window.fetch) => {
  const res = await fetch(`${BACKEND_URL_PREFIX}/random`);
  const json = await res.json();
  return json;
}

// export const fetchApi = async (path: string, fetch: typeof window.fetch = window.fetch) => {
//   const res = await fetch(`${BACKEND_URL_PREFIX}/${path}`);
//   const json = await res.json();
//   return json;
// }
