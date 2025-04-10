import { useContext, useEffect, useState } from "react";
import { KEYS as KEYBOARD_KEYS } from "../constants";
import { playNote } from "../services/playNote";
import { AnimalContext, AnimalContextType } from "../contexts/animal";

type Key = (typeof KEYBOARD_KEYS)[number]["keyPressed"];

type KeysPressed = Partial<Record<Key, boolean>>;

function useKeyTracker() {
  const [keysPressed, setKeysPressed] = useState<KeysPressed>({});
  const { selectedAnimal } = useContext(AnimalContext) as AnimalContextType;

  const onKeyPress: Record<string, () => void> = KEYBOARD_KEYS.reduce(
    (handlers, { keyPressed, note }) => {
      handlers[keyPressed] = () => playNote(note, selectedAnimal);
      return handlers;
    },
    {} as Record<string, () => void>
  );

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

  return [keysPressed, onKeyPress] as const;
}

export { useKeyTracker, KEYBOARD_KEYS };
