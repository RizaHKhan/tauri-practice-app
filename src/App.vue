<template>
    <div class="app">
        <Sidebar>
            <Button v-for="({ component, text }, key) in actions" :key="key" @click="setComponent(component)">
                {{ text }}</Button
            >
        </Sidebar>
        <component :is="components[component]" />
    </div>
</template>

<script setup lang="ts">
import { ref, computed, defineAsyncComponent } from 'vue'
import Sidebar from 'atoms/Sidebar.vue'
import Button from 'atoms/Button.vue'

const components = computed(() => ({
    style: defineAsyncComponent(() => import('views/Style.vue')),
    tasks: defineAsyncComponent(() => import('views/Tasks.vue')),
}))

const actions = computed(() => [
    {
        component: 'tasks',
        text: 'Tasks',
    },
    {
        component: 'style',
        text: 'Style',
    },
])

const component = ref(Object.keys(components.value)[0])

const setComponent = (key: string) => {
    component.value = key
}
</script>

<style lang="scss" scoped>
.app {
    @include flex-container(row);
    @include padding(md);
}

.cards {
    @include grid-container(1fr 1fr);
}
</style>
