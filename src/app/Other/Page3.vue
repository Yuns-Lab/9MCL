<script setup lang="ts">
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunLoading from "@/components/YunLoading.vue";
    // Icons
    import IconOtherInfo from "@/icons/IconOtherInfo.vue";
    import IconOtherLaw from "@/icons/IconOtherLaw.vue";
    import IconPageOther from "@/icons/IconPageOther.vue";
    
    const route = useRoute();
    const router = useRouter();
</script>

<template>
    <div id="OtherPage1">
        <div id="OpLeft">
            <span class="OpLeftSmTitle">信息</span>
            <YunLeftMenuButton
                @click="router.push('/other/1')"
                :hightlight="route.path === '/other/1'">
                <IconOtherInfo :size="18" />
                <span>关于 YunMCL</span>
            </YunLeftMenuButton>
            <YunLeftMenuButton
                @click="router.push('/other/2')"
                :hightlight="route.path === '/other/2'">
                <IconOtherLaw :size="24" />
                <span>版权信息</span>
            </YunLeftMenuButton>
            <br />
            <span class="OpLeftSmTitle">其他</span>
            <YunLeftMenuButton
                @click="router.push('/other/3')"
                :hightlight="route.path === '/other/3'">
                <IconPageOther
                    :size="18"
                    style="margin-left: -2px" />
                <span>小工具</span>
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
