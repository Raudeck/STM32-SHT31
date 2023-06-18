use stm32f4xx_hal::{pac::{self, USART1, I2C1}, prelude::*, gpio::{Output}, timer::SysDelay, i2c::{I2c, Mode}};
use cortex_m;
use core::{fmt::Write};
use core::cell::RefCell;

pub struct Function {
    pub led: stm32f4xx_hal::gpio::Pin<'B',12, Output>,
    pub delay: SysDelay,
    pub usart1: stm32f4xx_hal::serial::Tx<USART1>,
    pub i2c: RefCell<I2c<I2C1>>
}

impl Function {
    pub fn new() -> Self {
        let p = pac::Peripherals::take().unwrap();
        let clock = p.RCC.constrain().cfgr.hclk(100.MHz()).freeze();
        let gpiob = p.GPIOB.split();

        let mut tx = p.USART1.tx(
            p.GPIOA.split().pa9,
            9600.bps(),
            &clock
        ).unwrap();

        writeln!(tx, "Serial Initialization\r").unwrap();
        let scl = gpiob.pb6.internal_pull_up(true);
        let sda = gpiob.pb7.internal_pull_up(true);

        let i2c = p.I2C1.i2c((scl, sda), Mode::Standard { frequency: 100.kHz() }, &clock);

        Function {
            led: gpiob.pb12.into_push_pull_output(),
            delay: cortex_m::Peripherals::take().unwrap().SYST.delay(
                &clock
            ),
            usart1: tx,
            i2c: RefCell::new(i2c)
        }
    }

    pub fn blinking(&mut self) {
        self.led.set_high();
        self.delay.delay_ms(1000u32);
        self.led.set_low();
        self.delay.delay_ms(1000u32);
    }

    pub fn serial_print(&mut self, string: &str) {
        writeln!(self.usart1, "{}", string).unwrap();
    }

    fn _crc_calculation(&mut self, data: &[u8]) -> Result<u8, bool> {
        const POLYNOMIAL: u8 = 0x31;
        const INIT_VAL: u8 = 0xff;
        let mut crc_res = INIT_VAL;
        
        for i in 0..2 {
            crc_res ^= data[i];
            for _j in 0u8..8 {
                if (crc_res & 0x80) != 0 {
                    crc_res = (crc_res << 1) ^ POLYNOMIAL;
                } else {
                    crc_res <<= 1
                }
            }
        }

        Ok(crc_res)
    }

    pub fn sht31(&mut self) -> Result<[f32; 2], bool>{
        let mut data:[u8; 6] = [0; 6];
        //writeln!(self.usart1, "Hex for 0x44 << 1{}", 0x44 << 1).unwrap();
        self.i2c.borrow_mut().write(0x44, &[0x30, 0xa2]).unwrap();
        self.i2c.borrow_mut().write(0x44, &[0x2c, 0x06]).unwrap();
        self.delay.delay_ms(1000u32);
        self.i2c.borrow_mut().read(0x44, &mut data).unwrap();

        let temp: f32 = f32::from(
            u16::from((data[0] as u16) << 8
        ) | data[1] as u16) * 175. / (65536. - 1.) - 45.;

        let humd: f32 = f32::from(
            u16::from(((data[3] as u16) << 8) | data[4] as u16)
        ) * 100. / ((2 << 16) as f32 - 1.);
        
        let temp_crc = self._crc_calculation(&[data[0], data[1]]).unwrap();
        let humd_crc = self._crc_calculation(&[data[3], data[4]]).unwrap();
        let mut temp_check = false;
        let mut humd_check = false;
        if data[2] == temp_crc {
            writeln!(self.usart1, "Temperature: {:?} Celsius", temp).unwrap();
            temp_check = true;
        } else {
            writeln!(self.usart1, "Temperature Data NOT Verified").unwrap();
        }

        if data[5] == humd_crc {
            writeln!(self.usart1, "Humidity: {:?} %RH", humd).unwrap();
            humd_check = true;
        } else {
            writeln!(self.usart1, "Humidity Data NOT Verified").unwrap();
        }
        
        if temp_check && humd_check {
            return Ok([temp, humd]);
        } else {
            return  Err(false);
        }
    }
}

