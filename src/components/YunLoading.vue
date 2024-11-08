<script setup lang="ts">
    import YunCard from "./YunCard.vue";
    import YunLoadingCube from "./YunLoadingCube.vue";
    
    const props = defineProps<{
        text: string;
        isfailed?: boolean;
    }>();
</script>

<template>
    <YunCard class="yun-loading">
        <YunLoadingCube
            :hex-color="props.isfailed ? '#e06c60' : '#081c0e'"
            :pause="props.isfailed" />
        <span class="yun-loading__text">{{ props.text }}</span>
    </YunCard>
</template>

<style lang="scss" scoped>
    .yun-loading {
        width: 220px;
        height: 130px;
        display: flex;
        align-self: center;
        justify-content: center;
        align-items: center;
        margin: auto;
        flex-direction: column;
        gap: 16px;
        
        span.yun-loading__text {
            font-size: 1.3em;
        }
    }
</style>
