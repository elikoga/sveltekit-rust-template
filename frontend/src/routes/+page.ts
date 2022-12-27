import type { PageLoad } from './$types';
import { health, random } from '$lib/api';

export const load = (async (evt) => {
  const { fetch } = evt;
  const healthResponse = await health(fetch);
  const randomResponse = await random(fetch);
  return {
    health: healthResponse,
    random: randomResponse,
  }
}) satisfies PageLoad;