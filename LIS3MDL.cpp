/** Based on ST MicroElectronics LIS3MDL datasheet http://www.st.com/web/en/resource/technical/document/datasheet/DM00075867.pdf
* 12/11/2014 by Conor Forde <me@conorforde.com>
* Updates should be available at https://github.com/Snowda/LIS3MDL
*
* Changelog:
*     ... - ongoing development release

* NOTE: THIS IS ONLY A PARIAL RELEASE. 
* THIS DEVICE CLASS IS CURRENTLY UNDERGOING ACTIVE DEVELOPMENT AND IS MISSING MOST FEATURES. 
* PLEASE KEEP THIS IN MIND IF YOU DECIDE TO USE THIS PARTICULAR CODE FOR ANYTHING.
*/

#include "stdint.h"
#include "LIS3MDL.h"
#include "Wire.h"

LIS3MDL::LIS3MDL() {
    _address = LIS3MDL_I_AM_MASK;//LIS3MDL_DEFAULT_ADDRESS;
}

bool LIS3MDL::init(void) {
    Wire.begin(); 
    
    // Choose device mode (bits 1:0 = 00 = continuous data read, 01 = single conversion, 10 & 11 = default power down)
    writeByte(LIS3MDL_ADDRESS, LIS3MDL_CTRL_REG3, 0x00); // Enable continuous data read mode (bits 1:0 = 00)
    // Enable temperature sensor (bit 7 = 1)
    // Set magnetometer operative mode for x and y axes (bits 6:5)
    // Set magnetometer ODR (bits 4:2)
    writeByte(LIS3MDL_ADDRESS, LIS3MDL_CTRL_REG1, 0x80 | Mopmode << 5 | Modr << 2);
    writeByte(LIS3MDL_ADDRESS, LIS3MDL_CTRL_REG2, Mscale << 5); // Set magnetometer full scale range
    writeByte(LIS3MDL_ADDRESS, LIS3MDL_CTRL_REG4, Mopmode << 2); // Set magnetometer operative mode for z axis
    writeByte(LIS3MDL_ADDRESS, LIS3MDL_CTRL_REG5, 0x40); // output registers not updated until both data bytes have been read


    //Put into the correct operating mode 
    disableLowPower();
    enableAxisXYZ();
    setDataRate(2);
}

bool LIS3MDL::writeRegister(const uint8_t register_addr, const uint8_t value) {
    //send write call to sensor address
    //send register address to sensor
    //send value to register
    bool write_status = 0;
    Wire.beginTransmission(_address); //open communication with 
    Wire.write(register_addr);  
    Wire.write(value); 
    Wire.endTransmission(); 

    return write_status; //returns whether the write succeeded or failed
}

bool LIS3MDL::writeRegisters(const uint8_t msb_register, const uint8_t msb_value, const uint8_t lsb_register, const uint8_t lsb_value) { 
    //send write call to sensor address
    //send register address to sensor
    //send value to register
    bool msb_bool, lsb_bool;
    msb_bool = writeRegister(msb_register, msb_value);
    lsb_bool = writeRegister(lsb_register, lsb_value);
    return msb_bool | lsb_bool; //returns whether the write succeeded or failed
}

bool LIS3MDL::writeMaskedRegister(const uint8_t register_addr, const uint8_t mask, const bool value) {
    uint8_t data = readRegister(register_addr);
    uint8_t combo;
    if(value) {
        combo = (mask | data);
    } else {
        combo = ((~mask) & data);
    }
    return writeRegister(register_addr, combo);
}

bool LIS3MDL::writeMaskedRegister(const int register_addr, const int mask, const int value) {
    uint8_t data = readRegister(register_addr);
    uint8_t masked_value = (data | (mask & value)); //Not sure if right...
    return writeRegister(register_addr, masked_value);
}

uint16_t LIS3MDL::readRegisters(const uint8_t msb_register, const uint8_t lsb_register) {
    uint8_t msb = readRegister(msb_register);
    uint8_t lsb = readRegister(lsb_register);
    return (((int16_t)msb) << 8) | lsb;
}

uint8_t LIS3MDL::readMaskedRegister(const uint8_t register_addr, const uint8_t mask) {
    uint8_t data = readRegister(register_addr);
    return (data & mask);
}


/** Read the X axis registers
 * @see LIS3MDL_OUT_X_H
 * @see LIS3MDL_OUT_X_L
 */
int16_t LIS3MDL::getAxisX(void) {
    return readRegisters(LIS3MDL_OUT_X_H, LIS3MDL_OUT_X_L);
}


/** Read the Y axis registers
 * @see LIS3MDL_OUT_Y_H
 * @see LIS3MDL_OUT_Y_L
 */
int16_t LIS3MDL::getAxisY(void) {
    return readRegisters(LIS3MDL_OUT_Y_H, LIS3MDL_OUT_Y_L);
}

/** Read the Z axis registers
 * @see LIS3MDL_OUT_Z_H
 * @see LIS3MDL_OUT_Z_L
 */
int16_t LIS3MDL::getAxisZ(void) {
    return readRegisters(LIS3MDL_OUT_Z_H, LIS3MDL_OUT_Z_L);
}

