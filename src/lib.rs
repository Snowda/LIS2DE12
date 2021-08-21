#![no_std]

extern crate embedded_hal;

use embedded_hal::blocking::i2c;
use embedded_hal::digital::v2::{OutputPin, InputPin};
use embedded_hal::blocking::delay::DelayMs;
//use embedded_hal::spi::{Mode};

const PROPERTY_ENABLE: u8 = 1
const LIS2DE12_STATUS_REG_AUX: u8 = 0x07;

typedef struct{
	u8 not_used_01       : 2;
	u8 tda               : 1;
	u8 not_used_02       : 3;
	u8 tor               : 1;
	u8 not_used_03       : 1;
} lis2de12_status_reg_aux_t;

const LIS2DE12_OUT_TEMP_L: u8 = 0x0C;
const LIS2DE12_OUT_TEMP_H: u8 = 0x0D;
const LIS2DE12_WHO_AM_I: u8 = 0x0F;
const LIS2DE12_CTRL_REG0: u8 = 0x1E;

typedef struct{
	u8 not_used_01       : 7;
	u8 sdo_pu_disc       : 1;
} lis2de12_ctrl_reg0_t;

const LIS2DE12_TEMP_CFG_REG: u8 = 0x1F;

typedef struct {
	u8 not_used_01       : 6;
	u8 temp_en           : 2;

} lis2de12_temp_cfg_reg_t;

const LIS2DE12_CTRL_REG1: u8 = 0x20;

typedef struct {
	u8 xen               : 1;
	u8 yen               : 1;
	u8 zen               : 1;
	u8 lpen              : 1;
	u8 odr               : 4;
} ctrl_reg1;

const LIS2DE12_CTRL_REG2: u8 = 0x21;
typedef struct {
	u8 hp                : 3; /* HPCLICK + HP_IA2 + HP_IA1 -> HP */
	u8 fds               : 1;
	u8 hpcf              : 2;
	u8 hpm               : 2;
} lis2de12_ctrl_reg2_t;

const LIS2DE12_CTRL_REG3: u8 = 0x22;

typedef struct {
	u8 not_used_01       : 1;
	u8 i1_overrun        : 1;
	u8 i1_wtm            : 1;
	u8 not_used_02       : 1;
	u8 i1_zyxda          : 1;
	u8 i1_ia2            : 1;
	u8 i1_ia1            : 1;
	u8 i1_click          : 1;
} lis2de12_ctrl_reg3_t;

const LIS2DE12_CTRL_REG4: u8 = 0x23;

typedef struct {
	u8 sim               : 1;
	u8 st                : 2;
	u8 not_used_01       : 1;
	u8 fs                : 2;
	u8 not_used_02       : 1;
	u8 bdu               : 1;
} lis2de12_ctrl_reg4_t;

const LIS2DE12_CTRL_REG5: u8 = 0x24;

typedef struct {
	u8 d4d_int2          : 1;
	u8 lir_int2          : 1;
	u8 d4d_int1          : 1;
	u8 lir_int1          : 1;
	u8 not_used_01       : 2;
	u8 fifo_en           : 1;
	u8 boot              : 1;
} lis2de12_ctrl_reg5_t;

const LIS2DE12_CTRL_REG6: u8 = 0x25;

typedef struct {
	u8 not_used_01       : 1;
	u8 int_polarity      : 1;
	u8 not_used_02       : 1;
	u8 i2_act            : 1;
	u8 i2_boot           : 1;
	u8 i2_ia2            : 1;
	u8 i2_ia1            : 1;
	u8 i2_click          : 1;
} lis2de12_ctrl_reg6_t;

const LIS2DE12_REFERENCE: u8 = 0x26;
const LIS2DE12_STATUS_REG: u8 = 0x27;

typedef struct {
	u8 xda               : 1;
	u8 yda               : 1;
	u8 zda               : 1;
	u8 zyxda             : 1;
	u8 _xor              : 1;
	u8 yor               : 1;
	u8 zor               : 1;
	u8 zyxor             : 1;
} lis2de12_status_reg_t;

const LIS2DE12_FIFO_READ_START: u8 = 0x28;
const LIS2DE12_OUT_X_H: u8 = 0x29;
const LIS2DE12_OUT_Y_H: u8 = 0x2B;
const LIS2DE12_OUT_Z_H: u8 = 0x2D;
const LIS2DE12_FIFO_CTRL_REG: u8 = 0x2E;

typedef struct {
	u8 fth               : 5;
	u8 tr                : 1;
	u8 fm                : 2;
} lis2de12_fifo_ctrl_reg_t;

const LIS2DE12_FIFO_SRC_REG: u8 = 0x2F;

typedef struct {
	u8 fss               : 5;
	u8 empty             : 1;
	u8 ovrn_fifo         : 1;
	u8 wtm               : 1;
} lis2de12_fifo_src_reg_t;

const LIS2DE12_INT1_CFG: u8 = 0x30;

typedef struct {
	u8 xlie              : 1;
	u8 xhie              : 1;
	u8 ylie              : 1;
	u8 yhie              : 1;
	u8 zlie              : 1;
	u8 zhie              : 1;
	u8 _6d               : 1;
	u8 aoi               : 1;
} lis2de12_int1_cfg_t;

const LIS2DE12_INT1_SRC: u8 = 0x31;

typedef struct {
	u8 xl                : 1;
	u8 xh                : 1;
	u8 yl                : 1;
	u8 yh                : 1;
	u8 zl                : 1;
	u8 zh                : 1;
	u8 ia                : 1;
	u8 not_used_01       : 1;
} lis2de12_int1_src_t;

const LIS2DE12_INT1_THS: u8 = 0x32;

typedef struct {
	u8 ths               : 7;
	u8 not_used_01       : 1;
} lis2de12_int1_ths_t;

const LIS2DE12_INT1_DURATION: u8 = 0x33;

typedef struct {
	u8 d                 : 7;
	u8 not_used_01       : 1;
} lis2de12_int1_duration_t;

const LIS2DE12_INT2_CFG: u8 = 0x34;

typedef struct {
	u8 xlie              : 1;
	u8 xhie              : 1;
	u8 ylie              : 1;
	u8 yhie              : 1;
	u8 zlie              : 1;
	u8 zhie              : 1;
	u8 _6d               : 1;
	u8 aoi               : 1;
} lis2de12_int2_cfg_t;

const LIS2DE12_INT2_SRC: u8 = 0x35;

typedef struct {
	u8 xl                : 1;
	u8 xh                : 1;
	u8 yl                : 1;
	u8 yh                : 1;
	u8 zl                : 1;
	u8 zh                : 1;
	u8 ia                : 1;
	u8 not_used_01       : 1;
} lis2de12_int2_src_t;

const LIS2DE12_INT2_THS: u8 = 0x36;

typedef struct {
	u8 ths               : 7;
	u8 not_used_01       : 1;
} lis2de12_int2_ths_t;

const LIS2DE12_INT2_DURATION: u8 = 0x37;

typedef struct {
	u8 d                 : 7;
	u8 not_used_01       : 1;
} lis2de12_int2_duration_t;

