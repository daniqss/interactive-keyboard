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

const isDev = import.meta.env.MODE === "development";

export const CONFIG = {
  isTauri: isTauri(),
  isDev,
  audioPath: isDev ? "../../src-tauri/assets/audio" : "/assets",
  imagePath: isDev ? "../../src-tauri/assets/image" : "/assets",
};
