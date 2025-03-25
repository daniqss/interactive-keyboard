# how to setup
```bash
sudo usermod -a -G uucp $USER
wget https://micropython.org/resources/firmware/ESP32_GENERIC-20241129-v1.24.1.bin
python -m venv venv
pip install -m requirements.txt

esptool.py --port /dev/ttyUSB0  erase_flash
esptool.py --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 ESP32_GENERIC-20241129-v1.24.1.bin 
```
