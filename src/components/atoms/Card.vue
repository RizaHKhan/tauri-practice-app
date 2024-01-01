<template>
    <div class="card">
        <div v-if="hasHeader" class="card__header">
            <slot name="header">
                <h4>{{ title }}</h4>
            </slot>
        </div>
        <div class="card__content">
            <slot></slot>
        </div>
        <div v-if="hasFooter" class="card__footer">
            <slot name="footer"></slot>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { computed, useSlots } from 'vue'

interface IProps {
    title?: string
}

const slots = useSlots()

const props = withDefaults(defineProps<IProps>(), {
    title: '',
})

const hasHeader = computed(() => !!slots.header || !!props.title)
const hasFooter = computed(() => !!slots.footer)
</script>

<style lang="scss">
:root {
    --card--border-radius: #{$radius};
    --card--primary-border-color: #{$cordovan};
    --card--secondary-border-color: #{$wenge};
}

.card {
    border-radius: var(--card--border-radius);
    border: solid 1px var(--card--primary-border-color);
    @include padding(md, left);
    @include padding(md, right);

    > div {
        @include padding(md, top);
        @include padding(md, bottom);
    }

    &__header {
        border-bottom: solid 1px var(--card--secondary-border-color);
    }
    &__content {
        border-bottom: solid 1px var(--card--secondary-border-color);
    }
    &__footer {
        @include flex-container(row, flex-end, sm);
    }
}
</style>
