<script setup lang="ts">
    import YunButton from "@/components/YunButton.vue";
    
    const props = defineProps<{
        name: string,
        right: string,
        license: string,
        sourceUrl: string,
        is3rdSource: boolean,
        trustDocUrl?: string,
    }>();
    
    const openWebsite = (url: string) => {
        if (!url) return;
        window.open(url, "_blank");
    }
</script>

<template>
    <div class="yun3rd-party-comp">
        <li>{{ props.name }}</li>
        <div class="yun3rd-party-comp__right">
            <p>Copyright © {{ props.right }}</p>
            <p>Licensed under the {{ props.license }}.</p>
            <div class="yun3rd-party-comp__right-buttons">
                <YunButton @click="openWebsite(props.sourceUrl)">查看{{ props.is3rdSource? "来源" : "原" }}网站</YunButton>
                <YunButton v-if="props.trustDocUrl" @click="openWebsite(props.trustDocUrl)">查看许可文档</YunButton>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
    div.yun3rd-party-comp {
        display: flex;
        width: 100%;
        padding-left: 16px;
        gap: 32px;
        li {
            font-weight: bold;
        }
        div.yun3rd-party-comp__right {
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: flex-start;
            p {
                margin: 2px 0;
                font-size: 0.95em;
            }
            div.yun3rd-party-comp__right-buttons {
                margin-top: 4px;
                display: flex;
                gap: 16px;
            }
        }
    }
</style>