const LIS2DE12_CLICK_CFG: u8 = 0x38;

typedef struct {
	u8 xs                : 1;
	u8 xd                : 1;
	u8 ys                : 1;
	u8 yd                : 1;
	u8 zs                : 1;
	u8 zd                : 1;
	u8 not_used_01       : 2;
} lis2de12_click_cfg_t;

const LIS2DE12_CLICK_SRC: u8 = 0x39;

typedef struct {
	u8 x                 : 1;
	u8 y                 : 1;
	u8 z                 : 1;
	u8 sign              : 1;
	u8 sclick            : 1;
	u8 dclick            : 1;
	u8 ia                : 1;
	u8 not_used_01       : 1;
} lis2de12_click_src_t;

const LIS2DE12_CLICK_THS: u8 = 0x3A;

typedef struct {
	u8 ths               : 7;
	u8 lir_click         : 1;
} lis2de12_click_ths_t;

const LIS2DE12_TIME_LIMIT: u8 = 0x3B;

typedef struct {
	u8 tli               : 7;
	u8 not_used_01       : 1;
} lis2de12_time_limit_t;

const LIS2DE12_TIME_LATENCY: u8 = 0x3C;

typedef struct {
	u8 tla               : 8;
} lis2de12_time_latency_t;

const LIS2DE12_TIME_WINDOW: u8 = 0x3D;

typedef struct {
	u8 tw                : 8;
} lis2de12_time_window_t;

const LIS2DE12_ACT_THS: u8 = 0x3E;

typedef struct {
	u8 acth              : 7;
	u8 not_used_01       : 1;
} lis2de12_act_ths_t;

const LIS2DE12_ACT_DUR: u8 = 0x3F;

typedef struct {
	u8 actd              : 8;
} lis2de12_act_dur_t;

typedef enum {
	LIS2DE12_TEMP_DISABLE  = 0,
	LIS2DE12_TEMP_ENABLE   = 3,
} lis2de12_temp_en_t;

typedef enum {
	LIS2DE12_POWER_DOWN                      = 0x00,
	LIS2DE12_ODR_1Hz                         = 0x01,
	LIS2DE12_ODR_10Hz                        = 0x02,
	LIS2DE12_ODR_25Hz                        = 0x03,
	LIS2DE12_ODR_50Hz                        = 0x04,
	LIS2DE12_ODR_100Hz                       = 0x05,
	LIS2DE12_ODR_200Hz                       = 0x06,
	LIS2DE12_ODR_400Hz                       = 0x07,
	LIS2DE12_ODR_1kHz620_LP                  = 0x08,
	LIS2DE12_ODR_5kHz376_LP_1kHz344_NM_HP    = 0x09,
} lis2de12_odr_t;

typedef enum {
	LIS2DE12_AGGRESSIVE  = 0,
	LIS2DE12_STRONG      = 1,
	LIS2DE12_MEDIUM      = 2,
	LIS2DE12_LIGHT       = 3,
} lis2de12_hpcf_t;

typedef enum {
	LIS2DE12_NORMAL_WITH_RST  = 0,
	LIS2DE12_REFERENCE_MODE   = 1,
	LIS2DE12_NORMAL           = 2,
	LIS2DE12_AUTORST_ON_INT   = 3,
} lis2de12_hpm_t;

typedef enum {
	LIS2DE12_2g   = 0,
	LIS2DE12_4g   = 1,
	LIS2DE12_8g   = 2,
	LIS2DE12_16g  = 3,
} lis2de12_fs_t;

typedef enum {
	LIS2DE12_ST_DISABLE   = 0,
	LIS2DE12_ST_POSITIVE  = 1,
	LIS2DE12_ST_NEGATIVE  = 2,
} lis2de12_st_t;

typedef enum {
	LIS2DE12_DISC_FROM_INT_GENERATOR  = 0,
	LIS2DE12_ON_INT1_GEN              = 1,
	LIS2DE12_ON_INT2_GEN              = 2,
	LIS2DE12_ON_TAP_GEN               = 4,
	LIS2DE12_ON_INT1_INT2_GEN         = 3,
	LIS2DE12_ON_INT1_TAP_GEN          = 5,
	LIS2DE12_ON_INT2_TAP_GEN          = 6,
	LIS2DE12_ON_INT1_INT2_TAP_GEN     = 7,
} lis2de12_hp_t;

typedef enum {
	LIS2DE12_INT2_PULSED   = 0,
	LIS2DE12_INT2_LATCHED  = 1,
} lis2de12_lir_int2_t;

typedef enum {
	LIS2DE12_INT1_PULSED   = 0,
	LIS2DE12_INT1_LATCHED  = 1,
} lis2de12_lir_int1_t;

typedef enum {
	LIS2DE12_INT1_GEN = 0,
	LIS2DE12_INT2_GEN = 1,
} lis2de12_tr_t;

typedef enum {
	LIS2DE12_BYPASS_MODE           = 0,
	LIS2DE12_FIFO_MODE             = 1,
	LIS2DE12_DYNAMIC_STREAM_MODE   = 2,
	LIS2DE12_STREAM_TO_FIFO_MODE   = 3,
} lis2de12_fm_t;

typedef enum {
	LIS2DE12_TAP_PULSED   = 0,
	LIS2DE12_TAP_LATCHED  = 1,
} lis2de12_lir_click_t;

typedef enum {
	LIS2DE12_PULL_UP_DISCONNECT  = 1,
	LIS2DE12_PULL_UP_CONNECT     = 0,
} lis2de12_sdo_pu_disc_t;

typedef enum {
	LIS2DE12_SPI_4_WIRE = 0,
	LIS2DE12_SPI_3_WIRE = 1,
} lis2de12_sim_t;

pub struct LIS2DE12<I2C> {
	i2c: I2C,
	address: u8,
}

