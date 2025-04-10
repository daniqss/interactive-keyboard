import { useKeyTracker, KEYBOARD_KEYS } from "../hooks/useKeyTracker";
import "../styles/Keyboard.css";

export default function Keyboard() {
  const [keysPressed, onKeyPress] = useKeyTracker();

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
              <h3>{key.keyPressed}</h3>
              <span className="note">
                {key.note === "do-sharp" ? "do#" : key.note}
              </span>
            </button>
          </li>
        ))}
      </ul>
    </footer>
  );
}
