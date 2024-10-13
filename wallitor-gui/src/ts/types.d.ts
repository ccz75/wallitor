export interface wpConfig {
  name: string
  info: Info
  option: Option
}
type WallpaperType = 'Video'
export interface Info {
  media_type: WallpaperType
  description: string
  created: number
}
export interface Option {
  mute: boolean
}
export interface Cell {
  img: string
  path: string
  config: wpConfig
}
