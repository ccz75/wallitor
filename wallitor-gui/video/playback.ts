import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
interface Params {
  url?: string
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

listen('video_pause', () => {
  const player = document.getElementById('player') as HTMLVideoElement
  if (player) player.pause()
})

class player{
  playing:boolean
  params:Params
  interval:number
  constructor(params:Params) {
    this.params = params;
    this.playing = false;
    this.interval = 0;
  }
  init_player(videoUrl:string){
    const player = document.getElementById('player') as HTMLVideoElement
    if (player) {
      player.src = videoUrl
      player.play()
      this.playing = true;
      if (this.params.mute) {
        player.muted = JSON.parse(this.params.mute)
      }
      player.loop = true
      setTimeout(() => {
        player.style.opacity = '1'
      }, 0)
    }
    this.detect_zoom();
  }
  detect_zoom(){
    setInterval(() => {
      invoke('any_zoomed').then((res) => {
        if (res as boolean) {
          if(this.playing) {
            const player = document.getElementById('player') as HTMLVideoElement
            if (player) {
              player.pause();
              this.playing = false;
            }
          }
        }
        else{
          if(!this.playing) {
            const player = document.getElementById('player') as HTMLVideoElement
            if (player) {
              player.play();
              this.playing = true;
            }
          }
        }
      })
    }, 1000)
  }
} 

let player_instance:player | null = null;

window.onload = () => {
  invoke('set_wallpaper', {
    title: 'wallitor_video_playback'
  }).then((res) => {
    if (!res) {
      console.error('Unable to set wallpaper.')
      const win = getCurrentWindow()
      win.destroy()
      return
    }
    const params: Params = getSearchParamsAsObject()
    if (params.url) {
      invoke('get_file', {
        path: params.url
      }).then((res) => {
        const binary_data_arr = new Uint8Array(res as number[])
        const blob = new Blob([binary_data_arr], { type: 'video/mp4' })
        const videoUrl = URL.createObjectURL(blob)
        player_instance = new player(params);
        player_instance.init_player(videoUrl);
      })
    }
  })
}
