<template>
    <div @click="back_home">
        <SvgIcon name="direction-left"></SvgIcon>
        返回
    </div>
    <el-form label-position="right">
        <el-form-item label="标题栏样式">
            <el-switch></el-switch>
        </el-form-item>
        <el-form-item label="开机自启">
            <el-switch></el-switch>
        </el-form-item>
        <el-form-item label="最大化自动暂停">
            <el-switch></el-switch>
        </el-form-item>
    </el-form>
</template>

<script lang="ts" setup>
import { ElForm, ElFormItem, ElSwitch, ElOption } from 'element-plus';
import { useRouter } from 'vue-router';
import SvgIcon from '@/components/SvgIcon.vue';
const router = useRouter();

function back_home() {
    router.push("/")
}
</script>