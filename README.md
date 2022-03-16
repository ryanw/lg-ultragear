# LG UltraGear

A rust library to control the RGB LEDs on the back of the LG UltraGear
38GN950-B monitor. It may work on similar models, but I haven't tested it on
anything else.

Only tested on Linux, but should work on anything that supports libusb.

# Running

On Linux you will either need to be root, or add this udev rule to grant every
user write access to all LG USB devices

    SUBSYSTEM=="usb", ATTRS{idVendor}=="043e", MODE="0666"

# Videos

https://user-images.githubusercontent.com/3372/158331090-647ae2c9-2942-43ea-a55e-4f9f718ec1f5.mp4

https://user-images.githubusercontent.com/3372/158331125-1716c80c-e9e9-4bfc-9041-a977a5ced8ed.mp4
