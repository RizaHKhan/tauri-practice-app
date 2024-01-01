<template>
    <div>
        <pre>{{ tasks }}</pre>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { onMounted } from 'vue'

const tasks = ref([])

onMounted(() => {})
</script>
