import { isTauri } from "@tauri-apps/api/core";
import { reconnectPort } from "../services/tauri/reconnectPort";

function ReconnectButton() {
  return <>{isTauri() && <TauriReconnectButton />}</>;
}

function TauriReconnectButton() {
  return (
    <button className="reconnect-button" onClick={() => reconnectPort()}>
      reconnect
    </button>
  );
}

export default ReconnectButton;
