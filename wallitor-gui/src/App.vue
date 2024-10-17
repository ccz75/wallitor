<script setup lang="ts">
import { onBeforeMount } from 'vue';
import TitleBar from './components/TitleBar.vue';
import { RouterView } from 'vue-router'
import { useStore } from 'vuex';

const isDarkTheme = window.matchMedia("(prefers-color-scheme: dark)");
const html_node = document.getElementsByTagName("html")[0]
if (isDarkTheme.matches) html_node.classList.add("dark")
isDarkTheme.addEventListener('change', (event) => {
  if (event.matches) {
    html_node.classList.add("dark")
  } else {
    html_node.classList.remove("dark")
  }
});

onBeforeMount(() => {
  useStore().dispatch("get_settings");
})

</script>

<template>
  <title-bar></title-bar>
  <router-view></router-view>
</template>