impl LIS2DE12<I2C> {
	/** Read generic device register
	 * @param  reg   register to read
	 * @param  data  pointer to buffer that store the data read(ptr)
	 * @param  len   number of consecutive register to read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn read(&self, register: u8, data: &mut [u8], len: u16) -> bool { // TODO calculate len from data instead
		// TODO return the value instead of pointer to data
		return self.read_reg(address, register, data, len); // TODO replace with HAL
	}

	/** Write generic device register
	 * @param  data  pointer to data to write in register reg(ptr)
	 * @param  len   number of consecutive register to write
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn write(&self, register: u8, data: [u8]) -> i32 {
		return self.i2c.write(reg, &data); // TODO replace with HAL
	}

	// These functions convert raw-data into engineering units.
	fn from_fs2_to_mg(lsb: i16) -> f32{
		return (lsb as f32 / 256.0f) * 15.6f;
	}

	fn from_fs4_to_mg(lsb: i16) -> f32{
		return (lsb as f32 / 256.0f) * 31.2f;
	}

	fn from_fs8_to_mg(lsb: i16) -> f32 {
		return (lsb as f32 / 256.0f) * 62.5f;
	}

	fn from_fs16_to_mg(lsb: i16) -> f32 {
		return (lsb as f32 / 256.0f) * 187.5f;
	}

	fn from_lsb_to_celsius(lsb: i16) -> f32{
		return ((lsb as f32 / 256.0f) * 1.0f) + 25.0f;
	}

	// This section group all the functions concerning data generation.

	/** Temperature status register.[get]
	 * @param  buff     buffer that stores data read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn temp_status_reg_get(&self, buffer: &mut [u8]) -> i32 {
		return self.read(LIS2DE12_STATUS_REG_AUX, buffer, 1);
	}
	/** Temperature data available.[get]
	 * @param  val      change the values of tda in reg STATUS_REG_AUX
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn temp_data_ready_get(&self, u8 *val) -> i32 {
		lis2de12_status_reg_aux_t status_reg_aux;
		let ret: i32 = self.read(LIS2DE12_STATUS_REG_AUX, (u8 *)&status_reg_aux, 1);
		*val = status_reg_aux.tda;
		return ret;
	}
	/** Temperature data overrun.[get]
	 * @param  val      change the values of tor in reg STATUS_REG_AUX
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn temp_data_ovr_get(&self, u8 *val) -> i32 {
		lis2de12_status_reg_aux_t status_reg_aux;
		let ret: i32 = self.read(LIS2DE12_STATUS_REG_AUX, (u8 *)&status_reg_aux, 1);
		*val = status_reg_aux.tor;
		return ret;
	}
	/** Temperature output value.[get]
	 * @param  buff     buffer that stores data read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn temperature_raw_get(&self, i16 *val) -> i32 {
		u8 buff[2]; // TODO delete
		let ret: i32 = self.read(LIS2DE12_OUT_TEMP_L, buff, 2);
		*val = (i16)buff[1];
		*val = (*val * 256) + (i16)buff[0];
		// TODO return an optional value instead
		return ret;
	}

	/** Temperature sensor enable.[set]
	 * @param  val      change the values of temp_en in reg TEMP_CFG_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn temperature_meas_set(&self, lis2de12_temp_en_t val) -> i32 {
		lis2de12_temp_cfg_reg_t temp_cfg_reg;
		let ret: i32 = self.read(LIS2DE12_TEMP_CFG_REG, (u8 *)&temp_cfg_reg, 1);

		if ret == 0 {
			temp_cfg_reg.temp_en = (u8) val;
			ret = self.write(LIS2DE12_TEMP_CFG_REG, (u8 *)&temp_cfg_reg, 1);
		}

		return ret;
	}

	/** Temperature sensor enable.[get]
	 * @param  val      get the values of temp_en in reg TEMP_CFG_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
     */
	fn temperature_meas_get(&self, lis2de12_temp_en_t *val) -> i32 {
		lis2de12_temp_cfg_reg_t temp_cfg_reg;
		let ret: i32 = self.read(LIS2DE12_TEMP_CFG_REG, (u8 *)&temp_cfg_reg, 1);

		*val = match temp_cfg_reg.temp_en {
			LIS2DE12_TEMP_DISABLE => LIS2DE12_TEMP_DISABLE,
			LIS2DE12_TEMP_ENABLE => LIS2DE12_TEMP_ENABLE,
			_ => LIS2DE12_TEMP_DISABLE,
		}

		return ret;
	}

