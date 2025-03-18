import { invoke } from "@tauri-apps/api/core";

function invokePlayNote(note: string) {
  invoke("play_note", {
    note,
  });
}

export { invokePlayNote };
