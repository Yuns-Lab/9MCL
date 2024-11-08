<script setup lang="ts">
    const props = defineProps<{
        type?: "button" | "submit" | "reset";
    }>();
</script>

<template>
    <button
        class="yun-button"
        :type="props.type ?? `button`">
        <slot></slot>
    </button>
</template>

<style scoped lang="scss">
    button.yun-button {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 8px;
        border: 1px solid #081c0e;
        background-color: #f8f8f9;
        border-radius: 4px;
        padding: 8px 18px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        &:hover {
            background-color: #f2f2f2;
        }
        &:active {
            transform: scale(0.97);
        }
    }
</style>
