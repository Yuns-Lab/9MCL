<script setup lang="ts">
    import { ref } from "vue";
    import YunLauncherButton from "@/components/YunLauncherButton.vue";
    import YunButton from "@/components/YunButton.vue";
    import YunSkinButton from "@/components/YunSkinButton.vue";
    import IconMicrosoft from "@/icons/IconMicrosoft.vue";
    import IconOffline from "@/icons/IconOffline.vue";
    import IconAuthlibInjector from "@/icons/IconAuthlibInjector.vue";
    import IconLogout from "@/icons/IconLogout.vue";
    import IconSkinDownload from "@/icons/IconSkinDownload.vue";
    import IconEditSkin from "@/icons/IconEditSkin.vue";
    import IconList from "@/icons/IconList.vue";
    import YunTooltip from "@/components/YunTooltip.vue";
    
    let loginType: keyof typeof loginTypeText = "ms";
    const loginTypeText = {
        ms: "正版",
        legacy: "离线",
        authlib: "第三方登录"
    };
    
    const avatardisplay = ref<null | HTMLDivElement>(null);
    const showskb = () => {
        avatardisplay.value
            ?.querySelectorAll("button")
            ?.forEach((btn) => btn.classList.add("show"));
        setTimeout(hideskb, 2000);
    };
    const hideskb = () => {
        avatardisplay.value
            ?.querySelectorAll("button:not(:hover)")
            ?.forEach((btn) => btn.classList.remove("show"));
        avatardisplay.value?.querySelectorAll("button:hover")?.forEach((btn) =>
            btn.addEventListener(
                "mouseleave",
                () => btn.classList.remove("show"),
                {
                    once: true
                }
            )
        );
    };
</script>

<template>
    <div id="Home">
        <div id="HomeLeft">
            <div id="LoginType">
                <IconMicrosoft
                    v-if="loginType === 'ms'"
                    :size="24" />
                <IconOffline
                    v-else-if="loginType === 'legacy'"
                    :size="24" />
                <IconAuthlibInjector
                    v-else-if="loginType === 'authlib'"
                    :size="24" />
                <span>
                    {{ loginTypeText[loginType] }}
                </span>
            </div>
            <div id="PlayerInfo">
                <div
                    id="AvatarDisplay"
                    @contextmenu="showskb"
                    ref="avatardisplay">
                    <YunTooltip
                        tooltip="右键显示操作按钮"
                        direction="x"
                        position="top">
                        <div id="ImgContainer" style="width: 84px; height: 84px;">
                            <img
                                draggable="false"
                                src="/skins/MHF_Alex.png" />
                            <img
                                draggable="false"
                                src="https://minotar.net/helm/LingyunAwA" />
                        </div>
                    </YunTooltip>
                    <YunSkinButton
                        hoverColor="#8bc34a"
                        showTop="0%"
                        showLeft="140%">
                        <IconSkinDownload :size="18" />
                    </YunSkinButton>
                    <YunSkinButton
                        hoverColor="#8bc34a"
                        showTop="50%"
                        showLeft="150%">
                        <IconEditSkin :size="18" />
                    </YunSkinButton>
                    <YunSkinButton
                        hoverColor="#8bc34a"
                        showTop="100%"
                        showLeft="140%">
                        <IconLogout :size="16" />
                    </YunSkinButton>
                </div>
                <span id="PlayerName">LingyunAwA</span>
            </div>
            <div id="ButtonGroup">
                <YunLauncherButton clientName="1.20.3-Fabric 0.12.6" />
                <YunButton
                    :tabindex="-1"
                    style="padding: 8px">
                    <IconList :size="28" />
                </YunButton>
            </div>
        </div>
        <div id="HomeRight"></div>
    </div>
</template>

<style lang="scss" scoped deep>
    div#Home {
        width: 100vw;
        height: 100%;
        display: flex;
        
        div#HomeLeft {
            --pad-y: 48px;
            width: 350px;
            height: calc(100% - var(--pad-y) - 24px);
            background-color: #f8f8f9;
            box-shadow: 3px 0px 4px rgba(0, 0, 0, 0.25);
            display: flex;
            flex-direction: column;
            justify-content: space-evenly;
            align-items: center;
            padding: var(--pad-y) 0;
            padding-bottom: 24px;
            
            div#LoginType {
                background-color: hsl(138, 56%, 12%);
                color: #f8f8f9;
                display: flex;
                justify-content: center;
                align-items: center;
                gap: 16px;
                padding: 6px 16px;
                border-radius: 4px;
            }
            
            div#PlayerInfo {
                margin-top: auto;
                display: flex;
                align-items: center;
                justify-content: center;
                flex-direction: column;
                gap: 16px;
                
                div#AvatarDisplay {
                    --avatar-size: 84px;
                    width: var(--avatar-size);
                    height: var(--avatar-size);
                    position: relative;
                    
                    div#ImgContainer {
                        position: relative;
                        width: 100%;
                        height: 100%;
                        border-radius: 8px;
                        box-shadow: 0px 0px 12px 4px rgba(0, 0, 0, 0.25);
                        overflow: hidden;
                        
                        img {
                            position: absolute;
                            width: 100%;
                            height: 100%;
                        }
                    }
                    
                    button.yun-skin-button:active {
                        transform: translate(-50%, -50%) scale(0.94);
                    }
                }
                
                span#PlayerName {
                    font-size: 1.3em;
                    margin-top: 4px;
                }
            }
            
            div#ButtonGroup {
                width: 90%;
                margin-top: auto;
                display: flex;
                gap: 8px;
            }
        }
    }
</style>
