<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import {invoke} from "@tauri-apps/api"
import { appWindow } from '@tauri-apps/api/window'
import SvgIcon from './components/SvgIcon.vue';
import {ref} from 'vue'

const maximized = ref(false);

function minimize(){
  appWindow.minimize()
}

function toggleMaximize(){
  appWindow.toggleMaximize();
  maximized.value = !maximized.value;
}

function close(){
  appWindow.close()
}
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
  <div class="titlebar-button" id="titlebar-minimize" @click="minimize">
    <svg-icon name="window-minimize" color="red"></svg-icon>
  </div>
  <div class="titlebar-button" id="titlebar-maximize" @click="toggleMaximize">
    <template v-if="maximized">
      <svg-icon name="window-maximized"></svg-icon>
    </template>
    <template v-else>
      <svg-icon name="window-maximize" width="13px"></svg-icon>
    </template>
  </div>
  <div class="titlebar-button" id="titlebar-close" @click="close">
    <svg-icon name="window-close"></svg-icon>
  </div>
</div>
  <router-view></router-view>
</template>

<style>
html{
  background: transparent;
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
body{
  width: 100%;
  height: 100%;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
#app{
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
}
</style>
<style scoped>
.titlebar {
  height: 30px;
  background: #141414;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>
