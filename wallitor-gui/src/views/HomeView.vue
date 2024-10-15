<script setup lang="ts">
import ItemCard from '@/components/ItemCard.vue';
import ApplyBar from '@/components/ApplyBar.vue';
import AddItem from '@/components/AddItem.vue';
import { ref, onMounted, computed, nextTick } from 'vue';
import CRMenu from '@/components/CRMenu.vue';
import CRMenuCell from '@/components/CRMenuCell.vue';
import SvgIcon from '@/components/SvgIcon.vue';
import type { Cell, EditInfo } from '@/ts/types'
import { useStore } from 'vuex';
import { invoke } from '@tauri-apps/api/core';
import { ElMessage } from 'element-plus';
import CDialog from '@/components/CDialog.vue';
import EditItem from '@/components/EditItem.vue';

const store = useStore();
const items = computed<Cell[]>(() => store.state.wpList);
const apply_bar_visible = ref(false);
const applyBar = ref<InstanceType<typeof ApplyBar> | null>(null);
const item_add_visible = ref(false);
const r_display = ref(false);
const r_data = ref<Cell>({
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
});
const menu = ref<InstanceType<typeof CRMenu> | null>(null);
const options = ref<{ name: string, icon: string, handler: (data: Cell) => void }[]>([{
  name: "删除",
  icon: "delete",
  handler: del_wallpaper
}, {
  name: "修改",
  icon: "edit",
  handler: edit_wallpaper
}])
const edit_visible = ref(false);
const edit_data = ref<EditInfo>({
  path: "",
  name: "",
  preview: "",
  description: "",
});

onMounted(() => {
  store.commit("getWpList");
})

function openCard(config: Cell) {
  if (applyBar.value) applyBar.value.open(config);
}

function handleRightClick(event: MouseEvent, data: Cell) {
  r_data.value = data;
  r_display.value = true;
  nextTick(() => {
    if (menu.value) menu.value.handleOpen(event.x, event.y);
  })
}

function del_wallpaper(data: Cell) {
  invoke("del_folder", {
    path: data.path
  }).then((res) => {
    if (res) {
      store.commit("getWpList");
      ElMessage({
        type: "success",
        message: `已删除 ${data.path}`
      })
    }
    else ElMessage({
      type: "error",
      message: `删除失败`
    })
  })
}

function edit_wallpaper(data: Cell) {
  edit_visible.value = true;
  edit_data.value = {
    path: data.path,
    name: data.config.name,
    preview: data.img,
    description: data.config.info.description
  }
}

function handle_edit_close() {
  edit_visible.value = false;
}
</script>

<template>
  <main class="colbox home-main">
    <ItemCard v-for="(item, index) in items" :key="index" :cell="item" @click="openCard(item)"
      @contextmenu.prevent="(e) => handleRightClick(e, item)"></ItemCard>
  </main>
  <ApplyBar v-model="apply_bar_visible" ref="applyBar"></ApplyBar>
  <AddItem v-model="item_add_visible"></AddItem>
  <CRMenu ref="menu" v-model:visible="r_display">
    <template #content>
      <CRMenuCell v-for="option in options" :key="option.name" @click="option.handler(r_data); r_display = false;">
        <template #icon>
          <SvgIcon size="20px" :name="option.icon" style="color: var(--text-color);"></SvgIcon>
        </template>
        {{ option.name }}
      </CRMenuCell>
    </template>
  </CRMenu>
  <CDialog v-model:visible="edit_visible" height="80%" width="80%">
    <template #content>
      <EditItem v-model:data="edit_data" @submit="handle_edit_close"></EditItem>
    </template>
  </CDialog>
</template>

<style>
.home-main {
  width: calc(100% - 40px);
  height: 100%;
  overflow-y: auto;
  padding-left: 20px;
  padding-right: 20px;
  padding-top: 10px;
  flex-wrap: wrap;
  align-content: flex-start;
}
</style>
