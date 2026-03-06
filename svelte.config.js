import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

export default {
  // 为Svelte组件启用TypeScript
  preprocess: vitePreprocess(),

  // 编译器选项
  compilerOptions: {
    // 启用运行时检查
    runes: true,
    // 启用开发时警告
    dev: true,
    // 启用CSS作用域
    css: 'external',
    // 启用访问性警告
    accessibility: true,
    // 启用编译器警告
    warnings: {
      // 忽略a11y警告（开发时）
      'a11y-missing-attribute': false,
    },
  },

  // 扩展配置
  extensions: ['.svelte'],

  // 热重载
  hot: true,

  // 禁用内联样式（使用外部CSS文件）
  emitCss: true,
};