	/** Output data rate selection.[set]
	 * @param  val      change the values of odr in reg CTRL_REG1
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn data_rate_set(&self, lis2de12_odr_t val) -> i32 {
		lis2de12_ctrl_reg1_t ctrl_reg1;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG1, (u8 *)&ctrl_reg1, 1);

		if ret == 0 {
			ctrl_reg1.lpen = PROPERTY_ENABLE;
			ctrl_reg1.odr = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG1, (u8 *)&ctrl_reg1, 1);
		}

		return ret;
	}

	/** Output data rate selection.[get]
	 * @param  val      get the values of odr in reg CTRL_REG1
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn data_rate_get(&self, lis2de12_odr_t *val) -> i32 {
		lis2de12_ctrl_reg1_t ctrl_reg1;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG1, (u8 *)&ctrl_reg1, 1);

		*val = match ctrl_reg1.odr {
			LIS2DE12_POWER_DOWN => LIS2DE12_POWER_DOWN,
			LIS2DE12_ODR_1Hz => LIS2DE12_ODR_1Hz,
			LIS2DE12_ODR_10Hz => LIS2DE12_ODR_10Hz,
			LIS2DE12_ODR_25Hz => LIS2DE12_ODR_25Hz,
			LIS2DE12_ODR_50Hz => LIS2DE12_ODR_50Hz,
			LIS2DE12_ODR_100Hz => LIS2DE12_ODR_100Hz,
			LIS2DE12_ODR_200Hz => LIS2DE12_ODR_200Hz,
			LIS2DE12_ODR_400Hz => LIS2DE12_ODR_400Hz,
			LIS2DE12_ODR_1kHz620_LP => LIS2DE12_ODR_1kHz620_LP,
			LIS2DE12_ODR_5kHz376_LP_1kHz344_NM_HP => LIS2DE12_ODR_5kHz376_LP_1kHz344_NM_HP,
			_ => LIS2DE12_POWER_DOWN,
		}

		return ret;
	}

	/* High pass data from internal filter sent to output register and FIFO.
	 * @param  val      change the values of fds in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_on_outputs_set(&self, val: u8) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		if ret == 0 {
			ctrl_reg2.fds = val;
			ret = self.write(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);
		}

		return ret;
	}

	/** High pass data from internal filter sent to output register and FIFO.[get]
	 * @param  val      change the values of fds in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_on_outputs_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);
		*val = (u8)ctrl_reg2.fds;

		return ret;
	}

	/** High-pass filter cutoff frequency selection.[set]
	 * HPCF[2:1]\ft @1Hz    @10Hz  @25Hz  @50Hz @100Hz @200Hz @400Hz @1kHz6 ft@5kHz
	 * AGGRESSIVE   0.02Hz  0.2Hz  0.5Hz  1Hz   2Hz    4Hz    8Hz    32Hz   100Hz
	 * STRONG       0.008Hz 0.08Hz 0.2Hz  0.5Hz 1Hz    2Hz    4Hz    16Hz   50Hz
	 * MEDIUM       0.004Hz 0.04Hz 0.1Hz  0.2Hz 0.5Hz  1Hz    2Hz    8Hz    25Hz
	 * LIGHT        0.002Hz 0.02Hz 0.05Hz 0.1Hz 0.2Hz  0.5Hz  1Hz    4Hz    12Hz
	 * @param  val      change the values of hpcf in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_bandwidth_set(&self, lis2de12_hpcf_t val) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		if ret == 0 {
			ctrl_reg2.hpcf = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);
		}

		return ret;
	}

	/** High-pass filter cutoff frequency selection.[get]
	 * HPCF[2:1]\ft @1Hz    @10Hz  @25Hz  @50Hz @100Hz @200Hz @400Hz @1kHz6 ft@5kHz
	 * AGGRESSIVE   0.02Hz  0.2Hz  0.5Hz  1Hz   2Hz    4Hz    8Hz    32Hz   100Hz
	 * STRONG       0.008Hz 0.08Hz 0.2Hz  0.5Hz 1Hz    2Hz    4Hz    16Hz   50Hz
	 * MEDIUM       0.004Hz 0.04Hz 0.1Hz  0.2Hz 0.5Hz  1Hz    2Hz    8Hz    25Hz
	 * LIGHT        0.002Hz 0.02Hz 0.05Hz 0.1Hz 0.2Hz  0.5Hz  1Hz    4Hz    12Hz
	 * @param  val      get the values of hpcf in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_bandwidth_get(&self, lis2de12_hpcf_t *val) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		*val = match ctrl_reg2.hpcf {
			LIS2DE12_AGGRESSIVE => LIS2DE12_AGGRESSIVE,
			LIS2DE12_STRONG => LIS2DE12_STRONG,
			LIS2DE12_MEDIUM => LIS2DE12_MEDIUM,
			LIS2DE12_LIGHT => LIS2DE12_LIGHT,
			_ => LIS2DE12_LIGHT,
		}

		return ret;
	}

	/** High-pass filter mode selection.[set]
	 * @param  val      change the values of hpm in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	i32 high_pass_mode_set(&self, lis2de12_hpm_t val) {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		if ret == 0 {
			ctrl_reg2.hpm = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);
		}

		return ret;
	}

	/** High-pass filter mode selection.[get]
	 * @param  val      get the values of hpm in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	i32 high_pass_mode_get(&self, lis2de12_hpm_t *val) {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		*val = match ctrl_reg2.hpm {
			LIS2DE12_NORMAL_WITH_RST => LIS2DE12_NORMAL_WITH_RST,
			LIS2DE12_REFERENCE_MODE => LIS2DE12_REFERENCE_MODE,
			LIS2DE12_NORMAL => LIS2DE12_NORMAL,
			LIS2DE12_AUTORST_ON_INT => LIS2DE12_AUTORST_ON_INT,
			_ => LIS2DE12_NORMAL_WITH_RST,
		}

		return ret;
	}

	/** Full-scale configuration.[set]
	 * @param  val      change the values of fs in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn full_scale_set(&self, lis2de12_fs_t val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		if ret == 0 {
			ctrl_reg4.fs = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);
		}

		return ret;
	}

	/** Full-scale configuration.[get]
	 * @param  val      get the values of fs in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn full_scale_get(&self, lis2de12_fs_t *val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		*val = match ctrl_reg4.fs {
			LIS2DE12_2g => LIS2DE12_2g,
			LIS2DE12_4g => LIS2DE12_4g,
			LIS2DE12_8g => LIS2DE12_8g,
			LIS2DE12_16g => LIS2DE12_16g,
			_ => LIS2DE12_2g,
		}

		return ret;
	}

	/** Block Data Update.[set]
	 * @param  val      change the values of bdu in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn block_data_update_set(&self, u8 val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let mut ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		if ret == 0 {
			ctrl_reg4.bdu = val;
			ret = self.write(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);
		}

		return ret;
	}

	/** Block Data Update.[get]
	 * @param  val      change the values of bdu in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn block_data_update_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);
		*val = (u8)ctrl_reg4.bdu;
		return ret;
	}

	/** Reference value for interrupt generation.[set]
	 *         LSB = ~16@2g / ~31@4g / ~63@8g / ~127@16g
	 * @param  buff     buffer that contains data to write
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn filter_reference_set(&self, u8 *buff) -> i32 {
		return self.write(LIS2DE12_REFERENCE, buff, 1);
	}

	/** Reference value for interrupt generation.[get]
	 *         LSB = ~16@2g / ~31@4g / ~63@8g / ~127@16g
	 * @param  buff     buffer that stores data read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn filter_reference_get(&self, u8 *buff) -> i32 {
		return self.read(LIS2DE12_REFERENCE, buff, 1);
	}

	/** Acceleration set of data available.[get]
	 * @param  val      change the values of zyxda in reg STATUS_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn xl_data_ready_get(&self, u8 *val) -> i32 {
		lis2de12_status_reg_t status_reg;
		let ret: i32 = self.read(LIS2DE12_STATUS_REG, (u8 *)&status_reg, 1);
		*val = status_reg.zyxda;
		return ret;
	}
	/** Acceleration set of data overrun.[get]
	 * @param  val      change the values of zyxor in reg STATUS_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn xl_data_ovr_get(&self, u8 *val) -> i32 {
		lis2de12_status_reg_t status_reg;
		let ret: i32 = self.read(LIS2DE12_STATUS_REG, (u8 *)&status_reg, 1);
		*val = status_reg.zyxor;

		return ret;
	}
	/** Acceleration output value.[get]
	 * @param  buff     buffer that stores data read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn acceleration_raw_get(&self, i16 *val) -> i32 {
		u8 buff[6];
		let ret: i32 = self.read(LIS2DE12_FIFO_READ_START, buff, 6);
		val[0] = (i16)buff[1];
		val[0] = (val[0] * 256) + (i16)buff[0];
		val[1] = (i16)buff[3];
		val[1] = (val[1] * 256) + (i16)buff[2];
		val[2] = (i16)buff[5];
		val[2] = (val[2] * 256) + (i16)buff[4];
		return ret;
	}

	/** DeviceWhoamI .[get]
	 * @param  buff     buffer that stores data read
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn device_id_get(&self, u8 *buff) -> i32 {
		return self.read(LIS2DE12_WHO_AM_I, buff, 1);
	}

	/** Self Test.[set]
	 * @param  val      change the values of st in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn self_test_set(&self, lis2de12_st_t val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let mut ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		if ret == 0 {
			ctrl_reg4.st = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG4,  (u8 *)&ctrl_reg4, 1);
		}

		return ret;
	}

	/** Self Test.[get]
	 * @param  val      Get the values of st in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn self_test_get(&self, lis2de12_st_t *val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		*val = match ctrl_reg4.st {
			LIS2DE12_ST_DISABLE => LIS2DE12_ST_DISABLE,
			LIS2DE12_ST_POSITIVE => LIS2DE12_ST_POSITIVE,
			LIS2DE12_ST_NEGATIVE => LIS2DE12_ST_NEGATIVE,
			_ => LIS2DE12_ST_DISABLE,
		}

		return ret;
	}

	/** Reboot memory content. Reload the calibration parameters.[set]
	 * @param  val      change the values of boot in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn boot_set(&self, u8 val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.boot = val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** Reboot memory content. Reload the calibration parameters.[get]
	 * @param  val      change the values of boot in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn boot_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		*val = (u8)ctrl_reg5.boot;
		return ret;
	}

	/** Info about device status.[get]
	 * @param  val      register STATUS_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn status_get(&self, lis2de12_status_reg_t *val) -> i32 {
		return self.read(LIS2DE12_STATUS_REG, (u8 *) val, 1);
	}

	// This section group all the functions that manage the first interrupts generator

	/** Interrupt generator 1 configuration register.[set]
	 * @param  val      register INT1_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_conf_set(&self, lis2de12_int1_cfg_t *val) -> i32 {
		return self.write(LIS2DE12_INT1_CFG, (u8 *) val, 1);
	}

	/** Interrupt generator 1 configuration register.[get]
	 * @param  val      register INT1_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_conf_get(&self, lis2de12_int1_cfg_t *val) -> i32 {
		return self.read(LIS2DE12_INT1_CFG, (u8 *) val, 1);
	}

	/** Interrupt generator 1 source register.[get]
	 * @param  val      Registers INT1_SRC
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_source_get(&self, lis2de12_int1_src_t *val) -> i32 {
		return self.read(LIS2DE12_INT1_SRC, (u8 *) val, 1);
	}

	/** User-defined threshold value for xl interrupt event on generator 1.[set]
	 *         LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of ths in reg INT1_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_threshold_set(&self, u8 val) -> i32 {
		lis2de12_int1_ths_t int1_ths;
		let ret: i32 = self.read(LIS2DE12_INT1_THS, (u8 *)&int1_ths, 1);

		if ret == 0 {
			int1_ths.ths = val;
			ret = self.write(LIS2DE12_INT1_THS, (u8 *)&int1_ths, 1);
		}

		return ret;
	}

	/** User-defined threshold value for xl interrupt event on generator 1.[get]
	 *         LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of ths in reg INT1_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_threshold_get(&self, u8 *val) -> i32 {
		lis2de12_int1_ths_t int1_ths;
		let ret: i32 = self.read(LIS2DE12_INT1_THS, (u8 *)&int1_ths, 1);
		*val = (u8)int1_ths.ths;
		return ret;
	}

	/** The minimum duration (LSb = 1/ODR) of the Interrupt 1 event to be recognized.[set]
	 * @param  val      change the values of d in reg INT1_DURATION
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
     */
	fn int1_gen_duration_set(&self, u8 val) -> i32 {
		lis2de12_int1_duration_t int1_duration;
		let mut ret: i32 = self.read(LIS2DE12_INT1_DURATION, (u8 *)&int1_duration, 1);

		if ret == 0 {
			int1_duration.d = val;
			ret = self.write(LIS2DE12_INT1_DURATION, (u8 *)&int1_duration, 1);
		}

		return ret;
	}

