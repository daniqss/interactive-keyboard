import { invokePlayNote } from "./tauri/invokePlayNote";
import { Animal } from "../types/animals";

function playNote(note: string, selectedAnimal: Animal) {
  if (invokePlayNote(note)) return;

  new Audio(`../../src-tauri/assets/${selectedAnimal.sound}`)
    .play()
    .then(() =>
      console.log(`playing sound: ${note} for ${selectedAnimal.name}`)
    )
    .catch((error) =>
      console.error("Error playing sound:", error, selectedAnimal)
    );
}

export { playNote };
