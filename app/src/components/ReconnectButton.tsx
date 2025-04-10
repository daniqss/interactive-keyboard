import { reconnectPort } from "../services/tauri/reconnectPort";
import { CONFIG } from "../config";
import "../styles/Header.css";
import ReloadIcon from "./icons/ReloadIcon";

function ReconnectButton() {
  return <>{CONFIG.isTauri && <TauriReconnectButton />}</>;
}

function TauriReconnectButton() {
  return (
    <button className="reconnect-button" onClick={() => reconnectPort()}>
      <ReloadIcon width="20" height="20" strokeWidth="1.5" />
    </button>
  );
}

export default ReconnectButton;
