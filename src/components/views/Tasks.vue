<template>
    <View title="Tasks">
        <div class="tasks">
            <Card v-for="(task, key) in tasks" :key="key" :title="task.title" class="tasks__task">
                <p>
                    {{ task.description }}
                </p>
                <template #footer>
                    <Button warn @click="removeTask">Remove</Button>
                </template>
            </Card>
        </div>
    </View>
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { onMounted } from 'vue'
import Card from 'atoms/Card.vue'
import View from 'atoms/View.vue'
import Button from 'atoms/Button.vue'

interface Task {
    title: string
    description: string
}

const tasks: Ref<Task[] | []> = ref([])

onMounted(async () => {
    tasks.value = await invoke('get_tasks')
})

const removeTask = () => {}
</script>

<style lang="scss" scoped>
.tasks {
    @include grid-container(1fr 1fr 1fr);
}
</style>
