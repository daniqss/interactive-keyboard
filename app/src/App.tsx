import "./App.css";
import { useContext } from "react";
import { Animal } from "./types/animals";
import { AnimalContext, AnimalContextType } from "./contexts/animal";
import AnimalCard from "./components/AnimalCard";
import Keyboard from "./components/Keyboard";
import useAnimal from "./hooks/useAnimal";

function App() {
  const { animalList } = useContext(AnimalContext) as AnimalContextType;
  useAnimal();

  return (
    <main className="main-container">
      <section className="animal-selection">
        <ul>
          {animalList.map((animal: Animal) => {
            return (
              <li key={animal.name}>
                <AnimalCard animal={animal} />
              </li>
            );
          })}
        </ul>
      </section>
      <Keyboard />
    </main>
  );
}

export default App;
