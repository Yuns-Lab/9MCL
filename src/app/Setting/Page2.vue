<script setup lang="ts">
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunCard from "@/components/YunCard.vue";
    import YunScroll from "@/components/YunScroll.vue";
    import YunLoading from "@/components/YunLoading.vue";
    // Icons
    import IconSettingCustomize from "@/icons/IconSettingCustomize.vue";
    import IconSettingCore from "@/icons/IconSettingCore.vue";
    import IconSettingLaunch from "@/icons/IconSettingLaunch.vue";
    
    const route = useRoute();
    const router = useRouter();
</script>

<template>
    <div id="OtherPage1">
        <div id="OpLeft">
            <span class="OpLeftSmTitle">本体设置</span>
            <YunLeftMenuButton
                @click="router.push('/setting/1')"
                :hightlight="route.path === '/setting/1'">
                <IconSettingCustomize :size="18" />
                <span>个性化</span>
            </YunLeftMenuButton>
            <YunLeftMenuButton
                @click="router.push('/setting/2')"
                :hightlight="route.path === '/setting/2'">
                <IconSettingCore :size="18" />
                <span>核心设置</span>
            </YunLeftMenuButton>
            <br />
            <span class="OpLeftSmTitle">游戏设置</span>
            <YunLeftMenuButton
                @click="router.push('/setting/3')"
                :hightlight="route.path === '/setting/3'">
                <IconSettingLaunch :size="18" />
                <span>启动设置</span>
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
        
        div#OpRight {
            width: calc(100vw - 180px - 48px);
            padding: 24px;
            display: flex;
            flex-direction: column;
            gap: 16px;
        }
    }
</style>
