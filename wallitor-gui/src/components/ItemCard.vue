<template>
    <div :style="{
        backgroundImage: cell.img ? `url(${cell.img})` : 'linear-gradient(135deg, #00000020 0%, #FFFFFF20 100%)'
    }" class="item-card">
        <!-- <div class="item-card-main">
            <img :src="config.img" />
        </div> -->
        <div class="item-card-title">
            {{ cell.config.name }}
        </div>
    </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'
import type { PropType } from 'vue'
import type { Cell } from '@/ts/types'

const props = defineProps({
    cell: {
        type: Object as PropType<Cell>,
        required: true
    }
})
</script>

<style>
@keyframes item-card-entry{
    0%{opacity: 0;transform: translateY(10px);}
    100%{opacity: 1;transform: translateY(0px);}
}
:root {
    --item-card-size: 200px;
    --item-card-margin: 20px;
}

.item-card {
    text-align: center;
    width: var(--item-card-size);
    height: var(--item-card-size);
    margin-right: var(--item-card-margin);
    margin-bottom: var(--item-card-margin);
    position: relative;
    overflow: hidden;
    transition: .3s;
    border-radius: 10px;
    box-shadow: 0 0 1px 6px transparent;
    background-size: cover;
    background-position: center;
    animation: item-card-entry 0.3s ease-out;
}

.item-card-main {
    width: 100%;
    height: 100%;
    z-index: 0;
}

.item-card:hover {
    cursor: pointer;
    transform: scale(1.03);
    box-shadow: 0 0 2px 5px rgba(177, 177, 177, 0.151);
}

.item-card:active {
    transform: scale(0.95);
}

.item-card:hover .item-card-main {
    box-shadow: inset 0px -100px 50px -50px rgba(0, 0, 0, 0.7);
}

.item-card:hover .item-card-title {
    bottom: 5px;
}

.item-card-main img {
    width: 100%;
    height: 100%;
}

.item-card-title {
    position: absolute;
    bottom: -40px;
    right: 10px;
    transition: .3s;
    font-size: 30px;
    font-weight: 400;
    color: var(--text-color);
    text-shadow: 1px 1px 2px var(--bg-color);
}
</style>