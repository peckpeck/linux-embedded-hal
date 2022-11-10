use embedded_hal::i2c::{I2c, Operation as I2cOperation};
use linux_embedded_hal::I2cdev;

const ADDR: u8 = 0x12;

struct Driver<I2C> {
    i2c: I2C,
}

impl<I2C> Driver<I2C>
where
    I2C: I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Driver { i2c }
    }

    fn read_something(&mut self) -> Result<u8, I2C::Error> {
        let mut read_buffer = [0];
        let mut ops = [
            I2cOperation::Write(&[0xAB]),
            I2cOperation::Read(&mut read_buffer),
        ];
        self.i2c.transaction(ADDR, &mut ops).and(Ok(read_buffer[0]))
    }
}

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut driver = Driver::new(dev);
    let value = driver.read_something().unwrap();
    println!("Read value: {}", value);
}
