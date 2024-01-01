<template>
    <div class="tabs">
        <div class="tabs__header">
            <slot
                v-for="({ name, title }, key) in tabs"
                :key="key"
                :name="`header-${name}`"
                v-bind="{ name, title, selectTab, selectedTab }"
            >
                <Button @click="selectTab(name)" :secondary="selectedTab !== name">{{ title }}</Button>
            </slot>
        </div>
        <div class="tabs__content">
            <slot :name="`content-${selectedTab}`"></slot>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { Tab } from 'types'
import Button from 'atoms/Button.vue'

interface IProps {
    tabs: Tab[]
}

const props = withDefaults(defineProps<IProps>(), {})

const selectedTab = ref(props.tabs[0].name)

const selectTab = (name: string) => {
    selectedTab.value = name
}
</script>

<style lang="scss" scoped>
.tabs {
    &__header {
        @include flex-container(row, flex-start);
        @include padding(md, bottom);
        border-bottom: 1px solid #{$cordovan};
    }
    &__content {
        @include margin(md, top);
    }
}
</style>
