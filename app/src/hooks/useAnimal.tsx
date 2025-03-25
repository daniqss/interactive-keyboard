import { useContext, useEffect } from "react";
import { AnimalContext, AnimalContextType } from "../contexts/animal";
import { invokeSelectAnimal } from "../services/tauri/invokeSelectAnimal";

function useAnimal() {
  const { selectedAnimal } = useContext(AnimalContext) as AnimalContextType;

  useEffect(() => {
    if (invokeSelectAnimal(selectedAnimal)) return;
  }, [selectedAnimal]);
}

export default useAnimal;
