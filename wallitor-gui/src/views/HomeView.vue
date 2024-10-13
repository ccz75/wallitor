<script setup lang="ts">
import ItemCard from '@/components/ItemCard.vue';
import ApplyBar from '@/components/ApplyBar.vue';
import AddItem from '@/components/AddItem.vue';
import { ref, onMounted } from 'vue';
import { entry } from '@/ts/entry';
import { invoke } from '@tauri-apps/api/core';
import type { wpConfig, Cell } from '@/ts/types'

const items = ref<Cell[]>([])
const apply_bar_visible = ref(false);
const applyBar = ref<InstanceType<typeof ApplyBar> | null>(null);
const item_add_visible = ref(false);

interface Resource {
  "config.json": string,
  [filename: string]: string
}

interface ResourceDir {
  files: {
    [resId: string]: Resource
  }
}

function arrayBufferToString(buffer: ArrayBuffer): string {
  const decoder = new TextDecoder('utf-8');
  return decoder.decode(buffer);
}

onMounted(() => {
  const main = document.querySelector(".home-main") as HTMLElement;
  setTimeout(() => {
    entry("up", main, 20);
  })
  invoke("read_resource_dir", {}).then((res) => {
    let resource = JSON.parse(res as string) as ResourceDir;
    for (let id of Object.keys(resource.files)) {
      let dir = resource.files[id]
      if ("preview.jpg" in dir) {
        invoke("get_file", {
          path: dir["preview.jpg"]
        }).then((res) => {
          let binary_data_arr = new Uint8Array(res as number[]);
          const blob = new Blob([binary_data_arr], { type: 'image/jpeg' });
          const imageUrl = URL.createObjectURL(blob);
          invoke("get_file", {
            path: `${id}\\config.json`
          }).then((cfg) => {
            let config: wpConfig = JSON.parse(arrayBufferToString(cfg as ArrayBuffer));
            items.value.push({
              path: id,
              img: imageUrl,
              config: config
            })
          })
        })
      }
    }
  });
})

function openCard(config: Cell) {
  console.log(applyBar.value)
  if (applyBar.value) applyBar.value.open(config);
}
</script>

<template>
  <main class="colbox home-main">
    <ItemCard v-for="(item, index) in items" :key="index" :cell="item" @click="openCard(item)"></ItemCard>
  </main>
  <ApplyBar v-model="apply_bar_visible" ref="applyBar"></ApplyBar>
  <AddItem v-model="item_add_visible"></AddItem>
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
