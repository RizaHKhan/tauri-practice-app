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

<template>
    <div class="container">
        <Tabs :tabs="tabs">
            <template #content-style>
                <Style />
            </template>
        </Tabs>
    </div>
</template>
