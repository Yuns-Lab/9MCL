<script setup lang="ts">
    const props = defineProps<{
        modelValue: string;
        label?: string;
        placeholder?: string;
    }>();
    const emits = defineEmits(["update:modelValue"]);
    
    const handleInput = (event: Event) => {
        emits("update:modelValue", (event.target as HTMLInputElement).value);
    };
</script>

<template>
    <div class="yun-input">
        <span v-if="props.label">{{ props.label }}</span>
        <input
            :value="props.modelValue"
            :placeholder="props.placeholder"
            @input="handleInput" />
    </div>
</template>

<style scoped lang="scss">
    div.yun-input {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 16px;
        
        span {
            width: 2rem;
        }
        
        input {
            height: 24px;
            flex-grow: 1;
            border: 1px solid #081c0e6f;
            border-radius: 4px;
            padding: 4px 8px;
            
            &:focus {
                outline: none;
                border: 1px solid #081c0e;
            }
        }
    }
</style>
