import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import "virtual:svg-icons-register";
import globalComponents from '@/components/install'
import "@/style/global.css"

const app = createApp(App)

app.use(router);
app.use(globalComponents);

app.mount('#app')
