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

const { GH_PAGES, MODE } = import.meta.env;
const isDev = MODE === "development";

export const CONFIG = {
  isTauri: isTauri(),
  isDev,
  audioPath: isDev
    ? "../../src-tauri/assets/audio"
    : GH_PAGES
    ? "/interactive-keyboard/assets"
    : "/assets",
  imagePath: isDev
    ? "../../src-tauri/assets/image"
    : GH_PAGES
    ? "/interactive-keyboard/assets"
    : "/assets",
};
