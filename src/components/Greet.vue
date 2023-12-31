<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

const greetMsg = ref('')
const name = ref('')

async function greet() {
    greetMsg.value = await invoke('greet', { name: name.value })
}
</script>

<template>
    <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
    </form>

    <strong>Typography:</strong>
    <h1>h1</h1>
    <h2>h2</h2>
    <h3>h3</h3>
    <h4>h4</h4>
    <h5>h5</h5>
    <h6>h6</h6>
    <p>p normal</p>
</template>
