<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import Button from 'atoms/Button.vue'
import Input from 'atoms/Input.vue'

const greetMsg = ref('')
const name = ref('')

async function greet() {
    greetMsg.value = await invoke('greet', { name: name.value })
}

const input = ref('')
</script>

<template>
    <div class="container">
        <form class="row" @submit.prevent="greet">
            <Input v-model="name" placeholder="Enter a name..." />
            <button type="submit">Greet</button>
        </form>

        <div class="typography">
            <strong>Typography:</strong>
            <div class="content">
                <h1>h1</h1>
                <h2>h2</h2>
                <h3>h3</h3>
                <h4>h4</h4>
                <h5>h5</h5>
                <p>p</p>
            </div>
        </div>

        <div class="buttons">
            <strong>Buttons:</strong>
            <div class="content">
                <Button>Primary</Button>
                <Button secondary>Secondary</Button>
                <Button warn>Primary</Button>
            </div>

            <div class="content">
                <Button disabled>Primary</Button>
                <Button disabled secondary>Secondary</Button>
                <Button disabled warn>Primary</Button>
            </div>

            <div class="content">
                <Button secondary sm>Secondary</Button>
                <Button warn sm>Warn</Button>
                <Button warn sm>Warn</Button>
            </div>

            <div class="content">
                <Button secondary lg>Secondary</Button>
                <Button warn lg>Warn</Button>
                <Button warn lg>Warn</Button>
            </div>
        </div>
        <div class="inputs">
            {{ input }}
            <div class="form">
                <Input v-model="input" label="Name" placeholder="foo" />
                <Input v-model="input" email label="Email" placeholder="foo" />
                <Input v-model="input" label="Password" password placeholder="Password" />
            </div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
.container {
    @include padding(md);
}
.content {
    @include flex-container(row);
    @include margin(md, bottom);
}

.form {
    @include grid-container(1fr 1fr, auto, sm);
}
</style>
