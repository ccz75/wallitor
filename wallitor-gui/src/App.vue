<script setup lang="ts">
import TitleBar from './components/TitleBar.vue';
import { RouterView } from 'vue-router'

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

</script>

<template>
  <title-bar mode="win"></title-bar>
  <router-view></router-view>
</template>
