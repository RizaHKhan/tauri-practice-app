<template>
    <div class="input">
        <label :for="id" class="input__label">{{ label }}</label>
        <input v-model="model" :type="type" :placeholder="placeholder" class="input__input" />
    </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
interface IProps {
    name?: string
    label?: string
    modelValue: string
    placeholder?: string
    email?: boolean
    password?: boolean
}

const emit = defineEmits(['update:modelValue'])
const props = withDefaults(defineProps<IProps>(), {
    placeholder: '',
    name: '',
    label: '',
    email: false,
    password: false,
})

const model = computed({
    get: () => props.modelValue,
    set: (val) => emit('update:modelValue', val),
})

const type = computed(() => {
    if (props.email) {
        return 'email'
    }

    if (props.password) {
        return 'password'
    }

    return 'text'
})
</script>

<style lang="scss">
:root {
    --label-color: #{$wenge};

    --input-border-radius: #{$radius};
    --input-border: 1px solid transparent;
    --input-padding: 0.6em 1.2em;
    --input-font-size: 1em;
    --input-font-weight: 500;
    --input-font-family: inherit;
    --input-color: $white;
    --input-background-color: #{$wenge};
    --input-transition: all 0.25s;
    --input-box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

label {
    color: var(--label-color);
}

input {
    border-radius: var(--input-border-radius);
    border: var(--input-border);
    padding: var(--input-padding);
    font-size: var(--input-font-size);
    font-weight: var(--input-font-weight);
    font-family: var(--input-font-family);
    color: var(--input-color);
    background-color: var(--input-background-color);
    transition: var(--input-transition);
    box-shadow: var(--input-box-shadow);
}

.input {
    @include flex-container(column, flex-start, xs);

    &__label {
    }
    &__input {
        &:focus {
            border: solid 1px red;
        }
    }
}
</style>
