<template>
    <div class="sidebar">
        <component :is="icon" @click="toggleSidebar" size="40" />
        <div
            class="sidebar__content"
            :class="{
                shown,
            }"
        >
            <slot></slot>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import MenuIcon from 'vue-material-design-icons/Menu.vue'
import CloseBox from 'vue-material-design-icons/CloseBox.vue'

const shown = ref(false)

const toggleSidebar = () => {
    shown.value = !shown.value
}

const icon = computed(() => {
    if (shown.value) {
        return CloseBox
    }

    return MenuIcon
})
</script>

<style lang="scss" scoped>
.sidebar {
    &__activator {
    }

    &__content {
        @include flex-container(column);
        @include margin(md, top);
        transition: width 0.25s ease;
        width: 0;
        overflow: hidden;

        &.shown {
            width: 200px;
            overflow: visible;
        }
    }
}
</style>
