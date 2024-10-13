import type { Store } from 'vuex'
import type {  Cell } from '@/ts/types'

declare module 'vue' {
  // 声明自己的 store state
  interface State {
    wpList:Cell[]
  }

  // 为 `this.$store` 提供类型声明
  interface ComponentCustomProperties {
    $store: Store<State>
  }
}