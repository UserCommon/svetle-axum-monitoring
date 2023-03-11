<script>
    import { browser } from '$app/environment';
    import { onMount } from 'svelte/internal';
    let json;
    let used_memory = 0;
    let memory_percentage = 0;
    let cpus_usage = [0,0,0];
    let total_memory = 0;

    let swap_percentage = 0;
    let total_swap = 0;
    let used_swap = 0;

    

    if(browser) {
        const socket = new WebSocket("ws://127.0.0.1:8000/api/sys");

        socket.addEventListener("open", () => {
            console.log("opened");
        })
        socket.onmessage = (event) => {
            json = JSON.parse(event.data);
            //ram
            used_memory = parseFloat((json.used_memory / 2**20).toFixed(1));
            total_memory = parseFloat((json.total_memory / 2**20).toFixed(1));
            memory_percentage = parseFloat((used_memory/total_memory*100).toFixed(1));

            //swap
            used_swap = parseFloat((json.used_swap / 2 **20).toFixed(1));
            total_swap = parseFloat((json.total_swap / 2 **20).toFixed(1));
            swap_percentage = parseFloat((used_memory/total_memory).toFixed(1));
            //cpu
            cpus_usage      = json.cpus_usage.map(elem => elem.toFixed(1));
        }
    }

    function load_bar(variable) {
        if(variable < 40.0) return "bg-green-600 dark:bg-green-500";
        else if(variable > 40.0 && variable < 75.0) return "bg-yellow-600  dark:bg-yellow-500";
        else if(variable > 75.0) return "bg-red-600 dark:bg-red-500";
    };
    
</script>

<style>
    .percentage {
        border: solid black 1px;
        border-radius: 25px;
        margin-bottom: 1%;
    }
    .margined {
        margin: 1%;
    }
    .crop {
        /*left: var(--percentage);
        */
        width: var(--percentage);
        background-color: red;
    }

    .page {
        height: 100%;
        width: 100%;
    }

</style>


<!--
<div class="cpu-box flex flex-wrap gap-3 w-full bg-blue-600 rounded-xl m-10  p-2 w-2/5">
    {#each cpus_usage as cpu, i }
    <div class="flex-row basis-1/4 ">
        <h1 class="font-bold">CPU #{i+1}: {cpu}%</h1>
        <div class="h-4 bg-gray-200 rounded-full dark:bg-gray-700 m-2 flex-1">
            <div class="{load_bar(cpu)} h-4 rounded-full " style="width: {cpu}%">
            </div>
          </div>
        </div>
    {/each}
</div>
-->


<div class="cpu-box bg-blue-600 rounded-xl m-10  p-2 grid grid-cols-4 gap-4 ">
    <div class="font-bold col-span-full text-3xl">System</div>
    {#each cpus_usage as cpu, i }
    <div class="">
        <h1 class="font-bold">CPU #{i+1}: {cpu}%</h1>
        <div class="h-4 bg-gray-200 rounded-full dark:bg-gray-700 m-2">
            <div class="{load_bar(cpu)} h-4 rounded-full " style="width: {cpu}%">
            </div>
          </div>
        </div>
    {/each}
    <div class="col-span-full">
        <div class="font-bold h-7">Memory Used:  {memory_percentage}% {used_memory}/{total_memory} MiB</div>
        <div class="w-full h-4 bg-gray-200 rounded-full dark:bg-gray-700">
            
            <div class="h-4 rounded-full {load_bar(memory_percentage)}" style="width: {memory_percentage}%"></div>
        
          </div>
    </div>

    <div class="col-span-full">
        <div class="font-bold h-7">Swap Used:  {swap_percentage}% {used_swap}/{total_swap} MiB</div>
        <div class="w-full h-4 bg-gray-200 rounded-full dark:bg-gray-700">
            
            <div class="h-4 rounded-full {load_bar(swap_percentage)}" style="width: {swap_percentage}%"></div>
        
          </div>
    </div>
</div>


<body class="bg-blue-300 page">


</body>