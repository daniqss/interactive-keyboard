import machine
import sys
import time
from typing import TypedDict, List

class Button(TypedDict):
    pin: int
    note: str
    pin_obj: machine.Pin

buttons: List[Button] = [
    {"pin": 13, "note": "do"},
    {"pin": 12, "note": "re"},
    {"pin": 14, "note": "mi"},
    {"pin": 27, "note": "fa"},
    {"pin": 26, "note": "sol"},
    {"pin": 25, "note": "la"},
    {"pin": 33, "note": "si"},
    {"pin": 32, "note": "do_alto"}
]

for button in buttons:
    button["pin_obj"] = machine.Pin(button["pin"], machine.Pin.IN, machine.Pin.PULL_UP)

previous_states: List[int] = [1] * len(buttons)

while True:
    for i, button in enumerate(buttons):
        pin_obj: machine.Pin = button["pin_obj"]
        current_state: int = pin_obj.value()

        if current_state == 0 and previous_states[i] == 1:
            note: str = button["note"]
            sys.stdout.write(note + "\n")
            sys.stdout.flush()

        previous_states[i] = current_state

    time.sleep(0.01)
