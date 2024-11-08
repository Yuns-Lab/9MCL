<script setup lang="ts">
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunLoading from "@/components/YunLoading.vue";
    // Icons
    import IconDownloadAuto from "@/icons/IconDownloadAuto.vue";
    import IconDownloadManually from "@/icons/IconDownloadManually.vue";
    import IconDownloadMod from "@/icons/IconDownloadMod.vue";
    import IconDownloadModpack from "@/icons/IconDownloadModpack.vue";
    import IconDownloadAddon from "@/icons/IconDownloadAddon.vue";
    
    const route = useRoute();
    const router = useRouter();
</script>

<template>
    <div id="OtherPage1">
        <div id="OpLeft">
            <span class="OpLeftSmTitle">Minecraft 本体</span>
            <YunLeftMenuButton
                @click="router.push('/download/1')"
                :hightlight="route.path === '/download/1'">
                <IconDownloadAuto :size="18" />
                <span>自动安装</span>
            </YunLeftMenuButton>
            <YunLeftMenuButton
                @click="router.push('/download/2')"
                :hightlight="route.path === '/download/2'">
                <IconDownloadManually :size="16" />
                <span>手动安装包</span>
            </YunLeftMenuButton>
            <br />
            <span class="OpLeftSmTitle">Minecraft 相关</span>
            <YunLeftMenuButton
                @click="router.push('/download/3')"
                :hightlight="route.path === '/download/3'">
                <IconDownloadMod :size="18" />
                <span>模组</span>
            </YunLeftMenuButton>
            <YunLeftMenuButton
                @click="router.push('/download/4')"
                :hightlight="route.path === '/download/4'">
                <IconDownloadModpack :size="18" />
                <span>整合包</span>
            </YunLeftMenuButton>
            <YunLeftMenuButton
                @click="router.push('/download/5')"
                :hightlight="route.path === '/download/5'">
                <IconDownloadAddon :size="16" />
                <span>附加内容</span>
            </YunLeftMenuButton>
        </div>
        <!-- <YunScroll height="calc(100vh - 55px)"></YunScroll> -->
        <YunLoading text="页面施工中 · · ·" />
    </div>
</template>

<style lang="scss" scoped>
    div#OtherPage1 {
        width: 100vw;
        height: 100%;
        display: flex;
        
        div#OpLeft {
            --pad-y: 24px;
            width: 180px;
            height: calc(100% - var(--pad-y) - 24px);
            background-color: #f8f8f9;
            box-shadow: 3px 0px 4px rgba(0, 0, 0, 0.25);
            display: flex;
            gap: 8px;
            flex-direction: column;
            align-items: center;
            padding: var(--pad-y) 0;
            padding-bottom: 24px;
            
            span.OpLeftSmTitle {
                width: 85%;
                opacity: 0.8;
                font-size: 0.8em;
            }
        }
    }
</style>
