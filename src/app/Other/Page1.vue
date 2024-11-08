<script setup lang="ts">
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunCard from "@/components/YunCard.vue";
    import YunScroll from "@/components/YunScroll.vue";
    // AuthorCards
    import YunAcLingyun from "@/components/AuthorCards/YunAcLingyun.vue";
    import YunAcEncvar from "@/components/AuthorCards/YunAcEncvar.vue";
    // Icons
    import IconOtherInfo from "@/icons/IconOtherInfo.vue";
    import IconOtherLaw from "@/icons/IconOtherLaw.vue";
    import IconPageOther from "@/icons/IconPageOther.vue";
    
    const route = useRoute();
    const router = useRouter();
    const $env = import.meta.env;
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
                <IconPageOther :size="18" style="margin-left: -2px" />
                <span>小工具</span>
            </YunLeftMenuButton>
        </div>
        <YunScroll height="calc(100vh - 55px)">
            <div id="OpRight">
                <YunCard>
                    <div
                        style="
                            display: flex;
                            justify-content: center;
                            gap: 60px;
                            margin: 24px 0 32px 0;
                        ">
                        <img
                            draggable="false"
                            src="/LogoYunMCL.png"
                            height="162"
                            style="
                                box-shadow: 7px 7px 10px rgba(0, 0, 0, 0.25);
                                border-radius: 16px;
                            " />
                        <div
                            style="margin: auto 0; transform: translateY(-8px)">
                            <h1>云启 · YunMCL</h1>
                            <span>
                                {{ $env.VITE_APP_UPDATE_CHANNEL }}渠道 v{{
                                    $env.VITE_APP_VERSION
                                }}
                                {{
                                    $env.VITE_APP_MODE == "Dev"
                                        ? `@ ${$env.VITE_APP_DEV_HASH}`
                                        : ""
                                }}
                            </span>
                        </div>
                    </div>
                </YunCard>
                <YunCard title="开发者团队" :can-swap="true" :is-swapped="true">
                    <YunAcLingyun />
                    <br />
                    <YunAcEncvar />
                    <hr width="97%" style="margin: 24px auto 16px" />
                    <span style="margin-left: 48px; opacity: 0.8">
                        除此之外就再也没有别人了……&nbsp;&nbsp;&nbsp;&nbsp;期待有更多人加入我们团队……
                    </span>
                    <p style="margin: 4px 48px; opacity: 0.8">
                        如果你有兴趣来的话请在 GitHub
                        上找到创始人的邮箱，发一封自荐邮件
                    </p>
                    <p style="margin: 4px 48px; opacity: 0.8">
                        我们会经过讨论之后大概率你可以我们加入团队
                    </p>
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
        }
    }
</style>
