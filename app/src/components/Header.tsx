import ReconnectButton from "./ReconnectButton";
import "../styles/Header.css";

function Header({ children }: { children: React.ReactNode }) {
  return (
    <header className="header-container">
      <div></div>
      <h1>{children}</h1>
      <div>
        <ReconnectButton />
      </div>
    </header>
  );
}

export default Header;
