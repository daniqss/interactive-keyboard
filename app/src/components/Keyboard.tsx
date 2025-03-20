import { useKeyTracker, KEYBOARD_KEYS } from "../hooks/useKeyTracker";
import "./Keyboard.css";
import { playNote } from "../services/playNote";

export default function Keyboard() {
  const onKeyPress: Record<string, () => void> = KEYBOARD_KEYS.reduce(
    (handlers, { keyPressed, note }) => {
      handlers[keyPressed] = () => playNote(note);
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
