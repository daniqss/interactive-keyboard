# how to setup
```bash
sudo usermod -a -G uucp $USER
wget https://micropython.org/resources/firmware/ESP32_GENERIC-20241129-v1.24.1.bin
python -m venv venv
source venv/bin/activate
pip install -m requirements.txt

esptool.py --port /dev/ttyUSB0  erase_flash
esptool.py --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 ESP32_GENERIC-20241129-v1.24.1.bin 

mpremote cp main.py :/main.py
mpremote reset

# to check serial port for debugging
minicom -b 115200 -o -D /dev/ttyUSB0
```
