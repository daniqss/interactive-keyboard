import { useKeyTracker, KEYBOARD_KEYS } from "../hooks/useKeyTracker";
import { invoke } from "@tauri-apps/api/core";
import "./Keyboard.css";

export default function Keyboard() {
  const onKeyPress: Record<string, () => void> = KEYBOARD_KEYS.reduce(
    (handlers, { keyPressed, note }) => {
      handlers[keyPressed] = async () =>
        invoke("play_note", {
          note,
        });
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
              onClick={() => console.log(key.note)}
            >
              {key.keyPressed}
            </button>
          </li>
        ))}
      </ul>
    </footer>
  );
}
