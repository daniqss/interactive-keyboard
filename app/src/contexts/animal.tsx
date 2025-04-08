import {
  createContext,
  Dispatch,
  ReactNode,
  SetStateAction,
  useState,
} from "react";
import { Animal } from "../types/animals";
import { ANIMALS } from "../constants/animals";

export type AnimalContextType = {
  selectedAnimal: Animal;
  setSelectedAnimal: Dispatch<SetStateAction<Animal>>;
  animalList: Animal[];
};

const AnimalContext = createContext<AnimalContextType | null>(null);

function AnimalProvider({ children }: { children: ReactNode }) {
  const [animalList] = useState<Animal[]>(ANIMALS);
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
