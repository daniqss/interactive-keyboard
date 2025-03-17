export type Animal =
  | {
      name: "Elefante";
      sound: "./assets/sounds/elephant.mp3";
      image: "./assets/images/elephant.png";
    }
  | {
      name: "Tigre";
      sound: "./assets/sounds/tiger.mp3";
      image: "./assets/images/tiger.png";
    }
  | {
      name: "Perro";
      sound: "./assets/sounds/dog.mp3";
      image: "./assets/images/dog.png";
    }
  | {
      name: "Delfín";
      sound: "./assets/sounds/dolphin.mp3";
      image: "./assets/images/dolphin.png";
    };
