import { createPinia } from 'pinia';
import { createApp } from 'vue';

import App from './App.vue';
import './styles/tokens.css';
import './styles/base.css';
import './styles/ambient.css';
import './styles/glass.css';

createApp(App).use(createPinia()).mount('#app');
