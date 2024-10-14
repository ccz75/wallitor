import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'
interface Params {
  url?: string,
  mute?: string
}
function getSearchParamsAsObject() {
  const params = new URLSearchParams(window.location.search)
  const paramsObject = {}
  params.forEach((value, key) => {
    paramsObject[key] = value
  })
  return paramsObject
}

window.onload = () => {
  invoke('set_wallpaper', {
    title: 'wallitor_video_playback'
  }).then((res) => {
    if(!res) {
      console.error("Unable to set wallpaper.");
      const win = getCurrentWindow();
      win.destroy();
      return;
    }
    const params: Params = getSearchParamsAsObject()
    if (params.url) {
      invoke('get_file', {
        path: params.url
      }).then((res) => {
        const binary_data_arr = new Uint8Array(res as number[])
        const blob = new Blob([binary_data_arr], { type: 'video/mp4' })
        const videoUrl = URL.createObjectURL(blob)
        const player = document.getElementById('player') as HTMLVideoElement
        if (player) {
          player.src = videoUrl
          player.play()
          if(params.mute) {
            player.muted = JSON.parse(params.mute);
          }
          player.loop = true
          setTimeout(()=>{player.style.opacity = '1'},0);
        }
      })
    }
  })
}
