import { invoke } from "@tauri-apps/api/core";

async function reconnectPort(): Promise<string> {
  return invoke<string>("reconnect_port")
    .then((message) => message)
    .catch((error) => {
      console.error("Error reconnecting port:", error);
      return "Unable to reconnect port";
    });
}

export { reconnectPort };
