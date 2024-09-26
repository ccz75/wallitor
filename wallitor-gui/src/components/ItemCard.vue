<template>
    <div>
        <div class="item-card-main">

        </div>
        <div class="item-card-title">
            {{ config.name }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, defineProps } from 'vue'
import type { PropType } from 'vue'

interface Config {
    name: string,
}

const props = defineProps({
    config: {
        type: Object as PropType<Config>,
        required: true
    }
})
</script>