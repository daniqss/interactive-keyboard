import { invoke } from "@tauri-apps/api/core";
import { Animal } from "../../types/animals";
import { CONFIG } from "../../config";

function invokeSelectAnimal(selectedAnimal: Animal) {
  if (CONFIG.isTauri) invoke("select_animal", { animal: selectedAnimal.name });
  return CONFIG.isTauri;
}

export { invokeSelectAnimal };
