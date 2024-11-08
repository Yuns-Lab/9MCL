<script setup lang="ts">
    const props = defineProps<{ hoverColor: string; showTop: string; showLeft: string }>();
</script>

<template>
    <button
        class="yun-skin-button"
        :style="{ '--hover-color': props.hoverColor, '--show-top': props.showTop, '--show-left': props.showLeft }">
        <slot></slot>
    </button>
</template>

<style lang="scss" scoped>
    button.yun-skin-button {
        background-color: #f8f8f9;
        border-radius: 99px;
        border: 1px solid #081c0e;
        width: 32px;
        height: 32px;
        position: absolute;
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        transition: all 0.2s ease-in-out;
        z-index: -1;
        opacity: 0;
        
        &:hover {
            background-color: #f2f2f2;
        }
        
        &:active {
            transform: translate(-50%, -50%) scale(0.97);
        }
        
        &.show {
            left: var(--show-left);
            top: var(--show-top);
            opacity: 1;
            z-index: 1;
        }
    }
</style>
