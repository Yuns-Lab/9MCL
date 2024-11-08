<script setup lang="ts">
    import { ref } from "vue";
    import { RouterView, useRoute, useRouter } from "vue-router";
    import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
    // Components
    import YunIconButton from "@/components/YunIconButton.vue";
    // Icons
    import IconClose from "@/icons/IconClose.vue";
    import IconMinimize from "@/icons/IconMinimize.vue";
    import IconMultiPage from "@/icons/IconMultiPage.vue";
    import IconPageHome from "@/icons/IconPageHome.vue";
    import IconPageDownload from "@/icons/IconPageDownload.vue";
    import IconPageSetting from "@/icons/IconPageSetting.vue";
    import IconPageOther from "@/icons/IconPageOther.vue";
    const appWindow = getCurrentWebviewWindow()

    const menu = ref<null | HTMLDivElement>(null);
    const toggleMenu = () => {
        menu.value?.classList.toggle("show");
        if (menu.value?.classList.contains("show")) {
            setTimeout(() => {
                menu.value?.classList.remove("show");
            }, 5000)
        }
    };
    const route = useRoute();
    const router = useRouter();
</script>

<template>
    <div id="container">
        <div
            id="appnav"
            data-tauri-drag-region>
            <span
                id="title"
                data-tauri-drag-region>
                9MCL
            </span>
            <div id="nav-right">
                <div
                    id="nav-menu"
                    ref="menu">
                    <YunIconButton
                        :tabindex="-1"
                        style="padding: 0 7px"
                        @click="toggleMenu">
                        <IconMultiPage size="1.2em" />
                    </YunIconButton>
                    <hr style="margin-right: 8px" />
                    <YunIconButton
                        :tabindex="-1"
                        style="padding: 0 10px"
                        :hightlight="route.path === '/'"
                        @click="router.push('/')">
                        <IconPageHome size="1.2em" />
                        <span class="badge">主页</span>
                    </YunIconButton>
                    <YunIconButton
                        :tabindex="-1"
                        style="padding: 0 10px"
                        :hightlight="route.path.includes('download')"
                        @click="router.push('/download/1')">
                        <IconPageDownload size="1.2em" />
                        <span class="badge">下载</span>
                    </YunIconButton>
                    <YunIconButton
                        :tabindex="-1"
                        style="padding: 0 10px"
                        :hightlight="route.path.includes('setting')"
                        @click="router.push('/setting/1')">
                        <IconPageSetting size="1.2em" />
                        <span class="badge">配置</span>
                    </YunIconButton>
                    <YunIconButton
                        :tabindex="-1"
                        style="padding: 0 10px"
                        :hightlight="route.path.includes('other')"
                        @click="router.push('/other/1')">
                        <IconPageOther size="1.2em" />
                        <span class="badge">更多</span>
                    </YunIconButton>
                </div>
                <hr style="height: 12px; opacity: 0.7; margin: 0 8px" />
                <YunIconButton
                    :tabindex="-1"
                    @click="appWindow.minimize()">
                    <IconMinimize size="1.4em" />
                </YunIconButton>
                <YunIconButton
                    :tabindex="-1"
                    @click="appWindow.close()">
                    <IconClose size="1.4em" />
                </YunIconButton>
            </div>
        </div>
        <RouterView />
    </div>
</template>

<style lang="scss">
    // Public
    :root {
        font-family: "Bahnschrift", sans-serif;
        user-select: none;
    }
    ::-webkit-scrollbar {
        display: none;
    }
</style>

<style scoped lang="scss">
    div#container {
        display: flex;
        flex-direction: column;
        width: 100vw;
        height: 100vh;
        border-radius: 8px;
        overflow: hidden;
        color: #081c0e;
        background-color: #dbe0df;
        div#appnav {
            width: 100vw;
            height: 60px;
            background-image: url("/themebg.png"),
                linear-gradient(to right, #081c0e, #081c0e);
            display: flex;
            align-items: center;
            padding: 0 24px;
            position: relative;
            img#navbg {
                position: absolute;
                width: 100%;
                height: 60px;
                left: -24px;
                z-index: 0;
            }
            span#title {
                font-size: 24px;
                color: #dbe0df;
            }
            div#nav-right {
                display: flex;
                align-items: center;
                justify-content: flex-end;
                gap: 8px;
                height: 100%;
                margin-left: auto;
                margin-right: 42px;
                div#nav-menu {
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    width: 30px;
                    overflow: hidden;
                    transition: all 0.3s ease-in-out;
                    &.show {
                        width: 372.25px;
                    }
                    span.badge {
                        margin-left: 8px;
                        width: 2em;
                    }
                }
            }
        }
    }
</style>
