import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useState,
} from "react";
import { Animal } from "../types/animals";

export type AnimalContextType = {
  selectedAnimal: Animal;
  setSelectedAnimal: Dispatch<SetStateAction<Animal>>;
  animalList: Animal[];
};

const AnimalContext = createContext<AnimalContextType | null>(null);

function AnimalProvider({ children }: { children: ReactNode }) {
  const [animalList] = useState<Animal[]>([
    {
      name: "Elefante",
      sound: "elephant_sound.mp3",
      image: "./assets/images/elephant.png",
    },
    {
      name: "Tigre",
      sound: "tiger_sound.wav",
      image: "./assets/images/tiger.png",
    },
    {
      name: "Perro",
      sound: "dog_sound.wav",
      image: "./assets/images/dog.png",
    },
    {
      name: "Delfín",
      sound: "dolphin_sound.wav",
      image: "./assets/images/dolphin.png",
    },
  ]);
  const [selectedAnimal, setSelectedAnimal] = useState<Animal>(animalList[0]);

  const value = {
    selectedAnimal,
    setSelectedAnimal,
    animalList,
  };

  return (
    <AnimalContext.Provider value={value}>{children}</AnimalContext.Provider>
  );
}

export { AnimalContext, AnimalProvider };
