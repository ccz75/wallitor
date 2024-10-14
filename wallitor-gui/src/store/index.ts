import { createStore } from 'vuex'
import { invoke } from '@tauri-apps/api/core'
import type { ResourceDir, wpConfig, Cell } from '@/ts/types'

function arrayBufferToString(buffer: ArrayBuffer): string {
  const decoder = new TextDecoder('utf-8')
  return decoder.decode(buffer)
}

const support_ext = ['.jpg', '.png', '.gif', '.webp']
const support_ext_map: { [key in (typeof support_ext)[number]]: string } = {
  '.jpg': 'image/jpeg',
  '.png': 'image/png',
  '.gif': 'image/gif',
  '.webp': 'image/webp'
}

export const store = createStore({
  state() {
    return {
      wpList: [] as Cell[]
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
    }
  }
})