	/** The minimum duration (LSb = 1/ODR) of the Interrupt 1 event to be recognized.[get]
	 * @param  val      change the values of d in reg INT1_DURATION
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_gen_duration_get(&self, u8 *val) -> i32 {
		lis2de12_int1_duration_t int1_duration;
		let ret: i32 = self.read(LIS2DE12_INT1_DURATION, (u8 *)&int1_duration, 1);
		*val = (u8)int1_duration.d;

		return ret;
	}

	// This section group all the functions that manage the second interrupts generator

	/** Interrupt generator 2 configuration register.[set]
	 * @param  val      registers INT2_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_conf_set(&self, lis2de12_int2_cfg_t *val) -> i32 {
		return self.write(LIS2DE12_INT2_CFG, (u8 *) val, 1);
	}

	/** Interrupt generator 2 configuration register.[get]
	 * @param  val      registers INT2_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_conf_get(&self, lis2de12_int2_cfg_t *val) -> i32{
		return self.read(LIS2DE12_INT2_CFG, (u8 *) val, 1);
	}

	/** Interrupt generator 2 source register.[get]
	 * @param  val      registers INT2_SRC
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_source_get(&self, lis2de12_int2_src_t *val) -> i32 {
		return self.read(LIS2DE12_INT2_SRC, (u8 *) val, 1);
	}

	/** User-defined threshold value for xl interrupt event on generator 2.[set]
	 *          LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of ths in reg INT2_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_threshold_set(&self, u8 val) -> i32 {
		lis2de12_int2_ths_t int2_ths;
		let ret: i32 = self.read(LIS2DE12_INT2_THS, (u8 *)&int2_ths, 1);

		if ret == 0 {
			int2_ths.ths = val;
			ret = self.write(LIS2DE12_INT2_THS, (u8 *)&int2_ths, 1);
		}

		return ret;
	}

	/** User-defined threshold value for xl interrupt event on generator 2.[get]
	 *         LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of ths in reg INT2_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_threshold_get(&self, u8 *val) -> i32 {
		lis2de12_int2_ths_t int2_ths;
		let ret: i32 = self.read(LIS2DE12_INT2_THS, (u8 *)&int2_ths, 1);
		*val = (u8)int2_ths.ths;

		return ret;
	}

	/** The minimum duration (LSb = 1/ODR) of the Interrupt 1 event to be recognized .[set]
	 * @param  val      change the values of d in reg INT2_DURATION
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_duration_set(&self, u8 val) -> i32 {
		lis2de12_int2_duration_t int2_duration;
		let ret: i32 = self.read(LIS2DE12_INT2_DURATION, (u8 *)&int2_duration, 1);

		if ret == 0 {
			int2_duration.d = val;
			ret = self.write(LIS2DE12_INT2_DURATION, (u8 *)&int2_duration, 1);
		}

		return ret;
	}

	/** The minimum duration (LSb = 1/ODR) of the Interrupt 1 event to be recognized.[get]
	 * @param  val      change the values of d in reg INT2_DURATION
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_gen_duration_get(&self, u8 *val) -> i32 {
		lis2de12_int2_duration_t int2_duration;
		let ret: i32 = self.read(LIS2DE12_INT2_DURATION, (u8 *)&int2_duration, 1);
		*val = (u8)int2_duration.d;
		return ret;
	}

	// This section group all the functions that manage interrupt pins

	/** High-pass filter on interrupts/tap generator.[set]
	 * @param  val      change the values of hp in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_int_conf_set(&self, lis2de12_hp_t val) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		if ret == 0 {
			ctrl_reg2.hp = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);
		}

		return ret;
	}

	/** High-pass filter on interrupts/tap generator.[get]
	 * @param  val      Get the values of hp in reg CTRL_REG2
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn high_pass_int_conf_get(&self, lis2de12_hp_t *val) -> i32 {
		lis2de12_ctrl_reg2_t ctrl_reg2;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG2, (u8 *)&ctrl_reg2, 1);

		*val = match ctrl_reg2.hp {
			LIS2DE12_DISC_FROM_INT_GENERATOR => LIS2DE12_DISC_FROM_INT_GENERATOR,
			LIS2DE12_ON_INT1_GEN => LIS2DE12_ON_INT1_GEN,
			LIS2DE12_ON_INT2_GEN => LIS2DE12_ON_INT2_GEN,
			LIS2DE12_ON_TAP_GEN => LIS2DE12_ON_TAP_GEN,
			LIS2DE12_ON_INT1_INT2_GEN => LIS2DE12_ON_INT1_INT2_GEN,
			LIS2DE12_ON_INT1_TAP_GEN => LIS2DE12_ON_INT1_TAP_GEN,
			LIS2DE12_ON_INT2_TAP_GEN => LIS2DE12_ON_INT2_TAP_GEN,
			LIS2DE12_ON_INT1_INT2_TAP_GEN => LIS2DE12_ON_INT1_INT2_TAP_GEN,
			_ => LIS2DE12_DISC_FROM_INT_GENERATOR,
		}

		return ret;
	}

	/** Int1 pin routing configuration register.[set]
	 * @param  val      registers CTRL_REG3
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn pin_int1_config_set(&self, lis2de12_ctrl_reg3_t *val) -> i32 {
		return self.write(LIS2DE12_CTRL_REG3, (u8 *) val, 1);
	}

	/** Int1 pin routing configuration register.[get]
	 * @param  val      registers CTRL_REG3
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn pin_int1_config_get(&self, lis2de12_ctrl_reg3_t *val) -> i32 {
		return self.read(LIS2DE12_CTRL_REG3, (u8 *) val, 1);
	}

	/** int2_pin_detect_4d: [set]  4D enable: 4D detection is enabled
	 *                                    on INT2 pin when 6D bit on
	 *                                    INT2_CFG (34h) is set to 1.
	 * @param  val      change the values of d4d_int2 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_pin_detect_4d_set(&self, u8 val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.d4d_int2 = val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** 4D enable: 4D detection is enabled on INT2 pin when 6D bit on
	 *         INT2_CFG (34h) is set to 1.[get]
	 * @param  val      change the values of d4d_int2 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_pin_detect_4d_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		*val = (u8)ctrl_reg5.d4d_int2;
		return ret;
	}

	/** Latch interrupt request on INT2_SRC (35h) register, with
	 *  INT2_SRC (35h) register cleared by reading INT2_SRC(35h) itself.[set]
	 * @param  val      change the values of lir_int2 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_pin_notification_mode_set(&self, lis2de12_lir_int2_t val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.lir_int2 = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** Latch interrupt request on INT2_SRC (35h) register, with
	 *  INT2_SRC (35h) register cleared by reading INT2_SRC(35h) itself.[get]
	 * @param  val      Get the values of lir_int2 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int2_pin_notification_mode_get(&self, lis2de12_lir_int2_t *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		*val = match ctrl_reg5.lir_int2 {
			LIS2DE12_INT2_PULSED => LIS2DE12_INT2_PULSED,
			LIS2DE12_INT2_LATCHED => LIS2DE12_INT2_LATCHED,
			_ => LIS2DE12_INT2_PULSED,
		}

		return ret;
	}

	/** 4D enable: 4D detection is enabled on INT1 pin when 6D bit
	 *                    on INT1_CFG(30h) is set to 1.[set]
	 * @param  val      change the values of d4d_int1 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_pin_detect_4d_set(&self, u8 val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.d4d_int1 = val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** 4D enable: 4D detection is enabled on INT1 pin when 6D bit on
	 *         INT1_CFG(30h) is set to 1.[get]
	 * @param  val      change the values of d4d_int1 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_pin_detect_4d_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		*val = (u8)ctrl_reg5.d4d_int1;

		return ret;
	}

	/** Latch interrupt request on INT1_SRC (31h), with INT1_SRC(31h)
	 *          register cleared by reading INT1_SRC (31h) itself.[set]
	 * @param  val      change the values of lir_int1 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_pin_notification_mode_set(&self, lis2de12_lir_int1_t val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.lir_int1 = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** Latch interrupt request on INT1_SRC (31h), with INT1_SRC(31h)
	 *          register cleared by reading INT1_SRC (31h) itself.[get]
	 * @param  val      Get the values of lir_int1 in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn int1_pin_notification_mode_get(&self, lis2de12_lir_int1_t *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		*val = match ctrl_reg5.lir_int1 {
			LIS2DE12_INT1_PULSED => LIS2DE12_INT1_PULSED,
			LIS2DE12_INT1_LATCHED => LIS2DE12_INT1_LATCHED,
			_ => LIS2DE12_INT1_PULSED,
		}

		return ret;
	}

	/** Int2 pin routing configuration register.[set]
	 * @param  val      registers CTRL_REG6
	 * @retval          interface status (MANDATORY: return 0 -> no Error) */
	fn pin_int2_config_set(&self, lis2de12_ctrl_reg6_t *val) -> i32 {
		return self.write(LIS2DE12_CTRL_REG6, (u8 *) val, 1);
	}

