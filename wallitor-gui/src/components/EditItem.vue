<template>
    <div class="item-edit-content">
        <main class="item-edit-main">
            <table class="item-edit-form">
                <tbody>
                    <tr>
                        <td class="item-edit-form-title">标题</td>
                        <td><input type="text" v-model="data.name" /></td>
                    </tr>
                    <tr>
                        <td class="item-edit-form-title">文件</td>
                        <td>
                            <template v-if="image_src">
                                <img :src="image_src" class="item-edit-image" @click="selectPreview">
                            </template>
                            <template v-else-if="data.preview">
                                <img :src="data.preview" class="item-edit-image" @click="selectPreview">
                            </template>
                            <template v-else>
                                <div class="item-edit-preview" @click="selectPreview">
                                    <div class="item-edit-preview-text">
                                        <SvgIcon name="add" size="20px"></SvgIcon>点击添加封面
                                    </div>
                                </div>
                            </template>
                        </td>
                    </tr>
                    <tr>
                        <td class="item-edit-form-title">描述</td>
                        <td><textarea class="item-edit-description" v-model="data.description"></textarea>
                        </td>
                    </tr>
                    <tr>
                        <td class="item-edit-form-title">静音</td>
                        <td>
                            <div><input type="checkbox" v-model="data.mute" /></div>
                        </td>
                    </tr>
                    <tr>
                        <td></td>
                        <td><button class="apply-button" @click="handleEdit" ref="applyButton">添加</button></td>
                    </tr>
                </tbody>
            </table>
        </main>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from 'vuex';
import { ElMessage } from 'element-plus';
import type { EditInfo } from '@/ts/types';

const store = useStore();
const data = defineModel<EditInfo>("data", {
    required: true
})
const emit = defineEmits(["submit"]);
const image_src = ref("");
const image_path = ref("");
const support_img_ext = ['.jpg', '.png', '.gif', '.webp'];
const support_img_ext_map: { [key in (typeof support_img_ext)[number]]: string } = {
    '.jpg': 'image/jpeg',
    '.png': 'image/png',
    '.gif': 'image/gif',
    '.webp': 'image/webp'
}
const applyButton = ref<HTMLButtonElement | null>(null);

async function handleFileOpen() {
    const file = await open({
        multiple: false,
        directory: false,
    });
    return file;
}

function selectPreview() {
    handleFileOpen().then((file) => {
        if (file) {
            let ext = file.substring(file.lastIndexOf("."));
            if (support_img_ext.includes(ext)) {
                image_path.value = file;
                invoke("get_file", {
                    path: file
                }).then((res) => {
                    let binary_data_arr = new Uint8Array(res as number[]);
                    const blob = new Blob([binary_data_arr], { type: support_img_ext_map[ext] });
                    const imageUrl = URL.createObjectURL(blob);
                    image_src.value = imageUrl;
                })
            } else ElMessage({
                type: "error",
                message: "文件格式不受支持"
            })
        }
    })
}

function checkInfo(info: EditInfo) {
    if (!info.name) return false
    else return true;
}

function handleEdit() {
    if (checkInfo(data.value)) {
        if (applyButton.value) {
            ElMessage({
                type: "info",
                message: "正在修改项目, 请勿重新点击或关闭程序"
            })
            const info = Object.assign({}, data.value);
            info.preview = image_path.value;
            applyButton.value.disabled = true;
            invoke("edit_wallpaper", {
                info: info
            }).then((res) => {
                console.log(info);
                if (applyButton.value) applyButton.value.disabled = false;
                if (res as boolean) {
                    store.commit("getWpList");
                    emit("submit");
                    ElMessage({
                        type: "success",
                        message: "修改成功"
                    })
                }
                else ElMessage({
                    type: "error",
                    message: `修改失败 ${res}`
                })
            })
        }
    }
    else ElMessage({
        type: "error",
        message: "请填写名称并选择媒体文件"
    })

}
</script>

<style>
.item-edit-show {
    top: 60px;
}

.item-edit-content {
    padding: 10px;
    height: calc(100% - 20px);
    overflow: hidden;
}

.item-edit-main {
    margin: 10px;
    height: 100%;
    overflow: auto;
}

.item-edit-main::-webkit-scrollbar {
    display: none;
}

.item-edit-header-icon {
    width: 40px;
    height: 40px;
    border-radius: 5px;
    background-color: #3b93ff;
    transition: .3s;
    align-items: center;
    justify-content: center;
}

.item-edit-header-icon:hover {
    background-color: #1e5daa;
    cursor: pointer;
}

.item-edit-header-item {
    display: flex;
    align-items: center;
    color: var(--text-color);
}

.item-edit-header-title {
    font-size: 22px;
    font-weight: 600;
    margin-left: 10px;
}

.item-edit-main {
    color: var(--text-color);
}

.item-edit-preview {
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

.item-edit-preview:hover {
    background: var(--bg-hover-fill);
}

.item-edit-preview:active {
    transform: scale(0.95);
}

.item-edit-image {
    background: linear-gradient(135deg, #0000001A 0%, #FFFFFF1A 100%);
    border-radius: 5px;
    width: 400px;
    margin-bottom: 10px;
    box-shadow: var(--shadow-edge-glow);
    cursor: pointer;
}

.item-edit-image:active {
    transform: scale(0.95);
}

.item-edit-form {
    margin-top: 5px;
}

.item-edit-form td {
    vertical-align: top;
    padding: 5px;
}

.item-edit-description {
    width: 300px;
    height: 150px;
}

.item-edit-form-title {
    font-size: 15px;
    font-weight: 600;
}

.item-edit-preview-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}
</style>