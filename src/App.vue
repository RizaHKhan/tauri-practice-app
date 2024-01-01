<template>
    <div class="app">
        <Tabs :tabs="tabs">
            <template #content-style>
                <Style />
            </template>
        </Tabs>
    </div>
</template>

<script setup lang="ts">
import { Tab } from 'types'
import Style from 'views/Style.vue'
import Tabs from 'atoms/Tabs.vue'

const tabs: Tab[] = [
    {
        name: 'style',
        title: 'Style',
    },
    {
        name: 'other',
        title: 'Other',
    },
]
</script>

<style lang="scss" scoped>
.app {
    @include padding(md);
}
</style>
