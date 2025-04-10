from machine import Pin # type: ignore
import time

botons = [
    Pin(26, Pin.IN, Pin.PULL_UP),
    Pin(25, Pin.IN, Pin.PULL_UP),
    Pin(33, Pin.IN, Pin.PULL_UP),
    Pin(32, Pin.IN, Pin.PULL_UP),
    Pin(35, Pin.IN, Pin.PULL_UP),
    Pin(34, Pin.IN, Pin.PULL_UP),
    Pin(39, Pin.IN, Pin.PULL_UP),
    Pin(36, Pin.IN, Pin.PULL_UP),
]

notes = [
    "sol",
    "si",
    "re",
    "fa",
    "do",
    "la",
    "mi",
    "do-sharp",
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
