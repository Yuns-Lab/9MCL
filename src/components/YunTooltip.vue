<script setup lang="ts">
    const props = defineProps<{
        tooltip: string;
        direction: "x" | "y";
        position: "top" | "bottom" | "left" | "right";
    }>();
</script>

<template>
    <div class="yun-tooltip">
        <!-- <span
            class="yun-tooltip__content"
            :class="[props.position, props.direction]">
            {{ props.tooltip }}
        </span> -->
        <div class="yun-tooltip__real">
            <slot></slot>
        </div>
    </div>
</template>

<style lang="scss">
    div.yun-tooltip {
        position: relative;
        
        span.yun-tooltip__content {
            position: absolute;
            background-color: white;
            border: 1px solid black;
            border-radius: 4px;
            padding: 1px;
            transform: translateY(-100%);
        }
        
        div.yun-tooltip__real {
            position: absolute;
            top: 0;
        }
    }
</style>
