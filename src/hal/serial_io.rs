use std::io::{self, Read, Write, ErrorKind};
use std::io::Error as IoError;
use std::fmt;
use std::error::Error;
use ::error::*;
use raw::HAL_SerialPort;
use hal::{i2c, spi};

pub type RawSerialPort = HAL_SerialPort;

lazy_static! {
    static ref INITIALIZED_SERIAL_PORTS: Vec<SerialPort> = Vec::new();
    static ref INITIALIZED_SPI_PORTS: Vec<i32> = Vec::new();
    static ref INITIALIZED_I2C_PORTS: Vec<i32> = Vec::new();
}

#[derive(Copy, Clone, Debug)]
pub enum SerialError {
    ReadError,
    WriteError,
}

impl fmt::Display for SerialError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   SerialError::ReadError => "",
                   SerialError::WriteError => "",
               })
    }
}

impl Error for SerialError {
    fn description(&self) -> &str {
        "Serail I/O Error"
    }
}

pub trait HalSerialIO {
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32>;
    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32>;
    fn hal_flush(&mut self) -> HalResult<()> {
        Ok(())
    }
}

impl Read for HalSerialIO {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let read_count = self.hal_read(buf).map_err(|err| IoError::new(ErrorKind::Other, err))?;

        if read_count <= -1 {
            Err(IoError::new(ErrorKind::Other, SerialError::ReadError))
        } else {
            Ok(read_count as usize)
        }
    }
}

impl Write for HalSerialIO {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let write_count = self.hal_write(buf).map_err(|err| IoError::new(ErrorKind::Other, err))?;

