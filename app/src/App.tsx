import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useContext } from "react";
import { Animal } from "./types/animals";
import {
  AnimalContext,
  AnimalContextType,
  AnimalProvider,
} from "./contexts/animal";
import AnimalCard from "./components/AnimalCard";
import Keyboard from "./components/Keyboard";

function App() {
  const { animalList } = useContext(AnimalContext) as AnimalContextType;

  const greet = () => {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke("greet", { name: "hola" })
      .then((response) => {
        console.log(response);
      })
      .catch((error) => {
        console.error("Error:", error);
      });
  };

  return (
    <>
      <header className="header">
        <h1>Welcome to Tauri!</h1>
        <button onClick={greet}>Greet</button>
        <p>Click the button to invoke a Rust command.</p>
      </header>
      <AnimalProvider>
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
      </AnimalProvider>
    </>
  );
}

export default App;
