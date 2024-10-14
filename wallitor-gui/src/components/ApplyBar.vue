<template>
    <div class="apply-bar-mask" v-if="visible_" @click.self="handleClose">
        <div ref="bg" class="apply-bar-bg" :class="{
            'apply-bar-left': position == 'left',
            'apply-bar-right': position == 'right'
        }">
            <div class="apply-bar-content rowbox">
                <img :src="cell.img" class="apply-bar-img">
                <div class="apply-bar-title">{{ cell.config.name }}</div>
                <div class="apply-bar-info colbox" style="align-items: center;">
                    <div class="apply-bar-info-tag">
                        类型
                    </div>
                    <div class="apply-bar-info-main">
                        {{ cell.config.info.media_type }}
                    </div>
                </div>
                <div class="apply-bar-info rowbox">
                    <div class="apply-bar-info-tag">
                        描述
                    </div>
                    <div class="apply-bar-info-main">
                        {{ cell.config.info.description }}
                    </div>
                </div>
                <div class="apply-bar-info colbox" style="align-items: center;">
                    <div class="apply-bar-info-tag">
                        创建时间
                    </div>
                    <div class="apply-bar-info-main">
                        {{ (new Date(cell.config.info.created * 1000)).toLocaleString() }}
                    </div>
                </div>
                <div class="apply-bar-info rowbox">
                    <div class="apply-bar-info-tag">
                        设置
                    </div>
                    <div class="apply-bar-info-main">
                        <div class="apply-bar-settings colbox">
                            <div>静音</div>
                            <div><input type="checkbox" v-model="cell.config.option.mute" /></div>
                        </div>
                    </div>
                </div>
                <button class="apply-button" @click="apply">应用</button>
            </div>
            <div class="apply-bar-close" @click="handleClose">
                <svg-icon name="close" color="var(--text-color)"></svg-icon>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { defineProps, defineExpose, defineEmits, defineModel, watch, ref, type PropType } from 'vue';
type Position = "left" | "right";
import type { Info, Cell } from '@/ts/types'
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
    position: {
        type: String as PropType<Position>,
        default: "right"
    },

})
const visible = defineModel<boolean>();
const emit = defineEmits(["submit"]);
const visible_ = ref(false);
const cell = ref<Cell>({
    img: "",
    path: "",
    config: {
        name: "",
        info: {
            description: "",
            created: 0,
            media_type: "Video",
            entry_point: ""
        },
        option: {
            mute: true
        }
    }
})
const info_items = ref<(keyof Info)[]>(["media_type", "description", "created"]);
const bg = ref<HTMLDivElement | null>(null);
defineExpose({ open })
watch(() => visible.value, (val, _) => {
    if (val) {
        visible_.value = true;
    }
    else {
        if (bg.value) bg.value.style.animation = `apply-bar-disappear-${props.position} .3s ease-in`;
        setTimeout(() => { visible_.value = false }, 295);
    }
})

function handleClose() {
    visible.value = false;
}

function open(conFig: Cell) {
    cell.value = conFig;
    console.log(cell.value)
    visible.value = true;
}

import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
function apply() {
    const webview = new WebviewWindow('wallitor_video_playback', {
        title: "wallitor_video_playback",
        url: `/video/?url=${cell.value.path}\\res\\${cell.value.config.info.entry_point}`,
        fullscreen: true,
        decorations: false
    });
    webview.once("tauri://created", () => {
        invoke("set_wallpaper", {
            title: "wallitor_video_playback"
        }).then((res) => {
            console.log(res)
        })
    });
}
</script>

<style>
@keyframes apply-bar-appear-left {
    0% {
        transform: translate(-100%, -50%);
    }

    100% {
        transform: translate(0, -50%);
    }
}

@keyframes apply-bar-appear-right {
    0% {
        transform: translate(100%, -50%);
    }

    100% {
        transform: translate(0, -50%);
    }
}

@keyframes apply-bar-disappear-left {
    0% {
        opacity: 100%;
        transform: translate(0, -50%);
    }

    100% {
        opacity: 0%;
        transform: translate(-100%, -50%);
    }
}

@keyframes apply-bar-disappear-right {
    0% {
        opacity: 100%;
        transform: translate(0, -50%);
    }

    100% {
        opacity: 0%;
        transform: translate(100%, -50%);
    }
}

.apply-bar-mask {
    z-index: 200;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
}

.apply-bar-bg {
    border: solid var(--bd-color) 1px;
    backdrop-filter: blur(30px) saturate(180%);
    box-shadow: var(--shadow-edge-glow), var(--shadow);
    background-color: var(--bg-color-alpha-darker);
    border-radius: 8px;
    transform: translate(0, -50%);
    top: calc(50% + 15px);
    position: absolute;
    width: 50%;
    height: 85%;
}

.apply-bar-content {
    padding: 20px 15% 0 15%;
    height: calc(100% - 40px);
    width: 70%;
    overflow: auto;
    color: var(--text-color);
}

.apply-bar-img {
    width: 100%;
    border-radius: 7px;
}

.apply-bar-close {
    position: absolute;
    right: 0;
    top: 0;
    margin: 15px;
    cursor: pointer;
}

.apply-bar-left {
    left: 10px;
    animation: apply-bar-appear-left .6s cubic-bezier(0, 0.6, 0.2, 1.0);
}

.apply-bar-right {
    right: 10px;
    animation: apply-bar-appear-right .6s cubic-bezier(0, 0.6, 0.2, 1.0);
}

.apply-bar-title {
    font-size: 35px;
    font-weight: 700;
    margin-top: 5px;
    margin-bottom: 5px;
}

.apply-bar-info-tag {
    border-radius: 2px;
    border: 1px solid var(--bd-color);
    width: fit-content;
    padding: 4px;
    margin-top: 7px;
    margin-bottom: 5px;
    font-size: 12px;
    font-weight: 200;
    margin-right: 5px;
}

.apply-bar-info-main {
    font-size: 15px;
    font-weight: 500;
}
</style>