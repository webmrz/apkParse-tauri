import { ref } from 'vue';

export type Theme = 'light' | 'dark';

class ThemeManager {
  private static instance: ThemeManager;
  private _theme = ref<Theme>('light');

  private constructor() {
    this.init();
  }

  public static getInstance(): ThemeManager {
    if (!ThemeManager.instance) {
      ThemeManager.instance = new ThemeManager();
    }
    return ThemeManager.instance;
  }

  private init() {
    // 1. 首先检查用户之前的选择
    const savedTheme = localStorage.getItem('theme') as Theme;
    
    // 2. 如果没有保存的主题，则检查系统主题
    if (!savedTheme) {
      const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      this._theme.value = prefersDark ? 'dark' : 'light';
    } else {
      this._theme.value = savedTheme;
    }

    // 3. 应用主题
    this.applyTheme();

    // 4. 监听系统主题变化
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (!localStorage.getItem('theme')) {
        this._theme.value = e.matches ? 'dark' : 'light';
        this.applyTheme();
      }
    });
  }

  private applyTheme() {
    document.documentElement.classList.toggle('dark', this._theme.value === 'dark');
    localStorage.setItem('theme', this._theme.value);
  }

  public get theme() {
    return this._theme;
  }

  public toggleTheme() {
    this._theme.value = this._theme.value === 'light' ? 'dark' : 'light';
    this.applyTheme();
  }

  public setTheme(theme: Theme) {
    this._theme.value = theme;
    this.applyTheme();
  }
}

export const themeManager = ThemeManager.getInstance(); 