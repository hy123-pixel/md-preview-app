import { ref, computed, watch } from "vue";
import { useEditorStore } from "../stores/editor";

type Theme = "light" | "dark" | "system";

export function useTheme() {
  const store = useEditorStore();
  const systemDark = ref(false);

  const effectiveTheme = computed<"light" | "dark">(() => {
    if (store.settings.theme === "system") {
      return systemDark.value ? "dark" : "light";
    }
    return store.settings.theme;
  });

  const isDark = computed(() => effectiveTheme.value === "dark");

  function setTheme(theme: Theme) {
    store.updateSettings({ theme });
    applyTheme();
  }

  function applyTheme() {
    const html = document.documentElement;
    if (isDark.value) {
      html.classList.add("dark");
    } else {
      html.classList.remove("dark");
    }
  }

  function initSystemListener() {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    systemDark.value = mediaQuery.matches;
    mediaQuery.addEventListener("change", (e) => {
      systemDark.value = e.matches;
      applyTheme();
    });
    applyTheme();
  }

  return {
    effectiveTheme,
    isDark,
    setTheme,
    applyTheme,
    initSystemListener,
  };
}
