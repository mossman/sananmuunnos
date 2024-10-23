<script lang="ts">
  export let secondword;
  export let firstword;
  export let liked;
  export let failed;
  export let count;

  async function like() {
    const res = await fetch("/api/like", {
      method: "POST",
      headers: {
        "content-type": "application/json"
      },
      body: JSON.stringify({
        'first': firstword,
        'second': secondword,
      })
    })
    const response = await res;

    if (response.ok) {
      try {
        const json = response.json();
        liked = true;
        count += 1;
      } catch (e) {
        failed = true;
      } 
    } else {
      failed = true;
    }
  }

</script>

<tr class="bg-white border-b">
  <td class="py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap">
      {firstword}
  </td>
  <td class="py-4 px-6 text-sm text-gray-500 whitespace-nowrap">
    {secondword}
  </td>
  <td class="py-4 px-6 text-sm font-medium text-right whitespace-nowrap">
    {count}
    {#if liked}
    💪
    {:else if failed}
    💩
    {:else}
    <a href={"#"} on:click|preventDefault={like} class="text-blue-600 hover:text-blue-900">🍺</a>
    {/if}
  </td>
</tr>


<style>

</style>
