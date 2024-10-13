import { invoke } from '@tauri-apps/api/core'
interface Params{
    url?:string
}
function getSearchParamsAsObject() {
  const params = new URLSearchParams(window.location.search)
  const paramsObject = {}
  params.forEach((value, key) => {
    paramsObject[key] = value
  })
  return paramsObject
}
const params:Params = getSearchParamsAsObject()
if (params.url) {
  console.log(params.url)
  invoke('get_file', {
    path: params.url
  }).then((res) => {
    console.log(res)
    const binary_data_arr = new Uint8Array(res as number[])
    const blob = new Blob([binary_data_arr], { type: 'video/mp4' })
    const videoUrl = URL.createObjectURL(blob)
    const player = document.getElementById("player") as HTMLVideoElement;
    if(player) {
        player.src = videoUrl;
        player.play();
        player.muted = true;
        player.loop = true;
    }
  })
}