        if write_count <= -1 {
            Err(IoError::new(ErrorKind::Other, SerialError::WriteError))
        } else {
            Ok(write_count as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        self.hal_flush().map_err(|err| IoError::new(ErrorKind::Other, err))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SerialPort {
    OnBoard,
    MXP,
    USB1,
    USB2,
}

impl SerialPort {
    pub fn into_raw(&self) -> RawSerialPort {
        match *self {
            SerialPort::OnBoard => HAL_SerialPort::HAL_SerialPort_Onboard,
            SerialPort::MXP => HAL_SerialPort::HAL_SerialPort_MXP,
            SerialPort::USB1 => HAL_SerialPort::HAL_SerialPort_USB1,
            SerialPort::USB2 => HAL_SerialPort::HAL_SerialPort_USB2,
        }
    }
}

impl From<RawSerialPort> for SerialPort {
    fn from(raw: RawSerialPort) -> Self {
        match raw {
            HAL_SerialPort::HAL_SerialPort_Onboard => SerialPort::OnBoard,
            HAL_SerialPort::HAL_SerialPort_MXP => SerialPort::MXP,
            HAL_SerialPort::HAL_SerialPort_USB1 => SerialPort::USB1,
            HAL_SerialPort::HAL_SerialPort_USB2 => SerialPort::USB2,
        }
    }
}

pub struct SerialOptions {
    pub read_size: i32,
}

impl Default for SerialOptions {
    fn default() -> Self {
        SerialOptions { read_size: 1 }
    }
}

pub struct SerialDevice {
    port: SerialPort,
    opts: SerialOptions,
}

impl SerialDevice {
    pub fn new(port: SerialPort) -> Option<SerialDevice> {
        if INITIALIZED_SERIAL_PORTS.contains(&port) {
            None
        } else {
            Some(SerialDevice {
                port: port,
                opts: Default::default(),
            })
        }
    }
}

/// Which port the SPI is plugged into
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SpiPort {
    CS0,
    CS1,
    CS2,
    CS3,
    MXP,
    Unknown(i32),
}

impl SpiPort {
    pub fn get_port(&self) -> i32 {
        match *self {
            SpiPort::CS0 => 0,
            SpiPort::CS1 => 1,
            SpiPort::CS2 => 2,
            SpiPort::CS3 => 3,
            SpiPort::MXP => 4,
            SpiPort::Unknown(k) => k,
        }
    }
}

impl From<i32> for SpiPort {
    fn from(i: i32) -> SpiPort {
        match i {
            0 => SpiPort::CS0,
            1 => SpiPort::CS1,
            2 => SpiPort::CS2,
            3 => SpiPort::CS3,
            4 => SpiPort::MXP,
            k => SpiPort::Unknown(k),
        }
    }
}

/// Options for an SPI
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SpiOptions {
    /// The number of bytes to read per call
    pub read_size: i32,
}

impl Default for SpiOptions {
    fn default() -> Self {
        SpiOptions { read_size: 1 }
    }
}

/// Represents an SPI on the robot; there should only ever be the 4 on the RoboRIO
pub struct HalSpi {
    /// The port of this SPI
    port: i32,
    /// Options for this SPI
    opts: SpiOptions,
}

impl HalSpi {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: SpiPort) -> HalResult<HalSpi> {
        if INITIALIZED_SPI_PORTS.contains(&port.get_port()) {
            Err(HalError::ResourceAlreadyInitialized)
        } else {
            spi::initialize_spi(port.get_port())?;

            Ok(HalSpi {
                port: port.get_port(),
                opts: Default::default(),
            })
        }
    }

    /// Creates a new SPI instance from a port number
    ///
    /// # Safety
    /// Trying to read or write to the same SPI port at the same time from two different threads
    /// could lead to data races. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, opts: SpiOptions) -> HalResult<HalSpi> {
        spi::initialize_spi(port)?;

        Ok(HalSpi {
            port: port,
            opts: opts,
        })
    }

    /// Set the clock speed of this SPI
    pub fn set_speed(&self, speed: i32) {
        spi::set_spi_speed(self.get_handle(), speed)
    }

    /// Set this SPI's options
    pub fn set_opts(&self, msb_first: bool, sample_on_trailing: bool, clock_idle_high: bool) {
        spi::set_spi_opts(self.port, msb_first, sample_on_trailing, clock_idle_high);
    }

    pub fn set_chip_select_active_high(&self) -> HalResult<()> {
        spi::set_spi_chip_select_active_high(self.port)
    }

    /// Get this SPI's handle
    pub fn get_handle(&self) -> i32 {
        spi::get_spi_handle(self.port)
    }

    /// Set this SPI's handle
    pub fn set_handle(&self, handle: i32) {
        spi::set_spi_handle(self.port, handle);
    }
}

impl HalSerialIO for HalSpi {
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32> {
        Ok(spi::read_spi(self.port, buf, self.opts.read_size))
    }

    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32> {
        Ok(spi::write_spi(self.port, buf, buf.len() as i32))
    }
}

impl Drop for HalSpi {
    fn drop(&mut self) {
        spi::close_spi(self.port)
    }
}

/// Which port the SPI is plugged into
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum I2cPort {
    OnBoard,
    MXP,
}

impl I2cPort {
    pub fn get_port(&self) -> i32 {
        match *self {
            I2cPort::OnBoard => 0,
            I2cPort::MXP => 1,
        }
    }
}

/// Options for an I2C
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct I2cOptions {
    /// The number of bytes to read per call
    pub read_size: i32,
}

impl Default for I2cOptions {
    fn default() -> Self {
        I2cOptions { read_size: 1 }
    }
}

/// Represents an I2C on the robot; there should only ever be the 4 on the RoboRIO
pub struct I2C {
    /// The port of this I2C
    port: i32,
    /// TODO: What is this?
    address: i32,
    /// Options for this I2C
    opts: I2cOptions,
}

impl I2C {
    /// Construct and initialize a serial port with the default settings
    pub fn new(port: I2cPort, address: i32) -> Option<I2C> {
        I2C::new_with_opts(port, address, Default::default())
    }

    /// Construct and initialize a serial port using passed in options
    pub fn new_with_opts(port: I2cPort, address: i32, opts: I2cOptions) -> Option<I2C> {
        if INITIALIZED_I2C_PORTS.contains(&port.get_port()) {
            None
        } else {
            i2c::initialize_i2c(port.get_port());

            Some(I2C {
                port: port.get_port(),
                address: address,
                opts: opts,
            })
        }
    }

    /// Creates a new I2C instance from a port number
    ///
    /// # Safety
    /// Trying to read or write to the same I2C port at the same time from two different threads
    /// would cause a data race. The actual initialization is not unsafe.
    pub unsafe fn new_raw(port: i32, address: i32, opts: I2cOptions) -> I2C {
        i2c::initialize_i2c(port);

        I2C {
            port: port,
            address: address,
            opts: opts,
        }
    }
}

impl HalSerialIO for I2C {
    fn hal_read(&mut self, buf: &mut [u8]) -> HalResult<i32> {
        Ok(i2c::read_i2c(self.port, self.address, buf, self.opts.read_size))
    }

    fn hal_write(&mut self, buf: &[u8]) -> HalResult<i32> {
        Ok(i2c::write_i2c(self.port, self.address, buf, buf.len() as i32))
    }
}

impl Drop for I2C {
    fn drop(&mut self) {
        i2c::close_i2c(self.port)
    }
}