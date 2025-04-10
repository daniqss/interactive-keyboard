import { useContext } from "react";
import { Animal } from "../types";
import { AnimalContext, AnimalContextType } from "../contexts/animal";
import "../styles/AnimalCard.css";

type AnimalCardProps = {
  animal: Animal;
};

function AnimalCard({ animal }: AnimalCardProps) {
  const { selectedAnimal, setSelectedAnimal } = useContext(
    AnimalContext
  ) as AnimalContextType;

  const isSelected = animal === selectedAnimal;

  return (
    <>
      <button
        className={`animal-card ${isSelected ? "selected-animal" : ""}`}
        onClick={() => setSelectedAnimal(animal)}
      >
        <div className="image-container">
          <img
            src={animal.image}
            alt={animal.imageAlt}
            className="animal-image"
          />
        </div>
        <div className="animal-name">
          <h2>{animal.name}</h2>
        </div>
      </button>
      {isSelected && <div className="animal-shadow" />}
    </>
  );
}

export default AnimalCard;
