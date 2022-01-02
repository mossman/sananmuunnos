<script lang="ts">
import FirstWord from "./FirstWord.svelte";
import SecondWord from "./SecondWord.svelte";
import TopWord from "./TopWord.svelte";
import { BarLoader } from 'svelte-loading-spinners';

  let word = '';

  let promise = Promise.resolve([]);
  let top_promise = Promise.resolve([]);

  async function fetchItems() {
    const response = await fetch("/api/word/"+word);

    if (response.ok) {
      return response.json();
    } else {
      throw new Error(items);
    }
  }

  const getItems = () => {
    promise = fetchItems();
    top_promise = Promise.resolve([]);
  }

  async function fetchTopItems() {
    const response = await fetch("/api/likes");

    if (response.ok) {
      return response.json();
    } else {
      throw new Error(items);
    }
  }

  const getTopItems = () => {
    top_promise = fetchTopItems();
    promise = Promise.resolve([]);

  }


</script>

<main>
  <nav class="flex items-center justify-between flex-wrap bg-white py-4 shadow border-solid border-t-2 border-blue-700">
    <div class="flex justify-between lg:w-auto w-full lg:border-b-0 border-solid border-b-2 border-gray-300 pb-5 lg:pb-0">
      <div class="flex items-center flex-shrink-0 text-gray-800 mr-16">
        <span class="font-semibold text-xl tracking-tight px-2">Sananmuunnos</span>
      </div>
    </div>
      
    <div class="menu w-full lg:block flex-grow lg:flex lg:items-center lg:w-auto lg:px-3 px-8">
      <div class="text-md font-bold text-blue-700 lg:flex-grow">
        <a href={"#"}
          on:click|preventDefault={getTopItems}
          class="block mt-4 lg:inline-block lg:mt-0 hover:text-white px-4 py-2 rounded hover:bg-blue-700 mr-2">
          Parhaat
        </a>
      </div>
      <div class="relative mx-auto text-gray-600 lg:block">
        <input
            bind:value={word} on:change={getItems} class="border-2 border-gray-300 bg-white h-10 pl-2 pr-8 rounded-lg text-sm focus:outline-none"
            type="search" name="search" placeholder="Ruma sana"/>
        <button type="submit" class="absolute right-0 top-0 mt-3 mr-2">
          <svg class="text-gray-600 h-4 w-4 fill-current" xmlns="http://www.w3.org/2000/svg"
                version="1.1" id="Capa_1" x="0px" y="0px"
                viewBox="0 0 56.966 56.966" style="enable-background:new 0 0 56.966 56.966;"
                xml:space="preserve"
                width="512px" height="512px">
            <path
                d="M55.146,51.887L41.588,37.786c3.486-4.144,5.396-9.358,5.396-14.786c0-12.682-10.318-23-23-23s-23,10.318-23,23  s10.318,23,23,23c4.761,0,9.298-1.436,13.177-4.162l13.661,14.208c0.571,0.593,1.339,0.92,2.162,0.92  c0.779,0,1.518-0.297,2.079-0.837C56.255,54.982,56.293,53.08,55.146,51.887z M23.984,6c9.374,0,17,7.626,17,17s-7.626,17-17,17  s-17-7.626-17-17S14.61,6,23.984,6z"/>
          </svg>
        </button>
      </div>
      <div class="flex ">
        <a href="https://github.com/mossman/sananmuunnos"
            class=" block text-md px-4  ml-2 py-2 rounded text-blue-700 font-bold hover:text-white mt-4 hover:bg-blue-700 lg:mt-0">
            github
        </a>
      </div>
    </div>
      
  </nav>
  <div>
    <div class="flex flex-col">
      <div class="">
          <div class="inline-block py-2 min-w-full">
              <div class="overflow-hidden shadow-md sm:rounded-lg">
                  <table class="min-w-full">
                      <thead class="bg-gray-50">
                          <tr>
                              <th scope="col" class="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase">
                                  Sana 1
                              </th>
                              <th scope="col" class="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase">
                                  Sana 2
                              </th>
                              <th scope="col" class="py-3 px-6 text-xs font-medium tracking-wider text-right text-gray-700 uppercase">
                                  Peukutukset
                              </th>
                          </tr>
                      </thead>
                      <tbody>
                        {#await promise}
                          <div class="content-center"><BarLoader size=60></BarLoader></div>
                        {:then items}
                          {#each items as item}
                        <FirstWord word={item.rootword}/>
                        {#each item.endings as ending}
                        <SecondWord firstword={item.rootword} secondword={ending} failed={false} liked={false}/>
                          {/each}
                        {/each}
                      {:catch error}
                        <p>{error.message}</p>
                      {/await}
                      {#await top_promise}
                      <div class="content-center"><BarLoader size=60></BarLoader></div>
                      {:then items}
                      {#each items as item}
                      <TopWord firstword={item.first} secondword={item.second} count={item.count} liked={false} failed={false}/>
                      {/each}
                    {:catch error}
                      <p>{error.message}</p>
                    {/await}


                    </tbody>
                  </table>
              </div>
          </div>
      </div>
  </div>

</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    padding: 0em;
    margin: 0 auto;
  }

  

</style>
