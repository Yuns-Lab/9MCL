<script setup lang="ts">
    // Arco components
    import ATag from "@arco-design/web-vue/es/tag";
    import "@arco-design/web-vue/dist/arco.min.css";
    // Components
    import YunAuthorCard from "@/components/YunAuthorCard.vue";
    // Logos
    import LogoCpp from "@/icons/logos/LogoCpp.vue";
    // Icons
    import IconGithub from "@/icons/IconGithub.vue";

    const visitGithubUser = async (username: string) => {
        window.open(`https://github.com/${username}`, "_blank");
    };
</script>

<template>
    <YunAuthorCard image="/authors/encvar.jpg">
        <template #author>
            <h2 class="author-name">EncVar</h2>
            <ATag
                color="gray"
                bordered
                style="cursor: pointer"
                @click="visitGithubUser('EncVar')">
                <template #icon>
                    <IconGithub :size="16" style="transform: translateY(3px)" />
                </template>
                EncVar
            </ATag>
            <ATag color="blue" bordered style="margin: 0 16px">
                核心开发者
            </ATag>
        </template>
        <template #tags>
            <span style="transform: translateY(-5px)">主要职责：</span>
            <ATag color="arcoblue" bordered>
                <template #icon>
                    <LogoCpp :size="16" style="transform: translateY(3px)" />
                </template>
                后端
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
