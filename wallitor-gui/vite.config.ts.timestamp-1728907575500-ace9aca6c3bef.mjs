// vite.config.ts
import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "file:///D:/code/wallitor/wallitor-gui/node_modules/.pnpm/vite@5.4.7_@types+node@20.16.7/node_modules/vite/dist/node/index.js";
import vue from "file:///D:/code/wallitor/wallitor-gui/node_modules/.pnpm/@vitejs+plugin-vue@5.1.4_vite@5.4.7_@types+node@20.16.7__vue@3.5.8_typescript@5.4.5_/node_modules/@vitejs/plugin-vue/dist/index.mjs";
import vueDevTools from "file:///D:/code/wallitor/wallitor-gui/node_modules/.pnpm/vite-plugin-vue-devtools@7.4.6_rollup@4.22.4_vite@5.4.7_@types+node@20.16.7__vue@3.5.8_typescript@5.4.5_/node_modules/vite-plugin-vue-devtools/dist/vite.mjs";
import ElementPlus from "file:///D:/code/wallitor/wallitor-gui/node_modules/.pnpm/unplugin-element-plus@0.8.0_rollup@4.22.4/node_modules/unplugin-element-plus/dist/vite.mjs";
import path from "path";
import { createSvgIconsPlugin } from "file:///D:/code/wallitor/wallitor-gui/node_modules/.pnpm/vite-plugin-svg-icons@2.0.1_vite@5.4.7_@types+node@20.16.7_/node_modules/vite-plugin-svg-icons/dist/index.mjs";
var __vite_injected_original_dirname = "D:\\code\\wallitor\\wallitor-gui";
var __vite_injected_original_import_meta_url = "file:///D:/code/wallitor/wallitor-gui/vite.config.ts";
var vite_config_default = defineConfig({
  clearScreen: false,
  // Tauri expects a fixed port, fail if that port is not available
  server: {
    strictPort: true
  },
  envPrefix: ["VITE_", "TAURI_PLATFORM", "TAURI_ARCH", "TAURI_FAMILY", "TAURI_PLATFORM_VERSION", "TAURI_PLATFORM_TYPE", "TAURI_DEBUG"],
  plugins: [
    vue(),
    vueDevTools(),
    createSvgIconsPlugin({
      iconDirs: [
        path.resolve(__vite_injected_original_dirname, "src/assets/svgs")
      ],
      symbolId: "icon-[name]"
    }),
    ElementPlus({
      // options
    })
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", __vite_injected_original_import_meta_url))
    }
  },
  build: {
    // Tauri uses Chromium on Windows and WebKit on macOS and Linux
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    // don't minify for debug builds
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    // 为调试构建生成源代码映射 (sourcemap)
    sourcemap: !!process.env.TAURI_DEBUG,
    rollupOptions: {
      input: {
        main: path.resolve(__vite_injected_original_dirname, "index.html"),
        video: path.resolve(__vite_injected_original_dirname, "video/index.html")
      }
    }
  }
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCJEOlxcXFxjb2RlXFxcXHdhbGxpdG9yXFxcXHdhbGxpdG9yLWd1aVwiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9maWxlbmFtZSA9IFwiRDpcXFxcY29kZVxcXFx3YWxsaXRvclxcXFx3YWxsaXRvci1ndWlcXFxcdml0ZS5jb25maWcudHNcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfaW1wb3J0X21ldGFfdXJsID0gXCJmaWxlOi8vL0Q6L2NvZGUvd2FsbGl0b3Ivd2FsbGl0b3ItZ3VpL3ZpdGUuY29uZmlnLnRzXCI7aW1wb3J0IHsgZmlsZVVSTFRvUGF0aCwgVVJMIH0gZnJvbSAnbm9kZTp1cmwnXHJcblxyXG5pbXBvcnQgeyBkZWZpbmVDb25maWcgfSBmcm9tICd2aXRlJ1xyXG5pbXBvcnQgdnVlIGZyb20gJ0B2aXRlanMvcGx1Z2luLXZ1ZSdcclxuaW1wb3J0IHZ1ZURldlRvb2xzIGZyb20gJ3ZpdGUtcGx1Z2luLXZ1ZS1kZXZ0b29scydcclxuaW1wb3J0IEVsZW1lbnRQbHVzIGZyb20gJ3VucGx1Z2luLWVsZW1lbnQtcGx1cy92aXRlJ1xyXG5pbXBvcnQgcGF0aCBmcm9tIFwicGF0aFwiO1xyXG5pbXBvcnQgeyBjcmVhdGVTdmdJY29uc1BsdWdpbiB9IGZyb20gXCJ2aXRlLXBsdWdpbi1zdmctaWNvbnNcIjtcclxuXHJcbi8vIGh0dHBzOi8vdml0ZWpzLmRldi9jb25maWcvXHJcbmV4cG9ydCBkZWZhdWx0IGRlZmluZUNvbmZpZyh7XHJcbiAgY2xlYXJTY3JlZW46IGZhbHNlLFxyXG4gIC8vIFRhdXJpIGV4cGVjdHMgYSBmaXhlZCBwb3J0LCBmYWlsIGlmIHRoYXQgcG9ydCBpcyBub3QgYXZhaWxhYmxlXHJcbiAgc2VydmVyOiB7XHJcbiAgICBzdHJpY3RQb3J0OiB0cnVlLFxyXG4gIH0sXHJcbiAgZW52UHJlZml4OiBbJ1ZJVEVfJywgJ1RBVVJJX1BMQVRGT1JNJywgJ1RBVVJJX0FSQ0gnLCAnVEFVUklfRkFNSUxZJywgJ1RBVVJJX1BMQVRGT1JNX1ZFUlNJT04nLCAnVEFVUklfUExBVEZPUk1fVFlQRScsICdUQVVSSV9ERUJVRyddLFxyXG4gIHBsdWdpbnM6IFtcclxuICAgIHZ1ZSgpLFxyXG4gICAgdnVlRGV2VG9vbHMoKSxcclxuICAgIGNyZWF0ZVN2Z0ljb25zUGx1Z2luKHtcclxuICAgICAgaWNvbkRpcnM6IFtcclxuICAgICAgICBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCBcInNyYy9hc3NldHMvc3Znc1wiKSxcclxuICAgICAgXSxcclxuICAgICAgc3ltYm9sSWQ6IFwiaWNvbi1bbmFtZV1cIlxyXG4gICAgfSksXHJcbiAgICBFbGVtZW50UGx1cyh7XHJcbiAgICAgIC8vIG9wdGlvbnNcclxuICAgIH0pLFxyXG4gIF0sXHJcbiAgcmVzb2x2ZToge1xyXG4gICAgYWxpYXM6IHtcclxuICAgICAgJ0AnOiBmaWxlVVJMVG9QYXRoKG5ldyBVUkwoJy4vc3JjJywgaW1wb3J0Lm1ldGEudXJsKSlcclxuICAgIH1cclxuICB9LFxyXG4gIGJ1aWxkOiB7XHJcbiAgICAvLyBUYXVyaSB1c2VzIENocm9taXVtIG9uIFdpbmRvd3MgYW5kIFdlYktpdCBvbiBtYWNPUyBhbmQgTGludXhcclxuICAgIHRhcmdldDogcHJvY2Vzcy5lbnYuVEFVUklfUExBVEZPUk0gPT0gJ3dpbmRvd3MnID8gJ2Nocm9tZTEwNScgOiAnc2FmYXJpMTMnLFxyXG4gICAgLy8gZG9uJ3QgbWluaWZ5IGZvciBkZWJ1ZyBidWlsZHNcclxuICAgIG1pbmlmeTogIXByb2Nlc3MuZW52LlRBVVJJX0RFQlVHID8gJ2VzYnVpbGQnIDogZmFsc2UsXHJcbiAgICAvLyBcdTRFM0FcdThDMDNcdThCRDVcdTY3ODRcdTVFRkFcdTc1MUZcdTYyMTBcdTZFOTBcdTRFRTNcdTc4MDFcdTY2MjBcdTVDMDQgKHNvdXJjZW1hcClcclxuICAgIHNvdXJjZW1hcDogISFwcm9jZXNzLmVudi5UQVVSSV9ERUJVRyxcclxuICAgIHJvbGx1cE9wdGlvbnM6IHtcclxuICAgICAgaW5wdXQ6IHtcclxuICAgICAgICBtYWluOiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCAnaW5kZXguaHRtbCcpLFxyXG4gICAgICAgIHZpZGVvOiBwYXRoLnJlc29sdmUoX19kaXJuYW1lLCAndmlkZW8vaW5kZXguaHRtbCcpLFxyXG4gICAgICB9LFxyXG4gICAgfSxcclxuICB9LFxyXG59KVxyXG4iXSwKICAibWFwcGluZ3MiOiAiO0FBQWlSLFNBQVMsZUFBZSxXQUFXO0FBRXBULFNBQVMsb0JBQW9CO0FBQzdCLE9BQU8sU0FBUztBQUNoQixPQUFPLGlCQUFpQjtBQUN4QixPQUFPLGlCQUFpQjtBQUN4QixPQUFPLFVBQVU7QUFDakIsU0FBUyw0QkFBNEI7QUFQckMsSUFBTSxtQ0FBbUM7QUFBZ0ksSUFBTSwyQ0FBMkM7QUFVMU4sSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDMUIsYUFBYTtBQUFBO0FBQUEsRUFFYixRQUFRO0FBQUEsSUFDTixZQUFZO0FBQUEsRUFDZDtBQUFBLEVBQ0EsV0FBVyxDQUFDLFNBQVMsa0JBQWtCLGNBQWMsZ0JBQWdCLDBCQUEwQix1QkFBdUIsYUFBYTtBQUFBLEVBQ25JLFNBQVM7QUFBQSxJQUNQLElBQUk7QUFBQSxJQUNKLFlBQVk7QUFBQSxJQUNaLHFCQUFxQjtBQUFBLE1BQ25CLFVBQVU7QUFBQSxRQUNSLEtBQUssUUFBUSxrQ0FBVyxpQkFBaUI7QUFBQSxNQUMzQztBQUFBLE1BQ0EsVUFBVTtBQUFBLElBQ1osQ0FBQztBQUFBLElBQ0QsWUFBWTtBQUFBO0FBQUEsSUFFWixDQUFDO0FBQUEsRUFDSDtBQUFBLEVBQ0EsU0FBUztBQUFBLElBQ1AsT0FBTztBQUFBLE1BQ0wsS0FBSyxjQUFjLElBQUksSUFBSSxTQUFTLHdDQUFlLENBQUM7QUFBQSxJQUN0RDtBQUFBLEVBQ0Y7QUFBQSxFQUNBLE9BQU87QUFBQTtBQUFBLElBRUwsUUFBUSxRQUFRLElBQUksa0JBQWtCLFlBQVksY0FBYztBQUFBO0FBQUEsSUFFaEUsUUFBUSxDQUFDLFFBQVEsSUFBSSxjQUFjLFlBQVk7QUFBQTtBQUFBLElBRS9DLFdBQVcsQ0FBQyxDQUFDLFFBQVEsSUFBSTtBQUFBLElBQ3pCLGVBQWU7QUFBQSxNQUNiLE9BQU87QUFBQSxRQUNMLE1BQU0sS0FBSyxRQUFRLGtDQUFXLFlBQVk7QUFBQSxRQUMxQyxPQUFPLEtBQUssUUFBUSxrQ0FBVyxrQkFBa0I7QUFBQSxNQUNuRDtBQUFBLElBQ0Y7QUFBQSxFQUNGO0FBQ0YsQ0FBQzsiLAogICJuYW1lcyI6IFtdCn0K