int16_t getTemperature(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

void getMag(int16_t* ax, int16_t* ay, int16_t* az) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool whoAmI(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getTempEnabled(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setTempEnabled(bool enable) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint8_t getOperativeMode(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setOperativeMode(uint8_t mode) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint8_t getDataRate(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setDataRate(uint8_t rate) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getSelfTestEnabled(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setSelfTestEnabled(bool enable) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint8_t getFullScale(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setFullScale(uint8_t sccale) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool reboot(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool softReset(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getLowPowerMode(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setLowPowerMode(bool mode) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getSPIMode(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setSPIMode(bool mode) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint8_t getOperatingMode(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setOperatingMode(uint8_t mode) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint8_t getOMZ(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setOMZ(uint8_t mode) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getEndian(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setEndian(bool selection) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool disableDataUpdate(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool enableDataUpdate(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getDataUpdateStatus(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getXYZOverrun(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getZOverrun(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getYOverrun(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getXOverrun(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getXYZDataAvailable(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getZDataAvailable(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getYDataAvailabl(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool getXDataAvailable(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool enableXInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool disableXInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool isXIntEnabled(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool enableYInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool disableYInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool isYIntEnabled(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool enableZInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool disableZInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool isZIntEnabled(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setIntHigh(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setIntLow(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool checkIntConfig(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool setInteruptLatch(bool latch) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool checkInteruptLatch(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool enableIntInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool disableIntInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool checkIntInterupt(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}


bool posXThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool posYThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool posZThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool negXThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool negYThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool negZThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool measurementOverflow(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

bool checkInteruptEvent(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}


bool setInteruptThreshold(uint16_t threshold) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}

uint16_t getInteruptThreshold(void) {
    //return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}





/** Read the all axis registers
 * @see getAxisZ()
 * @see getAxisY()
 * @see getAxisZ()
 */
void LIS3MDL::getMotion(int16_t* ax, int16_t* ay, int16_t* az) {
    *ax = getAxisX();
    *ay = getAxisY();
    *az = getAxisZ();
}

bool LIS3MDL::tempHasOverrun(void) {
    uint8_t overrun = readMaskedRegister(LIS3MDL_STATUS_REG_AUX, LIS3MDL_TOR_MASK);
    return (overrun != 0);
}

bool LIS3MDL::tempDataAvailable(void) {
    uint8_t data = readMaskedRegister(LIS3MDL_STATUS_REG_AUX, LIS3MDL_TDA_MASK);
    return (data != 0);
}

uint16_t LIS3MDL::getTemperature(void) {
    if(tempDataAvailable()){
        return readRegisters(LIS3MDL_OUT_TEMP_H, LIS3MDL_OUT_TEMP_L);
    } else {
        //if new data isn't available
        return 0;
    }
}

bool LIS3MDL::whoAmI(void) {
    return (LIS3MDL_I_AM_MASK == readRegister(LIS3MDL_WHO_AM_I));
}

bool LIS3MDL::getTempEnabled(void) {
    return (readMaskedRegister(LIS3MDL_TEMP_CFG_REG, LIS3MDL_TEMP_EN_MASK) != 0);
}

bool LIS3MDL::setTempEnabled(bool enable) {
    return writeRegister(LIS3MDL_TEMP_CFG_REG, enable ? LIS3MDL_TEMP_EN_MASK : 0);
}

uint8_t LIS3MDL::getDataRate(void) {
    return readMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_ODR_MASK);
}

bool LIS3MDL::setDataRate(uint8_t rate) {
    if(rate > 9) {
        return 0;
    }
    uint8_t data_rate = rate << 4;
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_ODR_MASK, data_rate);
}


bool LIS3MDL::enableLowPower(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_LPEN_MASK, true);
}


bool LIS3MDL::disableLowPower(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_LPEN_MASK, false);
}


bool LIS3MDL::isLowPowerEnabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_LPEN_MASK) != 0);
}

bool LIS3MDL::enableAxisX(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_X_EN_MASK, true);
}

bool LIS3MDL::disableAxisX(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_X_EN_MASK, false);
}

bool LIS3MDL::isXAxisEnabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_X_EN_MASK) != 0);
}

bool LIS3MDL::enableAxisY(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Y_EN_MASK, true);
}

bool LIS3MDL::disableAxisY(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Y_EN_MASK, false);
}

bool LIS3MDL::isYAxisEnabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Y_EN_MASK) != 0);
}

bool LIS3MDL::enableAxisZ(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Z_EN_MASK, true);
}

bool LIS3MDL::disableAxisZ(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Z_EN_MASK, false);
}

bool LIS3MDL::isZAxisEnabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_Z_EN_MASK) != 0);
}

bool LIS3MDL::enableAxisXYZ(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_XYZ_EN_MASK, true);
}

bool LIS3MDL::disableAxisXYZ(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG1, LIS3MDL_XYZ_EN_MASK, false);
}

bool LIS3MDL::getHPFilterMode(uint8_t mode) {
    return readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPM_MASK);
}

bool LIS3MDL::setHPFilterMode(uint8_t mode) {
    if(mode > 3) {
        return 0;
    }
    uint8_t filter_mode = mode << 6;
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPM_MASK, filter_mode);
}

//FDS functions

bool LIS3MDL::EnableHPClick(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPCLICK_MASK, true);
}

bool LIS3MDL::disableHPClick(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPCLICK_MASK, false);
}

bool LIS3MDL::isHPClickEnabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPCLICK_MASK) != 0);
}

bool LIS3MDL::EnableHPIS1(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS1_MASK, true);
}

bool LIS3MDL::disableHPIS1(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS1_MASK, false);
}

bool LIS3MDL::isHPIS1Enabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS1_MASK) != 0);
}

bool LIS3MDL::EnableHPIS2(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK, true);
}

bool LIS3MDL::disableHPIS2(void) {
    return writeMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK, false);
}

bool LIS3MDL::isHPIS2Enabled(void) {
    return (readMaskedRegister(LIS3MDL_CTRL_REG2, LIS3MDL_HPIS2_MASK) != 0);
}