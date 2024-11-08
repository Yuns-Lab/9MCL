<script setup lang="ts">
    import { reactive } from "vue";
    import { useRoute, useRouter } from "vue-router";
    // Components
    import YunLeftMenuButton from "@/components/YunLeftMenuButton.vue";
    import YunScroll from "@/components/YunScroll.vue";
    import YunCard from "@/components/YunCard.vue";
    import YunInput from "@/components/YunInput.vue";
    import YunButton from "@/components/YunButton.vue";
    import YunLoading from "@/components/YunLoading.vue";
    // Icons
    import IconDownloadAuto from "@/icons/IconDownloadAuto.vue";
    import IconDownloadManually from "@/icons/IconDownloadManually.vue";
    import IconDownloadMod from "@/icons/IconDownloadMod.vue";
    import IconDownloadModpack from "@/icons/IconDownloadModpack.vue";
    import IconDownloadAddon from "@/icons/IconDownloadAddon.vue";
    import YunDropdown from "@/components/YunDropdown.vue";
    
    const route = useRoute();
    const router = useRouter();
    
    const searchOptions = reactive({
        searchText: "",
        searchPlatfrom: "all",
        searchMcVersion: "all",
        searchCategory: "all"
    });
    const dropdownOptions = reactive({
        mcVersions: [{ label: "全部", value: "all" }],
        platfroms: [
            { label: "全部", value: "all" },
            { label: "CurseForge", value: "cf" },
            { label: "Modrinth", value: "mr" }
        ],
        categories: [{ label: "全部", value: "all" }]
    });
    
    const handleResetSearch = () => {
        searchOptions.searchText = "";
        searchOptions.searchPlatfrom = "all";
        searchOptions.searchMcVersion = "all";
        searchOptions.searchCategory = "all";
    };
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
        <YunScroll height="calc(100vh - 55px)">
            <div id="OpRight">
                <YunCard
                    title="搜索 Mod"
                    style="overflow: visible">
                    <form
                        id="ModSearchContainer"
                        @submit="$event.preventDefault()">
                        <YunInput
                            style="grid-area: modname"
                            label="名称"
                            v-model="searchOptions.searchText" />
                        <YunDropdown
                            style="grid-area: modmcversion"
                            label="版本"
                            :options="dropdownOptions.mcVersions"
                            v-model="searchOptions.searchMcVersion" />
                        <YunDropdown
                            style="grid-area: modplatfrom"
                            label="平台"
                            :options="dropdownOptions.platfroms"
                            v-model="searchOptions.searchPlatfrom" />
                        <YunDropdown
                            style="grid-area: modcategory"
                            label="标签"
                            :options="dropdownOptions.categories"
                            v-model="searchOptions.searchCategory" />
                        <div
                            style="
                                grid-area: modsearch;
                                display: flex;
                                align-items: center;
                                gap: 16px;
                            ">
                            <YunButton
                                style="padding: 4px 32px; height: 33.6px"
                                type="submit">
                                搜索
                            </YunButton>
                            <YunButton
                                style="padding: 4px 32px; height: 33.6px"
                                @click="handleResetSearch">
                                重置条件
                            </YunButton>
                        </div>
                    </form>
                </YunCard>
                <YunLoading
                    text="页面施工中 · · ·"
                    style="margin-top: 120px" />
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
            
            form#ModSearchContainer {
                width: calc(100% - 24px);
                padding: 3px 12px;
                margin-bottom: 3px;
                display: grid;
                grid-template-rows: 1fr 1fr 1fr;
                grid-template-columns: 1fr 1fr 1fr;
                column-gap: 16px;
                row-gap: 12px;
                grid-template-areas:
                    "modname modname modname"
                    "modmcversion modplatfrom modcategory"
                    "modsearch modsearch modsearch";
            }
        }
    }
</style>
