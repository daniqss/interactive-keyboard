import { useContext } from "react";
import { Animal } from "../types";
import { AnimalContext, AnimalContextType } from "../contexts/animal";

type AnimalCardProps = {
  animal: Animal;
};

function AnimalCard({ animal }: AnimalCardProps) {
  const { selectedAnimal, setSelectedAnimal } = useContext(
    AnimalContext
  ) as AnimalContextType;

  return (
    <button
      className={`animal-card ${
        animal === selectedAnimal ? "selected-animal" : ""
      }`}
      onClick={() => setSelectedAnimal(animal)}
    >
      <h2>{animal.name}</h2>
    </button>
  );
}

export default AnimalCard;
