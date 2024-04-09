import { acceptHMRUpdate, defineStore } from "pinia";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api";

type window = [number, string];

type Settings = {
  isActive: boolean;
  interval: number;
  isRandomized: boolean;
  randomizedSpeed: {
    from: number;
    to: number;
  };
  hotkey: string[];
  windowDetection: boolean;
  windowInfo: window;
  lockableWindows: window[];
};

export const useSettingStore = defineStore("window", () => {
  const settings = reactive<Settings>({
    isActive: false,
    interval: 1000,
    isRandomized: false,
    randomizedSpeed: {
      from: 1000,
      to: 1000,
    },
    hotkey: [],
    windowDetection: false,
    windowInfo: [0, ""],
    lockableWindows: [],
  });

  async function populateLockableWindows() {
    const windows = await invoke<window[]>("get_windows");
    settings.lockableWindows = windows.filter(
      (window) => window[1] !== "Oroboros"
    );
  }

  listen<string>("autoclicker_status", (event) => {
    settings.isActive = JSON.parse(event.payload);
  });

  watch(
    () => settings.interval,
    (value) => {
      invoke("set_interval", { interval: value });
    }
  );

  watch(
    () => settings.hotkey,
    (value) => {
      invoke("set_hotkey", { hotkey: value });
    }
  );

  watch(
    () => settings.windowDetection,
    (value) => {
      invoke("set_window_detection", { windowDetection: value });
    }
  );

  watch(
    () => settings.windowInfo,
    (value) => {
      invoke("set_window_hwnd", { hwnd: value[0] });
    }
  );

  return {
    settings,
    populateLockableWindows,
  };
});

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useSettingStore, import.meta.hot));
}
