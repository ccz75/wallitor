<template>
  <div data-tauri-drag-region class="titlebar colbox">
    <div class="titlebar-icon-wrapper colbox">
      <template v-if="mode == 'mac'">
        <div class="titlebar-button titlebar-close button-style-mac" @click="close">
          <svg-icon name="window-close" :size="button_size_mac"></svg-icon>
        </div>
        <div class="titlebar-button titlebar-minimize button-style-mac" @click="minimize">
          <svg-icon name="window-minimize" :size="button_size_mac"></svg-icon>
        </div>
        <div class="titlebar-button titlebar-maximize button-style-mac" @click="toggleMaximize">
          <svg-icon name="window-maximize" :size="button_size_mac"></svg-icon>
        </div>
      </template>
      <img src="@/assets/vw.png" class="titlebar-icon">
      <div class="titlebar-icon-title">
        Wallitor
      </div>
    </div>

    <div class="titlebar-button-wrapper colbox">
      <div class="titlebar-button" id="titlebar-settings" @click="openSettings">
        <div class="titlebar-button-rect">
          <svg-icon name="setting" :size="button_size_default"></svg-icon>
        </div>
      </div>
      <template v-if="mode == 'win'">
        <div class="titlebar-button titlebar-minimize button-style-win" @click="minimize">
          <div class="titlebar-button-rect">
            <svg-icon name="window-minimize" :size="button_size_default"></svg-icon>
          </div>
        </div>
        <div class="titlebar-button titlebar-maximize button-style-win" @click="toggleMaximize">
          <template v-if="maximized">
            <div class="titlebar-button-rect">
              <svg-icon name="window-maximized" :size="button_size_alter"></svg-icon>
            </div>
          </template>
          <template v-else>
            <div class="titlebar-button-rect">
              <svg-icon name="window-maximize" :size="button_size_alter"></svg-icon>
            </div>
          </template>
        </div>
        <div class="titlebar-button titlebar-close button-style-win" @click="close">
          <div class="titlebar-button-rect">
            <svg-icon name="window-close" :size="button_size_default"></svg-icon>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import SvgIcon from '@/components/SvgIcon.vue';
import { ref, defineProps } from 'vue'
import type { PropType } from 'vue'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { useRouter } from 'vue-router';
const appWindow = getCurrentWebviewWindow();
const router = useRouter();

type Mode = "win" | "mac";

const maximized = ref(false);
const button_size_default = ref("18px");
const button_size_alter = ref("15px")
const button_size_mac = ref("11px")

const props = defineProps({
  mode: {
    type: String as PropType<Mode>,
    default: "win"
  }
})

function minimize() {
  appWindow.minimize()
}

function toggleMaximize() {
  appWindow.toggleMaximize();
  maximized.value = !maximized.value;
}

function close() {
  appWindow.hide();
}

function openSettings() {
  router.push("settings");
}
</script>

<style>
.titlebar {
  position: relative;
  justify-content: space-between;
  height: var(--titlebar-height);
  padding: 4px;
  color: var(--text-color);
}

.titlebar-icon-wrapper {
  height: 100%;
  width: fit-content;
  align-items: center;
  margin-left: 5px;
  margin-right: 5px;
}

.titlebar-icon-title {
  font-weight: 500;
  font-size: 23px;
  margin-left: 10px;
}

.titlebar-icon {
  height: var(--titlebar-height);
}

.titlebar-button-wrapper {
  height: calc(var(--titlebar-height) - 4px);
  margin-top: 5px;
  margin-right: 5px;
  width: fit-content;
  place-self: center;
  place-items: center;
  overflow: hidden;
  border-radius: var(--titlebar-height);
  border: solid 1px var(--bd-color);
  backdrop-filter: blur(10px) saturate(180%);
  box-shadow: var(--shadow-edge-glow), var(--shadow);
  background-color: var(--bg-color-alpha);
  z-index: 300;
}

.button-style-win.titlebar-button {
  height: 30px;
  width: 30px;
  padding-top: 3px;
  padding-bottom: 3px;
}

.titlebar-button-rect {
  height: 30px;
  width: 30px;
  border-radius: 100%;
  transition: .3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

#titlebar-settings {
  cursor: pointer;
}

.button-style-win#titlebar-settings {
  padding-left: 5px;
}

.button-style-win.titlebar-close {
  padding-right: 5px;
}

#titlebar-settings .titlebar-button-rect:hover,
.button-style-win.titlebar-minimize .titlebar-button-rect:hover,
.button-style-win.titlebar-maximize .titlebar-button-rect:hover {
  background-color: var(--bg-hover-fill);
}

.button-style-win.titlebar-close .titlebar-button-rect:hover {
  background-color: var(--bg-hover-fill-close);
}


.button-style-mac.titlebar-button {
  width: 13px;
  height: 13px;
  border-radius: 100%;
  margin-left: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #000000b4
}

.button-style-mac.titlebar-minimize {
  background: #FFBB39;
  border: 1px solid #CFA64E;
}

.button-style-mac.titlebar-maximize {
  background: #606060;
  border: 1px solid #656565;
  margin-right: 8px;
}

.button-style-mac:hover.titlebar-maximize {
  background: #00CD4E;
  border: 1px solid #0EA642;
}

.button-style-mac.titlebar-close {
  background: #FF5D5B;
  border: 1px solid #CF544D;
}

.button-style-mac svg {
  opacity: 0;
  transition: .2s;
}

.button-style-mac:hover svg {
  opacity: 1;
}
</style>