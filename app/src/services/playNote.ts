import { isTauri } from "@tauri-apps/api/core";
import { invokePlayNote } from "./invokePlayNote";

function playNote(note: string) {
  if (isTauri()) {
    invokePlayNote(note);
  } else {
    const audio = new Audio(`../../assets/elephant_sound.mp3`);
    audio
      .play()
      .then(() => console.log(`playing sound: ${note}`))
      .catch((error) => {
        console.error("Error playing sound:", error);
      });
  }
}

export { playNote };
