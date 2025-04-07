import { invoke, isTauri } from "@tauri-apps/api/core";
import { Animal } from "../../types/animals";

function invokeSelectAnimal(selectedAnimal: Animal) {
  if (isTauri()) invoke("select_animal", { animal: selectedAnimal.name });
  return isTauri();
}

export { invokeSelectAnimal };
