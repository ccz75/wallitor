import { createStore } from 'vuex'
import { invoke } from '@tauri-apps/api/core'
import type { ResourceDir, wpConfig, Cell, Settings } from '@/ts/types'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { emit } from '@tauri-apps/api/event'
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'
import { ElMessage } from 'element-plus'

function arrayBufferToString(buffer: ArrayBuffer): string {
  const decoder = new TextDecoder('utf-8')
  return decoder.decode(buffer)
}

function check_auto_start(settings: Settings) {
  const status = settings.auto_start
  isEnabled().then((current) => {
    if (current == true && status == false) {
      disable().then(() =>
        ElMessage({
          type: 'success',
          message: '已经禁用开机自启'
        })
      )
    } else if (current == false && status == true) {
      enable().then(() =>
        ElMessage({
          type: 'success',
          message: '已经启用开机自启'
        })
      )
    }
  })
}

const support_ext = ['.jpg', '.png', '.gif', '.webp']
const support_ext_map: { [key in (typeof support_ext)[number]]: string } = {
  '.jpg': 'image/jpeg',
  '.png': 'image/png',
  '.gif': 'image/gif',
  '.webp': 'image/webp'
}

type VideoWindow = null | WebviewWindow
export const store = createStore({
  state() {
    return {
      wpList: [] as Cell[],
      videoWindow: null as VideoWindow,
      settings: {
        title_bar: 'win',
        auto_pause: true,
        auto_start: false,
        version: '1.0.0'
      } as Settings
    }
  },
  mutations: {
    getWpList(state) {
      state.wpList = []
      invoke('read_resource_dir', {}).then((res) => {
        const resource = JSON.parse(res as string) as ResourceDir
        for (const id of Object.keys(resource.files)) {
          const dir = resource.files[id]
          if ('config.json' in dir)
            invoke('get_file', {
              path: `${id}\\config.json`
            }).then((cfg) => {
              const config: wpConfig = JSON.parse(arrayBufferToString(cfg as ArrayBuffer))
              let hasPreview = false
              for (const ext of support_ext) {
                const filename = 'preview' + ext
                if (filename in dir) {
                  hasPreview = true
                  invoke('get_file', {
                    path: dir[filename]
                  }).then((res) => {
                    const binary_data_arr = new Uint8Array(res as number[])
                    const blob = new Blob([binary_data_arr], {
                      type: support_ext_map[ext]
                    })
                    const imageUrl = URL.createObjectURL(blob)
                    invoke('get_file', {
                      path: `${id}\\config.json`
                    }).then((cfg) => {
                      const config: wpConfig = JSON.parse(arrayBufferToString(cfg as ArrayBuffer))
                      state.wpList.push({
                        path: id,
                        img: imageUrl,
                        config: config
                      })
                    })
                  })
                }
              }
              if (!hasPreview) {
                state.wpList.push({
                  path: id,
                  img: '',
                  config: config
                })
              }
            })
        }
      })
    },
    new_wallpaper_window(
      state,
      payload: {
        title: string
        url: string
      }
    ) {
      const window_options = {
        title: payload.title,
        url: payload.url,
        fullscreen: true,
        shadow: false,
        decorations: false,
        transparent: true
      }
      state.videoWindow = new WebviewWindow(payload.title, window_options)
    }
  },
  actions: {
    async apply_wallpaper(
      { state, commit },
      payload: {
        title: string
        url: string
      }
    ) {
      return new Promise<boolean>((resolve) => {
        if (state.videoWindow) {
          state.videoWindow.destroy().then(() => {
            commit('new_wallpaper_window', payload)
            resolve(true)
          })
        } else {
          commit('new_wallpaper_window', payload)
          resolve(true)
        }
      })
    },
    async get_settings({ state }) {
      const settings = await invoke('get_settings')
      state.settings = JSON.parse(settings as string) as Settings
      check_auto_start(state.settings)
      return
    },
    async set_settings({ state }, payload: Settings) {
      state.settings = payload
      const settings: boolean = await invoke('set_settings', {
        settings: state.settings
      })
      if (state.settings.auto_pause) {
        await emit('start_auto_pause')
      } else await emit('stop_auto_pause')
      check_auto_start(state.settings)
      return settings
    }
  }
})
