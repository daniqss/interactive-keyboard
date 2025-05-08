import { CONFIG } from "./config";
import { Animal, Key } from "./types";

export const ANIMALS: Animal[] = [
  {
    name: "Elefante",
    sound: `${CONFIG.audioPath}/elephant_sound.wav`,
    image: `${CONFIG.imagePath}/elefante.png`,
    imageAlt:
      "Photo by Rachel Claire: https://www.pexels.com/photo/huge-elephant-with-long-trunk-standing-on-green-meadow-4577791/",
  },
  {
    name: "Tigre",
    sound: `${CONFIG.audioPath}/tiger_sound.wav`,
    image: `${CONFIG.imagePath}/tigre.png`,
    imageAlt:
      "Photo by GEORGE DESIPRIS: https://www.pexels.com/photo/tiger-looking-ferocious-2055100/",
  },
  {
    name: "Perro",
    sound: `${CONFIG.audioPath}/dog_sound.wav`,
    image: `${CONFIG.imagePath}/perro.png`,
    imageAlt:
      "Photo by Pixabay: https://www.pexels.com/photo/long-coated-white-and-black-dog-220938/",
  },
  {
    name: "Delfín",
    sound: `${CONFIG.audioPath}/dolphin_sound.wav`,
    image: `${CONFIG.imagePath}/delfin.png`,
    imageAlt:
      "Photo by Coral Grandbois: https://www.pexels.com/photo/close-up-shot-of-dolphins-in-an-aquarium-7316200/",
  },
];

export const KEYS: Key[] = CONFIG.completeKeyboard
  ? [
      { keyPressed: "a", note: "do" },
      { keyPressed: "s", note: "re" },
      { keyPressed: "d", note: "mi" },
      { keyPressed: "f", note: "fa" },
      { keyPressed: "j", note: "sol" },
      { keyPressed: "k", note: "la" },
      { keyPressed: "l", note: "si" },
      { keyPressed: "ñ", note: "do-sharp" },
    ]
  : [
      { keyPressed: "s", note: "re" },
      { keyPressed: "j", note: "sol" },
      { keyPressed: "l", note: "si" },
    ];
