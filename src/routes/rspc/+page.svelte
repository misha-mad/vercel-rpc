<script lang="ts">
    import { client } from "$lib/client";
    import { onMount } from "svelte";

    let version = $state("loading...");
    let name = $state("SvelteKit");
    let greeting = $state("");
    let loading = $state(false);

    async function fetchVersion() {
        try {
            version = await client.query(["version"]);
        } catch (e) {
            version = "Error: " + e;
        }
    }

    async function sayHello() {
        loading = true;
        try {
            greeting = await client.query(["hello", name]);
        } catch (e) {
            greeting = "Error: " + e;
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        fetchVersion();
    });
</script>

<div class="container">
    <h1>RSPC + SvelteKit + Rust</h1>
    <p>Backend version: <strong>{version}</strong></p>

    <div class="card">
        <h2>Type-safe Query</h2>
        <input type="text" bind:value={name} placeholder="Enter your name" />
        <button onclick={sayHello} disabled={loading}>
            {loading ? 'Sending...' : 'Say Hello'}
        </button>
        
        {#if greeting}
            <div class="result">
                {greeting}
            </div>
        {/if}
    </div>
</div>

<style>
    .container {
        max-width: 600px;
        margin: 2rem auto;
        padding: 1rem;
        font-family: sans-serif;
    }
    .card {
        border: 1px solid #ccc;
        padding: 1rem;
        border-radius: 8px;
        background: #f9f9f9;
    }
    input {
        padding: 0.5rem;
        margin-right: 0.5rem;
    }
    button {
        padding: 0.5rem 1rem;
        cursor: pointer;
    }
    .result {
        margin-top: 1rem;
        padding: 0.5rem;
        background: #e0f0e0;
        border-left: 4px solid #4caf50;
    }
</style>
