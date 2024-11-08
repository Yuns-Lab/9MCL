<script setup lang="ts">
    const props = defineProps<{
        clientName: string;
    }>();
</script>

<template>
    <button
        class="yun-launcher-button"
        tabindex="-1">
        <span id="launch">启动游戏</span>
        <span id="client-name">{{ props.clientName }}</span>
    </button>
</template>

<style scoped lang="scss">
    button.yun-launcher-button {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 8px;
        border: 1px solid #081c0e;
        background-color: #f8f8f9;
        border-radius: 4px;
        padding: 12px 6px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        
        &:hover {
            background-color: #f2f2f2;
        }
        
        &:active {
            transform: scale(0.97);
        }
        
        span#launch {
            font-size: 1.5em;
        }
        
        span#client-name {
            font-size: 0.8em;
            opacity: 0.75;
        }
    }
</style>
