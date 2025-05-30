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

export const CONFIG = {
  isTauri,
  isDev,
  // with GH_PAGES works fine too but I was using GH_PAGES as bool and not as string (GH_PAGES === "true")
  audioPath: isDev
    ? "../../src-tauri/assets/audio"
    : isTauri
    ? "/assets"
    : "/interactive-keyboard/assets",
  imagePath: isDev
    ? "../../src-tauri/assets/image"
    : isTauri
    ? "/assets"
    : "/interactive-keyboard/assets",
};
