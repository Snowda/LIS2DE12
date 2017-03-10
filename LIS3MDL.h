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

#ifndef _LIS3MDL_H_
#define _LIS3MDL_H_

//Registers
#define LIS3MDL_WHO_AM_I 		0x0F
#define LIS3MDL_CTRL_REG1 		0x20
#define LIS3MDL_CTRL_REG2 		0x21
#define LIS3MDL_CTRL_REG3 		0x22
#define LIS3MDL_CTRL_REG4 		0x23
#define LIS3MDL_CTRL_REG5 		0x24
#define LIS3MDL_STATUS_REG 		0x27
#define LIS3MDL_OUT_X_L			0x28
#define LIS3MDL_OUT_X_H			0x29
#define LIS3MDL_OUT_Y_L			0x2A
#define LIS3MDL_OUT_Y_H			0x2B
#define LIS3MDL_OUT_Z_L			0x2C
#define LIS3MDL_OUT_Z_H			0x2D
#define LIS3MDL_TEMP_OUT_L 		0x2E
#define LIS3MDL_TEMP_OUT_H 		0x2F
#define LIS3MDL_INT_CFG			0x30
#define LIS3MDL_INT_SRC			0x31
#define LIS3MDL_INT_THS_L 		0x32
#define LIS3MDL_INT_THS_H 		0x33

#define LIS3MDL_ADDRESS 		0x1C

//Register Masks

//WHO_AM_I masks
#define LIS3MDL_I_AM_MASK 		0x3D

// CTRL_REG1 masks
#define LIS3MDL_TEMP_EN_MASK	0xF0
#define LIS3MDL_OM_MASK 		0x06
#define LIS3MDL_DO_MASK			0x1C
#define LIS3MDL_ST_MASK			0x01

// CTRL_REG2 masks
#define LIS3MDL_FS_MASK 		0x60
#define LIS3MDL_REBOOT_MASK 	0x08
#define LIS3MDL_SOFT_RST_MASK 	0x04

// CTRL_REG3 masks
#define LIS3MDL_LP_MASK			0x20
#define LIS3MDL_SIM_MASK 		0x04
#define LIS3MDL_MD_MASK 		0x03

// CTRL_REG4 masks
#define LIS3MDL_OMZ_MASK 		0x0C
#define LIS3MDL_BLE_MASK 		0x02

// CTRL_REG5 masks
#define LIS3MDL_BDU_MASK 		0x40

// STATUS_REG masks
#define LIS3MDL_ZYXOR_MASK		0x80
#define LIS3MDL_ZOR_MASK 		0x40
#define LIS3MDL_YOR_MASK 		0x20
#define LIS3MDL_XOR_MASK 		0x10
#define LIS3MDL_ZYXDA_MASK 		0x08
#define LIS3MDL_ZDA_MASK 		0x04
#define LIS3MDL_YDA_MASK 		0x02
#define LIS3MDL_XDA_MASK 		0x01

// INT_CFG masks
#define LIS3MDL_XIEN_MASK 		0x80
#define LIS3MDL_YIEN_MASK 		0x40
#define LIS3MDL_ZIEN_MASK 		0x20
#define LIS3MDL_IEA_MASK 		0x04
#define LIS3MDL_LIR_MASK 		0x02
#define LIS3MDL_IEN_MASK 		0x01

// INT_SRC masks
#define LIS3MDL_PTH_X_MASK 		0x80
#define LIS3MDL_PTH_Y_MASK 		0x40
#define LIS3MDL_PTH_Z_MASK 		0x20
#define LIS3MDL_NTH_X_MASK 		0x01
#define LIS3MDL_NTH_Y_MASK 		0x08
#define LIS3MDL_NTH_Z_MASK 		0x04
#define LIS3MDL_MROI_MASK 		0x02
#define LIS3MDL_INT_MASK 		0x01

// INT_THS_L masks
#define LIS3MDL_THS_L_MASK 		0xFF

// INT_THS_H masks
#define LIS3MDL_THS_H_MASK 		0x7F

//#define LIS3MDL_DEFAULT_ADDRESS 	0x18

class LIS3MDL {
    public:
        LIS3MDL();
       	bool init(void);

		int16_t getAxisX(void);
		int16_t getAxisY(void);
		int16_t getAxisZ(void);
		int16_t getTemperature(void);
		void getMag(int16_t* ax, int16_t* ay, int16_t* az);

		bool whoAmI(void);

		bool getTempEnabled(void);
		bool setTempEnabled(bool enable);
		uint8_t getOperativeMode(void);
		bool setOperativeMode(uint8_t mode);
		uint8_t getDataRate(void);
		bool setDataRate(uint8_t rate);
		bool getSelfTestEnabled(void);
		bool setSelfTestEnabled(bool enable);

		uint8_t getFullScale(void);
		bool setFullScale(uint8_t sccale);
		bool reboot(void);
		bool softReset(void);

		bool getLowPowerMode(void);
		bool setLowPowerMode(bool mode);
		bool getSPIMode(void);
		bool setSPIMode(bool mode);
		uint8_t getOperatingMode(void);
		bool setOperatingMode(uint8_t mode);

		uint8_t getOMZ(void);
		bool setOMZ(uint8_t mode);
		bool getEndian(void);
		bool setEndian(bool selection);

		bool disableDataUpdate(void);
		bool enableDataUpdate(void);
		bool getDataUpdateStatus(void);

		bool getXYZOverrun(void);
		bool getZOverrun(void);
		bool getYOverrun(void);
		bool getXOverrun(void);

		bool getXYZDataAvailable(void);
		bool getZDataAvailable(void);
		bool getYDataAvailabl(void);
		bool getXDataAvailable(void);

		bool enableXInterupt(void);
		bool disableXInterupt(void);
		bool isXIntEnabled(void);
		bool enableYInterupt(void);
		bool disableYInterupt(void);
		bool isYIntEnabled(void);
		bool enableZInterupt(void);
		bool disableZInterupt(void);
		bool isZIntEnabled(void);
		bool setIntHigh(void);
		bool setIntLow(void);
		bool checkIntConfig(void);
		bool setInteruptLatch(bool latch);
		bool checkInteruptLatch(void);
		bool enableIntInterupt(void);
		bool disableIntInterupt(void);
		bool checkIntInterupt(void);

		bool posXThreshold(void);
		bool posYThreshold(void);
		bool posZThreshold(void);
		bool negXThreshold(void);
		bool negYThreshold(void);
		bool negZThreshold(void);
		bool measurementOverflow(void);
		bool checkInteruptEvent(void);

		bool setInteruptThreshold(uint16_t threshold);
		uint16_t getInteruptThreshold(void);

    private:
        bool writeRegister(const uint8_t register_addr, const uint8_t value);
        bool writeRegisters(const uint8_t msb_register, const uint8_t msb_value, const uint8_t lsb_register, const uint8_t lsb_value);
        bool writeMaskedRegister(const uint8_t register_addr, const uint8_t mask, const bool value);
		bool writeMaskedRegister(const int register_addr, const int mask, const int value);
		uint8_t readRegister(const uint8_t register_addr);
		uint16_t readRegisters(const uint8_t msb_register, const uint8_t lsb_register);
		uint8_t readMaskedRegister(const uint8_t register_addr, const uint8_t mask);

        uint8_t _address;
        uint8_t _whoami;
};

#endif /* _LIS3MDL_H_ */