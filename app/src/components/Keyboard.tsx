const keyboardKeys = [
  {
    keyPressed: "a",
    note: "Do",
  },
  {
    keyPressed: "s",
    note: "Re",
  },
  {
    keyPressed: "d",
    note: "Mi",
  },
  {
    keyPressed: "f",
    note: "Fa",
  },
  {
    keyPressed: "j",
    note: "Sol",
  },
  {
    keyPressed: "k",
    note: "La",
  },
  {
    keyPressed: "l",
    note: "Si",
  },
  {
    keyPressed: "ñ",
    note: "Do#",
  },
];

function Keyboard() {
  return (
    <section className="keyboard">
      <ul>
        {keyboardKeys.map((key) => (
          <li key={key.keyPressed} className={`key ${key.note}`} />
        ))}
      </ul>
    </section>
  );
}

export default Keyboard;
