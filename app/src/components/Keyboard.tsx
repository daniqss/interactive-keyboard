import { useContext } from "react";
import { useKeyTracker, KEYBOARD_KEYS } from "../hooks/useKeyTracker";
import { AnimalContext, AnimalContextType } from "../contexts/animal";
import { playNote } from "../services/playNote";
import "./Keyboard.css";

export default function Keyboard() {
  const { selectedAnimal } = useContext(AnimalContext) as AnimalContextType;

  const onKeyPress: Record<string, () => void> = KEYBOARD_KEYS.reduce(
    (handlers, { keyPressed, note }) => {
      handlers[keyPressed] = async () => playNote(note, selectedAnimal);
      return handlers;
    },
    {} as Record<string, () => void>
  );

  const keysPressed = useKeyTracker(onKeyPress);

  return (
    <footer className="keyboard">
      <ul>
        {KEYBOARD_KEYS.map((key) => (
          <li key={key.keyPressed}>
            <button
              className={`key ${key.note} ${
                keysPressed[key.keyPressed] ? "active" : ""
              }`}
              onClick={onKeyPress[key.keyPressed]}
            >
              {key.keyPressed}
            </button>
          </li>
        ))}
      </ul>
    </footer>
  );
}
