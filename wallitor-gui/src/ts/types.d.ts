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
  entry_point: string
}
export interface Option {
  mute: boolean
}
export interface Cell {
  img: string
  path: string
  config: wpConfig
}

export interface Resource {
  'config.json': string
  [filename: string]: string
}

export interface ResourceDir {
  files: {
    [resId: string]: Resource
  }
}

interface AddInfo {
  name: string
  preview: string
  media: string
  description: string
}

interface EditInfo {
  path: string
  name: string
  preview: string
  description: string
}
