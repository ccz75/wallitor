<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import {invoke} from "@tauri-apps/api"
</script>

<template>
  <router-view></router-view>
</template>

<style>
html{
  background: transparent;
}
body{
  background: transparent;
}
</style>
<style scoped>
</style>
