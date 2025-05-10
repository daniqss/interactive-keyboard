import { reconnectPort } from "../services/tauri/reconnectPort";
import { CONFIG } from "../config";
import "../styles/Header.css";
import ReloadIcon from "./icons/ReloadIcon";
import { useEffect, useState } from "react";

function ReconnectButton() {
  return <>{CONFIG.isTauri && <TauriReconnectButton />}</>;
}

function TauriReconnectButton() {
  const [port, setPort] = useState<string | null>(null);

  // on component mount check if port is already connected
  useEffect(() => {
    reconnectPort().then((message) => {
      setPort(message);
    });
  }, []);

  const handleReconnect = async () => {
    const message = await reconnectPort();
    setPort(message);
  };

  return (
    <>
      <button className="reconnect-button" onClick={handleReconnect}>
        <ReloadIcon width="20" height="20" strokeWidth="1.5" />
      </button>
      {port && (
        <div className="reconnect-message">
          <p>{port}</p>
        </div>
      )}
    </>
  );
}

export default ReconnectButton;
