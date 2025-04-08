import { reconnectPort } from "../services/tauri/reconnectPort";
import { CONFIG } from "../config";

function ReconnectButton() {
  return <>{CONFIG.isTauri && <TauriReconnectButton />}</>;
}

function TauriReconnectButton() {
  return (
    <button className="reconnect-button" onClick={() => reconnectPort()}>
      reconnect
    </button>
  );
}

export default ReconnectButton;
