import './styles/app.css';
import { mount } from 'svelte';
import App from './lib/App.svelte';

// 初始化应用
const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
