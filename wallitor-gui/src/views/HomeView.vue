<script setup lang="ts">
import ItemCard from '@/components/ItemCard.vue';
import ApplyBar from '@/components/ApplyBar.vue';
import AddItem from '@/components/AddItem.vue';
import { ref, onMounted } from 'vue';
import { entry } from '@/ts/entry';
interface Config {
  name: string,
  img: string
}
const items = ref<Config[]>([{
  name: "test1",
  img: "https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png"
}, {
  name: "test2",
  img: "https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png"
}])
const apply_bar_visible = ref(false);
const applyBar = ref<InstanceType<typeof ApplyBar> | null>(null);
const item_add_visible = ref(false);

onMounted(() => {
  const main = document.querySelector(".home-main") as HTMLElement;
  setTimeout(() => {
    entry("up", main, 20);
  })
})

function openCard(config: Config) {
  if (applyBar.value) applyBar.value.open(config);
}
</script>

<template>
  <main class="colbox home-main">
    <ItemCard v-for="(item, index) in items" :key="index" :config="item" @click="openCard(item)"></ItemCard>
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
