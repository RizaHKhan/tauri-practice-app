<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import Button from './atoms/Button.vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
    greetMsg.value = await invoke('greet', { name: name.value })
}

const t = 'primary'
</script>

<template>
    <div class="container">
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
        <p>p</p>

        <div class="buttons">
            <strong>Buttons:</strong>
            <Button>Primyar</Button>
            <Button variant="secondary">Secondary</Button>
            <Button variant="warn">Warn</Button>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.buttons {
    @include flex-container(column, space-between, flex-start, wrap);
    @include gap(m);
}
</style>
