<script setup lang="ts">
    const props = defineProps<{
        tabIndex?: number;
        hightlight?: boolean;
    }>();
</script>

<template>
    <button
        class="yun-icon-button"
        :class="{ 'yun-icon-button--highlight': props.hightlight }"
        :tabindex="props.tabIndex || 0">
        <slot></slot>
    </button>
</template>

<style scoped lang="scss">
    button.yun-icon-button {
        background-color: transparent;
        border: none;
        color: #f8f8f9;
        cursor: pointer;
        border-radius: 40px;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 30px;
        transition: background-color 0.15s ease-in-out;
        
        &:hover {
            background-color: rgba(255, 255, 255, 0.2);
        }
        
        &.yun-icon-button--highlight {
            background-color: #f8f8f9;
            color: #081c0e;
        }
    }
</style>
