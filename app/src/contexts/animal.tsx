import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useState,
} from "react";
import { Animal } from "../types/animals";

export type AnimalContextType = {
  selectedAnimal: Animal | null;
  setSelectedAnimal: Dispatch<SetStateAction<Animal | null>>;
  animalList: Animal[];
};

const AnimalContext = createContext<AnimalContextType | null>(null);

function AnimalProvider({ children }: { children: ReactNode }) {
  const [selectedAnimal, setSelectedAnimal] = useState<Animal | null>(null);
  const [animalList] = useState<Animal[]>([
    {
      name: "Elefante",
      sound: "./assets/sounds/elephant.mp3",
      image: "./assets/images/elephant.png",
    },
    {
      name: "Tigre",
      sound: "./assets/sounds/tiger.mp3",
      image: "./assets/images/tiger.png",
    },
    {
      name: "Perro",
      sound: "./assets/sounds/dog.mp3",
      image: "./assets/images/dog.png",
    },
    {
      name: "Delfín",
      sound: "./assets/sounds/dolphin.mp3",
      image: "./assets/images/dolphin.png",
    },
  ]);

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
