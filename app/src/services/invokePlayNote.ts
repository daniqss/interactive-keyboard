import { invoke, isTauri } from "@tauri-apps/api/core";

function invokePlayNote(note: string) {
  if (isTauri()) invoke("play_note", { note });
  return isTauri();
}

export { invokePlayNote };
