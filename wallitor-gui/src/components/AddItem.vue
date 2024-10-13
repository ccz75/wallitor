<template>
    <div ref="bg" class="item-add-bg" :class="{
        'item-add-show': visible,
    }">
        <div class="item-add-content">
            <header class="colbox item-add-header">
                <div class="colbox item-add-header-icon item-add-header-item" @click="visible = !visible">
                    <template v-if="visible">
                        <SvgIcon name="keyboard-arrow-down" size="20px"></SvgIcon>
                    </template>
                    <template v-else>
                        <SvgIcon name="add" size="20px"></SvgIcon>
                    </template>
                </div>
                <div class="item-add-header-item item-add-header-title">
                    <div>添加</div>
                </div>
            </header>
            <main class="item-add-main">
                <table class="item-add-form">
                    <tr>
                        <td class="item-add-form-title">标题</td>
                        <td><input type="text" v-model="addInfo.name" /></td>
                    </tr>
                    <tr>
                        <td class="item-add-form-title">文件</td>
                        <td>
                            <template v-if="image_src">
                                <img :src="image_src" class="item-add-image">
                            </template>
                            <template v-else>
                                <div class="item-add-preview" @click="selectPreview">
                                    <div class="item-add-preview-text">
                                        <SvgIcon name="add" size="20px"></SvgIcon>点击添加封面
                                    </div>
                                </div>
                            </template>
                            <div class="colbox">
                                <button class="apply-button" @click="selectMedia">选择</button>
                                <div>{{ addInfo.media }}</div>
                            </div>
                        </td>
                    </tr>
                    <tr>
                        <td class="item-add-form-title">描述</td>
                        <td><textarea class="item-add-description" v-model="addInfo.description"></textarea></td>
                    </tr>
                    <tr>
                        <td></td>
                        <td><button class="apply-button" @click="handleFileOpen">添加</button></td>
                    </tr>
                </table>
            </main>
        </div>
    </div>
</template>

<script setup lang="ts">
import { defineExpose, defineModel, ref } from 'vue';
import SvgIcon from './SvgIcon.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
const visible = defineModel<boolean>();
const bg = ref<HTMLDivElement | null>(null);
interface AddInfo {
    name: string,
    preview: string,
    media: string,
    description: string
}
const addInfo = ref<AddInfo>({
    name: "",
    preview: "",
    media: "",
    description: ""
})
const image_src = ref("");
const support_ext = [".mp4", ".mkv", ".flv", ".ts"];

defineExpose({ open })

async function handleFileOpen() {
    const file = await open({
        multiple: false,
        directory: false,
    });
    return file;
}

function selectMedia() {
    handleFileOpen().then((file) => {
        if (file) {
            let ext = file.substring(file.lastIndexOf("."));
            if (support_ext.includes(ext)) {
                addInfo.value.media = file;
            }
        }
    })
}

function selectPreview() {
    handleFileOpen().then((file) => {
        if (file) {
            addInfo.value.preview = file;
            invoke("get_file", {
                path: file
            }).then((res) => {
                let binary_data_arr = new Uint8Array(res as number[]);
                const blob = new Blob([binary_data_arr], { type: 'image/jpeg' });
                const imageUrl = URL.createObjectURL(blob);
                image_src.value = imageUrl;
            })
        }
    })
}
</script>

<style>
.item-add-bg {
    border: solid var(--bd-color) 1px;
    backdrop-filter: blur(30px) saturate(180%);
    box-shadow: var(--shadow-edge-glow), var(--shadow);
    background-color: var(--bg-color-alpha-darker);
    border-radius: 8px;
    left: 50%;
    top: calc(100% - 60px);
    transform: translate(-50%, 0);
    position: absolute;
    width: 85%;
    height: calc(100% - 60px);
    transition: .8s cubic-bezier(0.9, 0, 0, 1.1);
}

.item-add-show {
    top: 60px;
}

.item-add-content {
    padding: 10px;
    height: calc(100% - 20px);
    overflow: hidden;
}

.item-add-main {
    margin: 5px;
    height: calc(100% - 45px);
    overflow: auto;
}

.item-add-main::-webkit-scrollbar {
    display: none;
}

.item-add-header-icon {
    width: 40px;
    height: 40px;
    border-radius: 5px;
    background-color: #3b93ff;
    transition: .3s;
    align-items: center;
    justify-content: center;
}

.item-add-header-icon:hover {
    background-color: #1e5daa;
    cursor: pointer;
}

.item-add-header-item {
    display: flex;
    align-items: center;
    color: var(--text-color);
}

.item-add-header-title {
    font-size: 22px;
    font-weight: 600;
    margin-left: 10px;
}

.item-add-main {
    color: var(--text-color);
}

.item-add-preview {
    background: linear-gradient(135deg, #0000000A 0%, #FFFFFF0A 100%);
    border-radius: 5px;
    width: 400px;
    height: 200px;
    margin-bottom: 10px;
    box-shadow: var(--shadow-edge-glow);
    position: relative;
    cursor: pointer;
    transition: .5s;
}

.item-add-preview:hover {
    background: var(--bg-color-alpha-darker);
}

.item-add-preview:active {
    transform: scale(0.95);
}

.item-add-image {
    background: linear-gradient(135deg, transparent 0%, #FFFFFF10 100%);
    border-radius: 5px;
    width: 400px;
    margin-bottom: 10px;
    box-shadow: var(--shadow-edge-glow);
}

.item-add-form {
    margin-top: 5px;
}

.item-add-form td {
    vertical-align: top;
    padding: 5px;
}

.item-add-description {
    width: 300px;
    height: 150px;
}

.item-add-form-title {
    font-size: 15px;
    font-weight: 600;
}

.item-add-preview-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
</style>