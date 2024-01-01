<template>
    <button :class="[variant]" @click="emit('click')">
        <slot />
    </button>
</template>

<script lang="ts" setup>
const emit = defineEmits(['click'])

interface IProps {
    variant?: 'primary' | 'secondary' | 'warn'
}

withDefaults(defineProps<IProps>(), {
    variant: 'primary',
})
</script>
