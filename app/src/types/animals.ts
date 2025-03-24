export type Animal =
  | {
      name: "Elefante";
      sound: "elephant_sound.mp3";
      image: "./assets/images/elephant.png";
    }
  | {
      name: "Tigre";
      sound: "tiger_sound.wav";
      image: "./assets/images/tiger.png";
    }
  | {
      name: "Perro";
      sound: "dog_sound.wav";
      image: "./assets/images/dog.png";
    }
  | {
      name: "Delfín";
      sound: "dolphin_sound.wav";
      image: "./assets/images/dolphin.png";
    };
