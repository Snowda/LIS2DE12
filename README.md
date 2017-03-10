LIS3MDL
======================

Arduino device driver for communicating with LIS3MDL magnetometer over I2C

![LIS3MDL datasheet (PDF)](http://www.st.com/web/en/resource/technical/document/datasheet/DM00075867.pdf)

======================
Installation
======================

Requires the Arduino Libraries
![Wire.h](http://arduino.cc/en/reference/wire) 
for I2C and 
![SPI.h](http://arduino.cc/en/Reference/SPI) 
depending on how you wish to interface with the chip.

*warning* SPI not supported yet!

Download the repo as a zip file and install throught the Arduino IDE and select:

Sketch -> Import Library -> Add Library

Make sure that if you are installing updates that you remove any pre-existing libraries called "LIS3MDL".

======================
Wiring Diagram
======================

![LIS3MDL wiring diagram](docs/LIS3MDL.png)

I2C lines: SDA and SCL