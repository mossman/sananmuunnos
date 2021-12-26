<script lang="ts">

  let word = '';

  let promise = Promise.resolve([]);

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
  }

  const onKeyPress = e => {
    if (e.charCode === 13) getItems();
  };

</script>

<main>
<nav class="flex items-center justify-between flex-wrap bg-white py-4 lg:px-12 shadow border-solid border-t-2 border-blue-700">
        <div class="flex justify-between lg:w-auto w-full lg:border-b-0 pl-6 pr-2 border-solid border-b-2 border-gray-300 pb-5 lg:pb-0">
            <div class="flex items-center flex-shrink-0 text-gray-800 mr-16">
                <span class="font-semibold text-xl tracking-tight">Sananmuunnos</span>
            </div>
        </div>
    
        <div class="menu w-full lg:block flex-grow lg:flex lg:items-center lg:w-auto lg:px-3 px-8">
            <div class="text-md font-bold text-blue-700 lg:flex-grow">
                <a href="#responsive-header"
                   class="block mt-4 lg:inline-block lg:mt-0 hover:text-white px-4 py-2 rounded hover:bg-blue-700 mr-2">
                    Parhaat
                </a>
            </div>
            <!-- This is an example component -->
            <div class="relative mx-auto text-gray-600 lg:block">
                <input
                    bind:value={word} on:keypress={onKeyPress} class="border-2 border-gray-300 bg-white h-10 pl-2 pr-8 rounded-lg text-sm focus:outline-none"
                    type="search" name="search" placeholder="Ruma sana">
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
                   class=" block text-md px-4  ml-2 py-2 rounded text-blue-700 font-bold hover:text-white mt-4 hover:bg-blue-700 lg:mt-0">github</a>
            </div>
        </div>
    
  </nav>
  <div>
    <div class="flex flex-col">
      <div class="overflow-x-auto sm:-mx-6 lg:-mx-8">
          <div class="inline-block py-2 min-w-full sm:px-6 lg:px-8">
              <div class="overflow-hidden shadow-md sm:rounded-lg">
                  <table class="min-w-full">
                      <thead class="bg-gray-50 dark:bg-gray-700">
                          <tr>
                              <th scope="col" class="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-400">
                                  Sana 1
                              </th>
                              <th scope="col" class="py-3 px-6 text-xs font-medium tracking-wider text-left text-gray-700 uppercase dark:text-gray-400">
                                  Sana 2
                              </th>
                              <th scope="col" class="relative py-3 px-6">
                                  <span class="sr-only">Peukuta</span>
                              </th>
                          </tr>
                      </thead>
                      <tbody>
                        {#await promise}
                        <p>Ruksuti ruksuti...</p>
                        {:then items}
                        {#each items as item}
                        <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                          <td class="py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap dark:text-white">
                              {item.rootword}
                          </td>
                          <td class="py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400">
                          </td>
                          <td class="py-4 px-6 text-sm font-medium text-right whitespace-nowrap">
                          </td>
                        </tr>
                          {#each item.endings as ending}
                          <tr class="bg-white border-b dark:bg-gray-800 dark:border-gray-700">
                            <td class="py-4 px-6 text-sm font-medium text-gray-900 whitespace-nowrap dark:text-white">
                            </td>
                            <td class="py-4 px-6 text-sm text-gray-500 whitespace-nowrap dark:text-gray-400">
                                {ending}
                            </td>
                            <td class="py-4 px-6 text-sm font-medium text-right whitespace-nowrap">
                                <a href="#" class="text-blue-600 hover:text-blue-900 dark:text-blue-500 dark:hover:underline">Peukkua</a>
                            </td>
                        </tr>

                          {/each}
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
    padding: 1em;
    margin: 0 auto;
  }

  

</style>