	/** Int2 pin routing configuration register.[get]
	 * @param  val      registers CTRL_REG6
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn pin_int2_config_get(&self, lis2de12_ctrl_reg6_t *val) -> i32{
		return self.read(LIS2DE12_CTRL_REG6, (u8 *) val, 1);
	}

	// This section group all the functions concerning the fifo usage

	/** FIFO enable.[set]
	 * @param  val      change the values of fifo_en in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_set(&self, u8 val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);

		if ret == 0 {
			ctrl_reg5.fifo_en = val;
			ret = self.write(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		}

		return ret;
	}

	/** FIFO enable.[get]
	 * @param  val      change the values of fifo_en in reg CTRL_REG5
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_get(&self, u8 *val) -> i32 {
		lis2de12_ctrl_reg5_t ctrl_reg5;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG5, (u8 *)&ctrl_reg5, 1);
		*val = (u8)ctrl_reg5.fifo_en;

		return ret;
	}

	/** FIFO watermark level selection.[set]
	 * @param  val      change the values of fth in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_watermark_set(&self, u8 val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);

		if ret == 0 {
			fifo_ctrl_reg.fth = val;
			ret = self.write(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);
		}

		return ret;
	}

	/** FIFO watermark level selection.[get]
	 * @param  val      change the values of fth in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_watermark_get(&self, u8 *val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);
		*val = (u8)fifo_ctrl_reg.fth;

		return ret;
	}

	/** Trigger FIFO selection.[set]
	 * @param  val      change the values of tr in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_trigger_event_set(&self, lis2de12_tr_t val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);

		if ret == 0 {
			fifo_ctrl_reg.tr = (u8)val;
			ret = self.write(LIS2DE12_FIFO_CTRL_REG,  (u8 *)&fifo_ctrl_reg, 1);
		}

		return ret;
	}

	/** Trigger FIFO selection.[get]
	 * @param  val      Get the values of tr in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_trigger_event_get(&self, lis2de12_tr_t *val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);

		*val = match fifo_ctrl_reg.tr {
			LIS2DE12_INT1_GEN => LIS2DE12_INT1_GEN,
			LIS2DE12_INT2_GEN => LIS2DE12_INT2_GEN,
			_ => LIS2DE12_INT1_GEN,
		}

		return ret;
	}

	/** FIFO mode selection.[set]
	 * @param  val      change the values of fm in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_mode_set(&self, lis2de12_fm_t val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);

		if ret == 0 {
			fifo_ctrl_reg.fm = (u8)val;
			ret = self.write(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);
		}

		return ret;
	}

	/** FIFO mode selection.[get]
	 * @param  val      Get the values of fm in reg FIFO_CTRL_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_mode_get(&self, lis2de12_fm_t *val) -> i32 {
		lis2de12_fifo_ctrl_reg_t fifo_ctrl_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_CTRL_REG, (u8 *)&fifo_ctrl_reg, 1);

		*val = match fifo_ctrl_reg.fm {
			LIS2DE12_BYPASS_MODE => LIS2DE12_BYPASS_MODE,
			LIS2DE12_FIFO_MODE => LIS2DE12_FIFO_MODE,
			LIS2DE12_DYNAMIC_STREAM_MODE => LIS2DE12_DYNAMIC_STREAM_MODE,
			LIS2DE12_STREAM_TO_FIFO_MODE => LIS2DE12_STREAM_TO_FIFO_MODE,
			_ => LIS2DE12_BYPASS_MODE,
		}

		return ret;
	}

	/** FIFO status register.[get]
	 * @param  val      registers FIFO_SRC_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_status_get(&self, lis2de12_fifo_src_reg_t *val) -> i32 {
		return self.read(LIS2DE12_FIFO_SRC_REG, (u8 *) val, 1);
	}

	/** FIFO stored data level.[get]
	 * @param  val      change the values of fss in reg FIFO_SRC_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_data_level_get(&self, u8 *val) -> i32 {
		lis2de12_fifo_src_reg_t fifo_src_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_SRC_REG, (u8 *)&fifo_src_reg, 1);
		*val = (u8)fifo_src_reg.fss;
		return ret;
	}

	/** Empty FIFO status flag.[get]
	 * @param  val      change the values of empty in reg FIFO_SRC_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_empty_flag_get(&self, u8 *val) -> i32 {
		lis2de12_fifo_src_reg_t fifo_src_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_SRC_REG, (u8 *)&fifo_src_reg, 1);
		*val = (u8)fifo_src_reg.empty;
		return ret;
	}

	/** FIFO overrun status flag.[get]
	 * @param  val      change the values of ovrn_fifo in reg FIFO_SRC_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
		*
	 */
	fn fifo_ovr_flag_get(&self, u8 *val) -> i32 {
		lis2de12_fifo_src_reg_t fifo_src_reg;
		let ret: i32 = self.read(LIS2DE12_FIFO_SRC_REG, (u8 *)&fifo_src_reg, 1);
		*val = (u8)fifo_src_reg.ovrn_fifo;
		return ret;
	}

