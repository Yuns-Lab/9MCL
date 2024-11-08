<script setup lang="ts">
    // Arco components
    import ATag from "@arco-design/web-vue/es/tag";
    import "@arco-design/web-vue/dist/arco.min.css";
    // Components
    import YunAuthorCard from "@/components/YunAuthorCard.vue";
    // Logos
    import LogoVue from "@/icons/logos/LogoVue.vue";
    import LogoRust from "@/icons/logos/LogoRust.vue";
    // Icons
    import IconColorPen from "@/icons/IconColorPen.vue";
    import IconGithub from "@/icons/IconGithub.vue";

    const visitGithubUser = async (username: string) => {
        window.open(`https://github.com/${username}`, "_blank");
    };
</script>

<template>
    <YunAuthorCard image="/authors/lingyun.png">
        <template #author>
            <h2 class="author-name">凌云</h2>
            <ATag
                color="gray"
                bordered
                style="cursor: pointer"
                @click="visitGithubUser('JingHai-Lingyun')">
                <template #icon>
                    <IconGithub :size="16" style="transform: translateY(3px)" />
                </template>
                JingHai-Lingyun
            </ATag>
            <ATag color="orange" bordered style="margin: 0 16px"> 创始人 </ATag>
            <ATag color="blue" bordered> 核心开发者 </ATag>
            <ATag color="magenta" bordered> 核心美术 </ATag>
        </template>
        <template #tags>
            <span style="transform: translateY(-5px)">主要职责：</span>
            <ATag color="green" bordered>
                <template #icon>
                    <LogoVue :size="0.3" />
                </template>
                前端
            </ATag>
            <ATag color="orangered" bordered>
                <template #icon>
                    <LogoRust style="transform: translateY(1px)" :size="12" />
                </template>
                后端
            </ATag>
            <ATag color="pinkpurple" bordered>
                <template #icon>
                    <IconColorPen
                        :size="14"
                        style="transform: translateY(4px)" />
                </template>
                UI/UX
            </ATag>
        </template>
    </YunAuthorCard>
</template>

<style lang="scss" scoped>
    h2.author-name {
        font-size: 2em;
        margin: 0;
        margin-right: 4px;
    }
    a {
        color: #081c0e;
    }
</style>
