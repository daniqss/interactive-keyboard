import { isTauri as isTauriFn } from "@tauri-apps/api/core";

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

const { MODE } = import.meta.env;
const isDev = MODE === "development";
const isTauri = isTauriFn();

const ghPagesAssetsPath = "/interactive-keyboard/assets";
const tauriAssetsPath = "/assets";

export const CONFIG = {
  isTauri,
  isDev,
  audioPath: isDev
    ? "../../src-tauri/assets/audio"
    : isTauri === true
    ? tauriAssetsPath
    : ghPagesAssetsPath,
  imagePath: isDev
    ? "../../src-tauri/assets/image"
    : isTauri === true
    ? tauriAssetsPath
    : ghPagesAssetsPath,
};
