<script setup lang="ts">
    const props = defineProps<{
        image: string;
    }>();
</script>

<template>
    <div class="yun-author-card">
        <img
            :src="props.image"
            class="yun-author-card__image"
            width="80" />
        <div class="yun-author-card__info">
            <div class="yun-author-card__container">
                <slot name="author"></slot>
            </div>
            <hr width="100%" />
            <div class="yun-author-card__container" style="margin-left: auto; margin-right: 16px;">
                <slot name="tags"></slot>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
    div.yun-author-card {
        background-color: #edeff0;
        border: 1px solid rgba(8, 28, 14, 0.3);
        padding: 4px 16px;
        border-radius: 8px;
        overflow: hidden;
        transition: all 0.2s ease-in-out;
        box-shadow: 2px 2px 6px rgba(8, 28, 14, 0.15);
        height: 100px;
        width: 90%;
        display: flex;
        align-items: center;
        gap: 16px;
        margin: 0 auto;
        
        img.yun-author-card__image {
            border-radius: 8px;
            box-shadow: 0px 0px 12px 4px rgba(0, 0, 0, 0.25);
        }
        
        div.yun-author-card__info {
            display: flex;
            flex-direction: column;
            width: 100%;
            padding: 0 8px 0 4px;
            
            div.yun-author-card__container {
                display: flex;
                gap: 4px;
                align-items: flex-end;
            }
        }
    }
</style>
