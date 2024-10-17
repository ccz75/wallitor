<template>
    <div class="settings-wrapper">
        <div class="settings-back colbox" @click="back_home">
            <SvgIcon name="direction-left"></SvgIcon>
            <div class="settings-back-text">返回</div>
        </div>
        <table class="settings-table">
            <tbody class="setting-table-body">
                <tr>
                    <td>
                        <div class="settings-label">
                            <div class="settings-label-title">标题栏样式</div>
                            <div class="settings-label-subtitle">选择选题栏关闭按钮样式</div>
                        </div>
                    </td>
                    <td>
                        <el-select @change="save" v-model="settings.title_bar">
                            <el-option label="Windows" value="win"></el-option>
                            <el-option label="OSX" value="mac"></el-option>
                        </el-select>
                    </td>
                </tr>
                <tr>
                    <td>
                        <div class="settings-label">
                            <div class="settings-label-title">开机自启</div>
                            <div class="settings-label-subtitle">开机自启</div>
                        </div>
                    </td>
                    <td><el-switch @change="save" v-model="settings.auto_start"></el-switch></td>
                </tr>
                <tr>
                    <td>
                        <div class="settings-label">
                            <div class="settings-label-title">最大化自动暂停</div>
                            <div class="settings-label-subtitle">最大化自动暂停</div>
                        </div>
                    </td>
                    <td><el-switch @change="save" v-model="settings.auto_pause"></el-switch></td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script lang="ts" setup>
import { ElSelect, ElSwitch, ElOption, ElMessage } from 'element-plus';
import { entry } from '@/ts/entry';
import { useRouter } from 'vue-router';
import { useStore } from 'vuex';
import { onMounted, ref } from 'vue';
import SvgIcon from '@/components/SvgIcon.vue';
import type { Settings } from '@/ts/types';
const router = useRouter();
const store = useStore();
const settings = ref<Settings>(store.state.settings);

onMounted(() => {
    const table_body = document.querySelector(".setting-table-body") as HTMLElement;
    entry("left", table_body, 50);
})

function back_home() {
    router.push("/")
}

function save() {
    store.dispatch("set_settings", settings.value).then((status) => {
        if (status as boolean) {
            ElMessage({
                type: "success",
                message: "设置已更改"
            })
        }
        else ElMessage({
            type: "error",
            message: "设置更改失败"
        })
    })
}
</script>

<style>
.settings-wrapper {
    padding-left: 30px;
    padding-right: 30px;
    width: calc(100% - 60px);
}

.settings-back {
    color: var(--text-color);
    padding: 8px;
    margin: 5px;
    width: fit-content;
    border-radius: 10px;
    transition: .3s;
    cursor: pointer;
}

.settings-back:hover {
    background-color: var(--bg-color-alter);
}

.settings-back:active {
    transform: scale(0.95);
}

.settings-table {
    width: calc(100% - 80px);
    margin-left: 40px;
    margin-right: 40px;
}

.settings-table td,
th {
    padding: 5px;
    color: var(--text-color);
}

.settings-label-title {
    margin-bottom: 3px;
    font-size: 20px;
    font-weight: 600;
}

.settings-label-subtitle {
    font-size: 13px;
    font-weight: 300;
    margin-bottom: 10px;
}

.settings-back-text {
    transform: translateY(-3px);
    margin-left: 4px;
    font-weight: 600;
}
</style>