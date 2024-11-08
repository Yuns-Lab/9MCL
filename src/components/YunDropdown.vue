<script setup lang="ts">
    import IconArrowRight from "@/icons/IconArrowRight.vue";
    import { ref, onMounted, onBeforeUnmount, watch } from "vue";
    
    // Define props and emit
    const props = defineProps<{
        options: { label: string; value: string | number }[];
        modelValue?: string | number;
        label?: string;
    }>();
    const emit = defineEmits(["update:modelValue"]);
    
    const isOpen = ref(false);
    const selectedLabel = ref("Select an option");
    const dropdown = ref<HTMLElement | null>(null);
    const updateSelectedLabel = () => {
        const selectedOption = props.options.find(
            (option) => option.value === props.modelValue
        );
        selectedLabel.value = selectedOption
            ? selectedOption.label
            : "Select an option";
    };
    const toggleDropdown = () => {
        isOpen.value = !isOpen.value;
    };
    const selectOption = (option: {
        label: string;
        value: string | number;
    }) => {
        emit("update:modelValue", option.value);
        isOpen.value = false;
    };
    const handleClickOutside = (event: MouseEvent) => {
        if (dropdown.value && !dropdown.value.contains(event.target as Node)) {
            isOpen.value = false;
        }
    };
    watch(() => props.modelValue, updateSelectedLabel);
    
    onMounted(() => {
        window.addEventListener("click", handleClickOutside);
        updateSelectedLabel();
    });
    onBeforeUnmount(() => {
        window.removeEventListener("click", handleClickOutside);
    });
</script>

<template>
    <div class="yun-dropdown">
        <span
            v-if="props.label"
            class="label"
        >{{ props.label }}</span
        >
        <div
            class="dropdown"
            ref="dropdown">
            <button
                type="button"
                @click="toggleDropdown"
                class="dropdown-button">
                {{ selectedLabel }}
                <IconArrowRight
                    :size="24"
                    :class="{
                        swapped: isOpen,
                    }"
                    class="yun-dropdown__arrow" />
            </button>
            <ul
                v-if="isOpen"
                class="dropdown-menu">
                <li
                    v-for="option in options"
                    :key="option.value"
                    @click="selectOption(option)"
                    class="dropdown-item">
                    {{ option.label }}
                </li>
            </ul>
        </div>
    </div>
</template>

<style lang="scss" scoped>
    div.yun-dropdown {
        width: 100%;
        display: flex;
        align-items: center;
        gap: 16px;
        
        span.label {
            width: 2rem;
        }
        
        .dropdown {
            flex-grow: 1;
            position: relative;
            display: inline-block;
            
            button.dropdown-button {
                width: 100%;
                border: 1px solid #081c0e6f;
                background-color: white;
                border-radius: 4px;
                padding: 4px 8px;
                cursor: pointer;
                display: flex;
                justify-content: flex-end;
                align-items: center;
                
                &:focus {
                    outline: none;
                    border: 1px solid #081c0e;
                }
            }
            
            .dropdown-menu {
                position: absolute;
                top: 100%;
                left: 0;
                background-color: white;
                box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
                list-style-type: none;
                margin: 0;
                padding: 0;
                width: calc(100% - 2px - 8px);
                border-radius: 4px;
                border: 1px solid #081c0e;
                padding: 4px;
                padding-top: 6px;
                z-index: 101;
            }
            
            .dropdown-item {
                padding: 4px;
                margin-bottom: 2px;
                cursor: pointer;
                border-radius: 4px;
                
                &:hover {
                    background-color: #081c0e0f;
                }
            }
        }
    }
    
    .yun-dropdown__arrow {
        margin-left: auto;
        transition: all 0.3s cubic-bezier(0.27, -0.51, 0.74, 1.51);
        transform: translateY(-1px) rotate(90deg);
        
        &.swapped {
            transform: translateY(-1px) rotate(-90deg);
        }
    }
</style>
