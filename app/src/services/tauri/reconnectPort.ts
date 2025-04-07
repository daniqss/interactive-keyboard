import { invoke } from "@tauri-apps/api/core";

function reconnectPort() {
  invoke("reconnect_port")
    .then((message) => console.log(message))
    .catch((error) => console.error("Error reconnecting port:", error));
}

export { reconnectPort };
