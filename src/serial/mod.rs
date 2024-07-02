mod config;

pub use config::*;
use core::{fmt::Write, ptr};
pub const UART0_BASE: u64 = 0x10000000;
pub static mut UART: Option<Uart> = None;
#[derive(Copy, Clone, Debug)]
pub struct Uart {
    base_address: u64,
}

impl Uart {
    fn init(
        &self,
        word_length: WordLength,
        stop_bits: StopBits,
        parity_bit: Parity,
        parity_select: ParitySelect,
        brk: Break,
    ) {
        self.set_lcr(
            word_length,
            stop_bits,
            parity_bit,
            parity_select,
            brk,
            DLAB::SET,
        );

        let ptr = (self.base_address) as *mut u8;
        unsafe {
            ptr.write_volatile(DIVISOR);
        }
        self.set_lcr(
            word_length,
            stop_bits,
            parity_bit,
            parity_select,
            brk,
            DLAB::CLEAR,
        );
    }

    fn set_lcr(
        &self,
        word_length: WordLength,
        stop_bits: StopBits,
        parity_bit: Parity,
        parity_select: ParitySelect,
        brk: Break,
        dlab: DLAB,
    ) {
        let ptr = (self.base_address + 3) as *mut u8;
        unsafe {
            ptr.write_volatile(
                word_length as u8
                    | ((stop_bits as u8) << 2)
                    | ((parity_bit as u8) << 3)
                    | ((parity_select as u8) << 4)
                    | ((brk as u8) << 6)
                    | ((dlab as u8) << 7),
            );
        }
    }

    fn put(&self, c: u8) {
        let tx: *mut u8 = self.base_address as *mut u8;
        unsafe {
            loop {
                let lsr = (self.base_address + 0x14) as *const u32;
                if ptr::read_volatile(lsr) >> 5 & 1 == 1 {
                    break;
                }
            }
            tx.write_volatile(c);
        }
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        s.as_bytes().iter().for_each(|byte| {
            self.put(*byte);
        });
        Ok(())
    }
}

pub fn init_log(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    let uart = Uart {
        base_address: UART0_BASE,
    };
    uart.init(
        WordLength::EIGHT,
        StopBits::ONE,
        Parity::DISABLE,
        ParitySelect::EVEN,
        Break::DISABLE,
    );
    unsafe {
        UART = Some(uart);
    }
    unsafe { MAX_LOG_LEVEL = level.to_level().unwrap() };
    log::set_logger(&LOGGER).map(|()| log::set_max_level(level))
}

#[macro_export]
macro_rules! println {
    () => {
        writeln!(unsafe{$crate::serial::UART.as_mut().unwrap()}).unwrap();
    };
    ($($arg:tt)*) => {
        writeln!(unsafe{$crate::serial::UART.as_mut().unwrap()},$($arg)*).unwrap();
    };
}

#[macro_export]
macro_rules! print {
    () => {
        write!(unsafe{$crate::serial::UART.as_mut().unwrap()}).unwrap();
    };
    ($($arg:tt)*) => {
        write!(unsafe{$crate::serial::UART.as_mut().unwrap()},$($arg)*).unwrap();
    };
}

use log::{Level, Metadata, Record};

static LOGGER: SimpleLogger = SimpleLogger;
static mut MAX_LOG_LEVEL: Level = Level::Trace;
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= unsafe { MAX_LOG_LEVEL }
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}