	/** FIFO watermark status.[get]
	 * @param  val      change the values of wtm in reg FIFO_SRC_REG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn fifo_fth_flag_get(&self, u8 *val) -> i32 {
		lis2de12_fifo_src_reg_t fifo_src_reg;
		let ret: i32 = self.read( LIS2DE12_FIFO_SRC_REG, (u8 *)&fifo_src_reg, 1);
		*val = (u8)fifo_src_reg.wtm;
		return ret;
	}

	// This section group all the functions that manage the tap and double tap event generation

	/** Tap/Double Tap generator configuration register.[set]
	 * @param  val      registers CLICK_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_conf_set(&self, lis2de12_click_cfg_t *val) -> i32 {
		return self.write(LIS2DE12_CLICK_CFG, (u8 *) val, 1);
	}

	/** Tap/Double Tap generator configuration register.[get]
	 * @param  val      registers CLICK_CFG
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_conf_get(&self, lis2de12_click_cfg_t *val) -> i32 {
		return self.read(LIS2DE12_CLICK_CFG, (u8 *) val, 1);
	}
	/** Tap/Double Tap generator source register.[get]
	 * @param  val      registers CLICK_SRC
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_source_get(&self, lis2de12_click_src_t *val): i32 {
		return self.read(LIS2DE12_CLICK_SRC, (u8 *) val, 1);
	}

	/** User-defined threshold value for Tap/Double Tap event.[set]
	 *         1 LSB = full scale/128
	 * @param  val      change the values of ths in reg CLICK_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_threshold_set(&self, u8 val) -> i32 {
		lis2de12_click_ths_t click_ths;
		let ret: i32 = self.read(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);

		if ret == 0 {
			click_ths.ths = val;
			ret = self.write(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);
		}

		return ret;
	}

	/** User-defined threshold value for Tap/Double Tap event.[get]
	 *         1 LSB = full scale/128
	 * @param  val      change the values of ths in reg CLICK_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_threshold_get(&self, u8 *val) -> i32 {
		lis2de12_click_ths_t click_ths;
		let ret: i32 = self.read(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);
		*val = (u8)click_ths.ths;
		return ret;
	}

	/** If the LIR_Click bit is not set, the interrupt is kept high
	 *          for the duration of the latency window.
	 *          If the LIR_Click bit is set, the interrupt is kept high until the
	 *          CLICK_SRC(39h) register is read.[set]
	 * @param  val      change the values of lir_click in reg CLICK_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn tap_notification_mode_set(&self, lis2de12_lir_click_t val) -> i32 {
		lis2de12_click_ths_t click_ths;
		let ret: i32 = self.read(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);
		if ret == 0 {
			click_ths.lir_click = (u8)val;
			ret = self.write(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);
		}

		return ret;
	}

	/** If the LIR_Click bit is not set, the interrupt is kept high
	 *          for the duration of the latency window.
	 *          If the LIR_Click bit is set, the interrupt is kept high until the
	 *          CLICK_SRC(39h) register is read.[get]
	 * @param  val      Get the values of lir_click in reg CLICK_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
		*
	 */
	fn tap_notification_mode_get(&self, lis2de12_lir_click_t *val) -> i32 {
		lis2de12_click_ths_t click_ths;
		let ret: i32 = self.read(LIS2DE12_CLICK_THS, (u8 *)&click_ths, 1);

		*val = match click_ths.lir_click {
			LIS2DE12_TAP_PULSED => LIS2DE12_TAP_PULSED,
			LIS2DE12_TAP_LATCHED => LIS2DE12_TAP_LATCHED,
			_ => LIS2DE12_TAP_PULSED,
		}

		return ret;
	}

