<script setup lang="ts">
    import IconArrowRight from "@/icons/IconArrowRight.vue";
    import { ref } from "vue";
    
    const props = defineProps<{
        title?: string;
        canSwap?: boolean;
        isSwapped?: boolean;
    }>();
    
    const yuncard = ref<HTMLDivElement>();
    const isCardOpen = ref<boolean>(
        props.canSwap ? (props.isSwapped ? false : true) : true
    );
    const toggleCard = () => {
        if (!props.canSwap || yuncard.value == null) return;
        isCardOpen.value = !isCardOpen.value;
        switch (isCardOpen.value) {
            case true:
                yuncard.value.style.height = "auto";
                const { height } = yuncard.value.getBoundingClientRect();
                yuncard.value.style.height = "23px";
                yuncard.value.offsetHeight; // force reflow
                yuncard.value.style.height = `${height - 23}px`;
                break;
            case false:
                yuncard.value.style.height = "23px";
        }
    };
</script>

<template>
    <div
        class="yun-card"
        :class="{
            swapped: !isCardOpen,
        }"
        ref="yuncard">
        <div
            class="yun-card__top"
            :class="{
                canSwap: props.canSwap,
            }"
            v-if="props.title || props.canSwap"
            @click="toggleCard">
            <span>{{ props.title }}</span>
            <IconArrowRight
                :size="24"
                v-if="props.canSwap"
                :class="{
                    swapped: isCardOpen,
                }"
                class="yun-card__top-icon" />
        </div>
        <slot></slot>
    </div>
</template>

<style scoped lang="scss">
    div.yun-card {
        background-color: #f8f8f9;
        padding: 12px;
        border-radius: 8px;
        overflow: hidden;
        transition: all 0.2s ease-in-out;
        box-shadow: 2px 2px 6px rgba(8, 28, 14, 0.15);
        
        div.yun-card__top {
            display: flex;
            margin-bottom: 12px;
            
            .yun-card__top-icon {
                margin-left: auto;
                transition: all 0.3s cubic-bezier(0.27, -0.51, 0.74, 1.51);
                transform: translateY(-1px) rotate(90deg);
            }
            
            span {
                font-weight: bold;
                color: #081c0e;
                transform: translateY(2px);
            }
            
            &.canSwap {
                cursor: pointer;
            }
        }
        
        &.swapped {
            height: 23px;
        }
        
        div.yun-card__top .yun-card__top-icon.swapped {
            transform: translateY(-1px) rotate(-90deg);
        }
    }
</style>
