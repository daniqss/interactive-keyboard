import { invokePlayNote } from "./tauri/invokePlayNote";
import { Animal } from "../types";
import { ANIMALS } from "../constants";

const audioBuffers = ANIMALS.reduce<Record<string, HTMLAudioElement>>(
  (buffers, animal) => {
    buffers[animal.name] = new Audio(animal.sound);
    return buffers;
  },
  {}
);

function playNote(note: string, selectedAnimal: Animal) {
  if (invokePlayNote(note)) return;

  if (!audioBuffers[selectedAnimal.name]) {
    console.error(`Audio buffer not found for animal ${selectedAnimal.name}`);
    return;
  }
  const requestedAudio = new Audio(audioBuffers[selectedAnimal.name].src);

  requestedAudio
    .play()
    .then(() => console.log(`playing sound ${note} for ${selectedAnimal.name}`))
    .catch((error) =>
      console.error(
        `Error playing sound ${selectedAnimal.sound} for ${selectedAnimal.name}\n`,
        error
      )
    );
}

export { playNote };
