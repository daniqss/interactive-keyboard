import { invokePlayNote } from "./tauri/invokePlayNote";
import { Animal } from "../types/animals";
import { ANIMALS } from "../constants/animals";
const audioModules = import.meta.glob("../../src-tauri/assets/audio/*", {
  eager: true,
  query: "url",
  import: "default",
});

const sounds: Record<string, string> = ANIMALS.reduce((acc, animal) => {
  const entry = Object.entries(audioModules).find(([path]) =>
    path.endsWith(animal.sound)
  );
  if (entry) {
    const [_, url] = entry;
    acc[animal.name] = url as string;
  }

  return acc;
}, {} as Record<string, string>);

function playNote(note: string, selectedAnimal: Animal) {
  if (invokePlayNote(note)) return;

  const audioPath = sounds[selectedAnimal.name];

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
