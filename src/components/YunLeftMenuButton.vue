<script setup lang="ts">
    import IconArrowRight from "@/icons/IconArrowRight.vue";
    
    const props = defineProps<{
        tabIndex?: number;
        hightlight?: boolean;
    }>();
</script>

<template>
    <button
        class="yun-left-menu-button"
        :class="{ 'yun-left-menu-button--highlight': props.hightlight }"
        :tabindex="props.tabIndex || 0">
        <div class="yun-left-menu-button__content">
            <slot></slot>
        </div>
        <IconArrowRight
            :size="20"
            style="margin-left: auto" />
    </button>
</template>

<style scoped lang="scss">
    button.yun-left-menu-button {
        background-color: transparent;
        border: none;
        color: #081c0e;
        cursor: pointer;
        border-radius: 4px;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 40px;
        transition: background-color 0.15s ease-in-out;
        width: 85%;
        margin-left: auto;
        margin-right: 8px;
        font-size: 13px;
        
        &:hover {
            background-color: rgba(8, 28, 14, 0.1);
        }
        
        &.yun-left-menu-button--highlight {
            background-color: rgba(29, 103, 51, 0.2);
        }
        
        div.yun-left-menu-button__content {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 4px;
            margin-left: 4px;
        }
    }
</style>
