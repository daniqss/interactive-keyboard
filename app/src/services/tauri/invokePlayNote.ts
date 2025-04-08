import { invoke } from "@tauri-apps/api/core";
import { CONFIG } from "../../config";

function invokePlayNote(note: string) {
  if (CONFIG.isTauri) invoke("play_note_command", { note });
  return CONFIG.isTauri;
}

export { invokePlayNote };
