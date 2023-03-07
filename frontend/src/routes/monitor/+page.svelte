<script>
    import { browser } from '$app/environment';
    let json;
    let used_memory = 0;
    let total_memory=0; // Remove to static api pls
    let memory_percentage = 0;
    let cpus_usage = [0,0,0];

    if(browser) {
        const socket = new WebSocket("ws://127.0.0.1:8000/api/sys");

        

        socket.addEventListener("open", () => {
            console.log("opened");
        })
        socket.onmessage = (event) => {
            json = JSON.parse(event.data);
            
            used_memory = json.used_memory / 2**20;
            total_memory = json.total_memory / 2**20;
            cpus_usage      = json.cpus_usage;
            memory_percentage = used_memory/total_memory*100;
            console.log(json)

        }
    }

</script>

<style>
    
</style>

<p>Memory used in %: {memory_percentage}</p>
<p>{used_memory}</p>
<p>{total_memory}</p>

<div class="box">
    {#each cpus_usage as cpu, i }
        <p>CPU #{i}: {cpu}</p>
    {/each}
</div>
