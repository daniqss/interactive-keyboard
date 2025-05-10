from machine import Pin # type: ignore
import time

botons = [
    Pin(26, Pin.IN, Pin.PULL_UP),
    Pin(25, Pin.IN, Pin.PULL_UP),
    Pin(33, Pin.IN, Pin.PULL_UP),
]

notes = [
    "re",
    "sol",
    "si",
]

led = Pin(2, Pin.OUT)

def main():
    while True:
        for boton, note in zip(botons, notes):
            if boton.value() == 0:
                led.on()

                # send note through serial
                print(note)
                time.sleep_ms(200)

                while boton.value() == 0:
                    pass

                led.off()

        time.sleep_ms(50)

if __name__ == '__main__':
    main()
