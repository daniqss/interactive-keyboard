import { invokePlayNote } from "./tauri/invokePlayNote";
import { Animal } from "../types/animals";
import { ANIMALS } from "../constants/animals";
import { CONFIG } from "../config";
const audioModules = import.meta.glob("../../src-tauri/assets/audio/*", {
  eager: true,
  query: "url",
  import: "default",
});

const sounds: Record<string, string> = {};

for (const animal of ANIMALS) {
  for (const path in audioModules) {
    if (path.endsWith(animal.sound)) {
      sounds[animal.name] = audioModules[path] as string;
      break;
    }
  }
}

function playNote(note: string, selectedAnimal: Animal) {
  if (invokePlayNote(note)) return;

  const audioPath = `${CONFIG.audioPath}/${selectedAnimal.sound}`;

  console.log(sounds, audioModules, audioPath);

  new Audio(audioPath)
    .play()
    .then(() => console.log(`playing sound ${note} for ${selectedAnimal.name}`))
    .catch((error) =>
      console.error(
        `Error playing sound ${audioPath} for ${selectedAnimal.name}\n`,
        error
      )
    );
}

export { playNote };
