import { createStore } from 'vuex'
import { invoke } from '@tauri-apps/api/core';
import type {ResourceDir,wpConfig} from '@/ts/types'

function arrayBufferToString(buffer: ArrayBuffer): string {
  const decoder = new TextDecoder('utf-8');
  return decoder.decode(buffer);
}

export const store = createStore({
    state() {
        return {
            wpList:[]
        }
    },
    mutations: {
        getWpList(state){
            state.wpList = [];
            invoke("read_resource_dir", {}).then((res) => {
                const resource = JSON.parse(res as string) as ResourceDir;
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
                        state.wpList.push({
                          path: id,
                          img: imageUrl,
                          config: config
                        })
                      })
                    })
                  }
                }
              });
        }
    }
})