	/** The maximum time (1 LSB = 1/ODR) interval that can elapse
	 *         between the start of the click-detection procedure and when the
	 *         acceleration falls back below the threshold.[set]
	 * @param  val      change the values of tli in reg TIME_LIMIT
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	i32 shock_dur_set(&self, u8 val) {
		lis2de12_time_limit_t time_limit;
		let ret: i32 = self.read(LIS2DE12_TIME_LIMIT, (u8 *)&time_limit, 1);

		if ret == 0 {
			time_limit.tli = val;
			ret = self.write(LIS2DE12_TIME_LIMIT, (u8 *)&time_limit, 1);
		}

		return ret;
	}

	/** The maximum time (1 LSB = 1/ODR) interval that can elapse between
	 *         the start of the click-detection procedure and when the
	 *         acceleration falls back below the threshold.[get]
	 * @param  val      change the values of tli in reg TIME_LIMIT
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn shock_dur_get(&self, u8 *val) -> i32 {
		lis2de12_time_limit_t time_limit;
		let ret: i32  = self.read(LIS2DE12_TIME_LIMIT, (u8 *)&time_limit, 1);
		*val = (u8)time_limit.tli;

		return ret;
	}

	/** The time (1 LSB = 1/ODR) interval that starts after the first
	 *         click detection where the click-detection procedure is
	 *         disabled, in cases where the device is configured for
	 *         double-click detection.[set]
	 * @param  val      change the values of tla in reg TIME_LATENCY
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn quiet_dur_set(&self, u8 val) -> i32 {
		lis2de12_time_latency_t time_latency;
		let ret: i32 = self.read(LIS2DE12_TIME_LATENCY, (u8 *)&time_latency, 1);

		if ret == 0 {
			time_latency.tla = val;
			ret = self.write(LIS2DE12_TIME_LATENCY, (u8 *)&time_latency, 1);
		}

		return ret;
	}

	/** The time (1 LSB = 1/ODR) interval that starts after the first
	 *         click detection where the click-detection procedure is
	 *         disabled, in cases where the device is configured for
	 *         double-click detection.[get]
	 * @param  val      change the values of tla in reg TIME_LATENCY
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn quiet_dur_get(&self, u8 *val) -> i32 {
		lis2de12_time_latency_t time_latency;
		let ret: i32 = self.read(LIS2DE12_TIME_LATENCY, (u8 *)&time_latency, 1);
		*val = (u8)time_latency.tla;

		return ret;
	}

	/** The maximum interval of time (1 LSB = 1/ODR) that can elapse
	 *         after the end of the latency interval in which the click-detection
	 *         procedure can start, in cases where the device is configured
	 *         for double-click detection.[set]
	 * @param  val      change the values of tw in reg TIME_WINDOW
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn double_tap_timeout_set(&self, u8 val) -> i32 {
		lis2de12_time_window_t time_window;
		let ret: i32 = self.read(LIS2DE12_TIME_WINDOW, (u8 *)&time_window, 1);

		if ret == 0 {
			time_window.tw = val;
			ret = self.write(LIS2DE12_TIME_WINDOW, (u8 *)&time_window, 1);
		}

		return ret;
	}

	/** The maximum interval of time (1 LSB = 1/ODR) that can elapse
	 *         after the end of the latency interval in which the
	 *         click-detection procedure can start, in cases where the device
	 *         is configured for double-click detection.[get]
	 * @param  val      change the values of tw in reg TIME_WINDOW
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn double_tap_timeout_get(&self, u8 *val) -> i32{
		lis2de12_time_window_t time_window;
		let ret: i32 = self.read(LIS2DE12_TIME_WINDOW, (u8 *)&time_window, 1);
		*val = (u8)time_window.tw;

		return ret;
	}

	// This section groups all functions concerning activity inactivity functionality

	/** Sleep-to-wake, return-to-sleep activation threshold in
	 *           low-power mode.[set]
	 *           1 LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of acth in reg ACT_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn act_threshold_set(&self, u8 val) -> i32 {
		lis2de12_act_ths_t act_ths;
		let ret: i32 = self.read(LIS2DE12_ACT_THS, (u8 *)&act_ths, 1);

		if ret == 0 {
			act_ths.acth = val;
			ret = self.write(LIS2DE12_ACT_THS, (u8 *)&act_ths, 1);
		}

		return ret;
	}

	/** Sleep-to-wake, return-to-sleep activation threshold in low-power
	 *         mode.[get]
	 *         1 LSb = 16mg@2g / 32mg@4g / 62mg@8g / 186mg@16g
	 * @param  val      change the values of acth in reg ACT_THS
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn act_threshold_get(&self, u8 *val) -> i32 {
		lis2de12_act_ths_t act_ths;
		let ret: i32 = self.read(LIS2DE12_ACT_THS, (u8 *)&act_ths, 1);
		*val = (u8)act_ths.acth;
		return ret;
	}

	/** Sleep-to-wake, return-to-sleep.[set]
	 *         duration = (8*1[LSb]+1)/ODR
	 * @param  val      change the values of actd in reg ACT_DUR
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn act_timeout_set(&self, u8 val) -> i32 {
		lis2de12_act_dur_t act_dur;
		let ret: i32 = self.read(LIS2DE12_ACT_DUR, (u8 *)&act_dur, 1);

		if ret == 0 {
			act_dur.actd = val;
			ret = self.write(LIS2DE12_ACT_DUR, (u8 *)&act_dur, 1);
		}

		return ret;
	}

	/** Sleep-to-wake, return-to-sleep.[get] duration = (8*1[LSb]+1)/ODR
	 * @param  val      change the values of actd in reg ACT_DUR
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn act_timeout_get(&self, u8 *val) -> i32 {
		lis2de12_act_dur_t act_dur;
		let ret: i32 = self.read(LIS2DE12_ACT_DUR, (u8 *)&act_dur, 1);
		*val = (u8)act_dur.actd;
		return ret;
	}

	// This section group all the functions concerning serial interface management

	/** Connect/Disconnect SDO/SA0 internal pull-up.[set]
	 * @param  val      change the values of sdo_pu_disc in reg CTRL_REG0
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn pin_sdo_sa0_mode_set(&self, lis2de12_sdo_pu_disc_t val) -> i32 {
		lis2de12_ctrl_reg0_t ctrl_reg0;
		let mut ret: i32 = self.read(LIS2DE12_CTRL_REG0, (u8 *)&ctrl_reg0, 1);

		if ret == 0 {
			ctrl_reg0.sdo_pu_disc = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG0, (u8 *)&ctrl_reg0, 1);
		}

		return ret;
	}

	/** Connect/Disconnect SDO/SA0 internal pull-up.[get]
	 * @param  val      Get the values of sdo_pu_disc in reg CTRL_REG0
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn pin_sdo_sa0_mode_get(&self, lis2de12_sdo_pu_disc_t *val) -> i32 {
		lis2de12_ctrl_reg0_t ctrl_reg0;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG0, (u8 *)&ctrl_reg0, 1);

		*val = match ctrl_reg0.sdo_pu_disc {
			LIS2DE12_PULL_UP_DISCONNECT => LIS2DE12_PULL_UP_DISCONNECT,
			LIS2DE12_PULL_UP_CONNECT => LIS2DE12_PULL_UP_CONNECT,
			_ => LIS2DE12_PULL_UP_DISCONNECT,
		}

		return ret;
	}

	/** SPI Serial Interface Mode selection.[set]
	 * @param  val      change the values of sim in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
     */
	fn spi_mode_set(&self, lis2de12_sim_t val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		if ret == 0 {
			ctrl_reg4.sim = (u8)val;
			ret = self.write(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);
		}

		return ret;
	}

	/** SPI Serial Interface Mode selection.[get]
	 * @param  val      Get the values of sim in reg CTRL_REG4
	 * @retval          interface status (MANDATORY: return 0 -> no Error)
	 */
	fn spi_mode_get(&self, lis2de12_sim_t *val) -> i32 {
		lis2de12_ctrl_reg4_t ctrl_reg4;
		let ret: i32 = self.read(LIS2DE12_CTRL_REG4, (u8 *)&ctrl_reg4, 1);

		*val = match ctrl_reg4.sim {
			LIS2DE12_SPI_4_WIRE => LIS2DE12_SPI_4_WIRE,
			LIS2DE12_SPI_3_WIRE => LIS2DE12_SPI_3_WIRE,
			_ => LIS2DE12_SPI_4_WIRE,
		}

		return ret;
	}
}