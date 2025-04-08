import { isTauri } from "@tauri-apps/api/core";

const isDev = import.meta.env.MODE === "development";

export const CONFIG = {
  isTauri: isTauri(),
  isDev,
  audioPath: isDev ? "../../src-tauri/assets/audio" : "/assets",
};
