<script setup lang="ts">
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunScroll from "@/components/YunScroll.vue";
    import YunCard from "@/components/YunCard.vue";
    import Yun3rdPartyComp from "@/components/Yun3rdPartyComp.vue";
    // Icons
    import IconOtherInfo from "@/icons/IconOtherInfo.vue";
    import IconOtherLaw from "@/icons/IconOtherLaw.vue";
    import IconPageOther from "@/icons/IconPageOther.vue";
    import IconGithub from "@/icons/IconGithub.vue";
    
    const route = useRoute();
    const router = useRouter();
    
    const openWebsite = (url: string) => {
        window.open(url, "_blank");
    };
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
        <YunScroll height="calc(100vh - 55px)">
            <div id="OpRight">
                <YunCard title="法律信息">
                    <div style="padding-left: 16px">
                        <h3 class="subtitle">隐私信息声明</h3>
                        <span>本软件向服务器发送请求时会携带上软件版本号，用于统计各版本使用人数</span>
                        <br />
                        <span>除此之外，本软件不会收集和向开发者发送任何用户信息</span>
                        <h3>版权</h3>
                        <span>Copyright © 凌云 2024. All Rights Reserved.</span>
                        <br />
                        <div>
                            作者平台信息：
                            <IconGithub size="18" style="transform: translateY(3.5px); margin-right: 2px" />
                            <span @click="openWebsite('https://github.com/Jim-Lin-4549')"
                                  style="color: inherit; text-decoration: underline; cursor: pointer">Jim Lin
                            </span>
                        </div>
                        <h3>其他信息</h3>
                        <span>违法违规举报邮箱 <a style="color: inherit" href="mailto:Jim.Lin.4549@outlook.com">Jim.Lin.4549@outlook.com</a></span>
                        <br />
                        <span>非 Minecraft 官方产品，未经 Mojang 或 Microsoft 批准，也与 Mojang 或 Microsoft 没有任何关联。</span>
                    </div>
                </YunCard>
                <YunCard title="第三方内容版权和许可声明" :can-swap="true" :is-swapped="true">
                    <Yun3rdPartyComp name="Material Design Icons" right="Pictogrammers" license="Apache 2.0 License"
                                     source-url="https://icones.js.org/collection/mdi" :is3rd-source="true"
                                     trust-doc-url="https://github.com/Templarian/MaterialDesign/blob/master/LICENSE" />
                </YunCard>
            </div>
        </YunScroll>
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
            box-shadow: 3px 0 4px rgba(0, 0, 0, 0.25);
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
            
            h3.subtitle {
                margin-top: 16px;
                margin-bottom: 8px;
            }
        }
    }
</style>
