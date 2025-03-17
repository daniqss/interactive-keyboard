import { useContext } from "react";
import { AnimalContext, AnimalContextType } from "../contexts/animal";
import { Animal } from "../types/animals";

type AnimalCardProps = {
  animal: Animal;
};

function AnimalCard({ animal }: AnimalCardProps) {
  const { selectedAnimal, setSelectedAnimal } = useContext(
    AnimalContext
  ) as AnimalContextType;

  const handleAnimalSelection = (animal: Animal) => {
    setSelectedAnimal(animal);
  };

  return (
    <button
      className={`animal-card ${
        animal === selectedAnimal ? "selected-animal" : ""
      }`}
      onClick={() => handleAnimalSelection(animal)}
    >
      <h2>{animal.name}</h2>
    </button>
  );
}

export default AnimalCard;
