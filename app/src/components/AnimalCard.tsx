import { useContext } from "react";
import { Animal } from "../types";
import { AnimalContext, AnimalContextType } from "../contexts/animal";
import { CONFIG } from "../config";

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
      <img
        src={`${CONFIG.imagePath}/${animal.image}`}
        alt={animal.imageAlt}
        className="animal-image"
      />
    </button>
  );
}

export default AnimalCard;
