<script lang="ts">
  import { invalidateAll } from '$app/navigation';
  import type { PageData } from './$types';

  export let data: PageData;
</script>

<h1>sveltekit-rust-auth-frontend</h1>

<p>Health: {data.health.status}</p>
<p>Random: {data.random.random}</p>

<a href="."><button on:click={invalidateAll}>Reload</button></a>
