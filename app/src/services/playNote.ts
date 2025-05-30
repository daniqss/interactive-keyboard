import { invokePlayNote } from "./tauri/invokePlayNote";
import { Animal } from "../types";
import { ANIMALS } from "../constants";

// use audio context instead of audio element to get same result as in rodio
const audioContext = new (window.AudioContext ??
  (window as any).webkitAudioContext)();

const audioBuffers: Record<string, AudioBuffer> = {};
ANIMALS.forEach(async (animal) => {
  const response = await fetch(animal.sound);
  const arrayBuffer = await response.arrayBuffer();
  const decodedBuffer = await audioContext.decodeAudioData(arrayBuffer);
  audioBuffers[animal.name] = decodedBuffer;
});

function playNote(note: string, selectedAnimal: Animal) {
  if (invokePlayNote(note)) return;

  const buffer = audioBuffers[selectedAnimal.name];

  if (!buffer) {
    console.error(`Audio buffer not found for animal ${selectedAnimal.name}`);
    return;
  }
  const requestedAudio = audioContext.createBufferSource();
  requestedAudio.buffer = buffer;

  requestedAudio.playbackRate.value = getSemitone(note);

  requestedAudio.connect(audioContext.destination);
  requestedAudio.start();
}

const semitones: Record<string, number> = {
  do: 0,
  re: 2,
  mi: 4,
  fa: 5,
  sol: 7,
  la: 9,
  si: 11,
  "do-sharp": 12,
};

function getSemitone(note: string): number {
  const semitone = semitones[note];
  return Math.pow(2, (semitone === undefined ? semitone : 1) / 12);
}

export { playNote };
