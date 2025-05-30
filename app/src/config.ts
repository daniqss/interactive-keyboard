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

const { GH_PAGES, MODE } = import.meta.env;
const isDev = MODE === "development";
const isTauri = isTauriFn();

export const CONFIG = {
  isTauri,
  isDev,
  // with GH_PAGES works fine too but I was using GH_PAGES as bool and not as string (GH_PAGES === "true")
  audioPath: isDev
    ? "../../src-tauri/assets/audio"
    : GH_PAGES === "true"
    ? "/interactive-keyboard/assets"
    : "/assets",

  imagePath: isDev
    ? "../../src-tauri/assets/image"
    : GH_PAGES === "true"
    ? "/interactive-keyboard/assets"
    : "/assets",
};
