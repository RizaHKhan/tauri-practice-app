<template>
    <button :class="[variant, size]" :disabled="disabled" @click="emit('click')">
        <slot />
    </button>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
const emit = defineEmits(['click'])

interface IProps {
    secondary?: boolean
    warn?: boolean
    disabled?: boolean
    sm?: false
    lg?: false
}

const props = withDefaults(defineProps<IProps>(), {
    secondary: false,
    warn: false,
    disabled: false,
    sm: false,
    lg: false,
})

const variant = computed(() => {
    if (props.secondary) {
        return 'secondary'
    }

    if (props.warn) {
        return 'warn'
    }

    return 'primary'
})

const size = computed(() => {
    if (props.sm) {
        return 'sm'
    }

    if (props.lg) {
        return 'lg'
    }

    return 'md'
})
</script>

<style lang="scss">
:root {
    --button-border-radius: #{$radius};
    --button-border: 1px solid transparent;
    --button-padding: 10px 20px;
    --button-font-size: 16px;
    --button-font-weight: 500;
    --button-font-family: inherit;
    --button-color: #{$eerie-black};
    --button-opacity: 0.8;
    --button-background-color: #{$robin-egg-blue};
    --button-transition: all 0.25s;
    --button-box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
    border-radius: var(--button-border-radius);
    border: var(--button-border);
    padding: var(--button-padding);
    font-size: var(--button-font-size);
    font-weight: var(--button-font-weight);
    font-family: var(--button-font-family);
    color: var(--button-color);
    opacity: var(--button-opacity);
    background-color: var(--button-background-color);
    transition: var(--button-transition);
    box-shadow: var(--button-box-shadow);

    &:not(:disabled) {
        cursor: pointer;
    }

    &:disabled {
        opacity: 0.3;
    }

    &:hover {
        --button-opacity: 1;
    }

    &:active {
        --button-opacity: 0.5;
    }

    &.secondary {
        --button-color: white;
        --button-background-color: #{$wenge};
    }

    &.warn {
        --button-background-color: #{$sunglow};
    }

    &.lg {
        --button-padding: 20px 30px;
        --button-font-size: 20px;
    }

    &.sm {
        --button-padding: 5px 10px;
        --button-font-size: 13px;
    }
}
</style>
