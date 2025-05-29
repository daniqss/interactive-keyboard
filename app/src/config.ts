import { isTauri } from "@tauri-apps/api/core";

import.meta.glob("../src-tauri/assets/audio/*", {
  eager: true,
  query: "url",
  import: "default",
});
import.meta.glob("../src-tauri/assets/image/*", {
  eager: true,
  query: "url",
  import: "default",
});

const { VITE_COMPLETE, MODE } = import.meta.env;
const isDev = MODE === "development";
const completeKeyboard = VITE_COMPLETE === "true";

export const CONFIG = {
  completeKeyboard,
  isTauri: isTauri(),
  isDev,
  audioPath: isDev
    ? "../../src-tauri/assets/audio"
    : "/interactive-keyboard/assets",
  imagePath: isDev
    ? "../../src-tauri/assets/image"
    : "/interactive-keyboard/assets",
};
