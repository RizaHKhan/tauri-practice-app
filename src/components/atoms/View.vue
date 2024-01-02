<template>
    <div class="view">
        <div class="view__header">
            <slot name="header">
                <h4>{{ title }}</h4>
            </slot>
        </div>

        <div class="view__content">
            <slot></slot>
        </div>
    </div>
</template>

<script setup lang="ts">
interface IProps {
    title?: string
}

withDefaults(defineProps<IProps>(), {
    title: '',
})
</script>

<style lang="scss" scoped>
.view {
    width: 100%;
    @include flex-container(column, flex-start, lg);
    &__header {
        border-bottom: 1px solid red;
    }
}
</style>
