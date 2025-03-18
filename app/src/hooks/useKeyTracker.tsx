import { useEffect, useState } from "react";

const KEYBOARD_KEYS = [
  { keyPressed: "a", note: "do" },
  { keyPressed: "s", note: "re" },
  { keyPressed: "d", note: "mi" },
  { keyPressed: "f", note: "fa" },
  { keyPressed: "j", note: "sol" },
  { keyPressed: "k", note: "la" },
  { keyPressed: "l", note: "si" },
  { keyPressed: "ñ", note: "do-sharp" },
];

type Key = (typeof KEYBOARD_KEYS)[number]["keyPressed"];
type KeyPressHandlers = Partial<Record<Key, () => void>>;

type KeysPressed = Partial<Record<Key, boolean>>;

function useKeyTracker(onKeyPress: KeyPressHandlers = {}): KeysPressed {
  const [keysPressed, setKeysPressed] = useState<KeysPressed>({});

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (
        KEYBOARD_KEYS.some((k) => k.keyPressed === event.key) &&
        !keysPressed[event.key as Key]
      ) {
        setKeysPressed((prev) => ({ ...prev, [event.key]: true }));
        if (onKeyPress[event.key as Key]) onKeyPress[event.key as Key]!();
      }
    };

    const handleKeyUp = (event: KeyboardEvent) => {
      if (KEYBOARD_KEYS.some((k) => k.keyPressed === event.key)) {
        setKeysPressed((prev) => ({ ...prev, [event.key]: false }));
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);

    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [keysPressed, onKeyPress]);

  return keysPressed;
}

export { useKeyTracker, KEYBOARD_KEYS };
