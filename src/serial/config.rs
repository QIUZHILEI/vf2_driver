/// Baud rate 115200
/// Baud Rate	|   Divisor (in decimal)	|   Divisor Latch High Byte	|   Divisor Latch Low Byte
/// 115200	    |   1	                    |   $00	                    |   $01
pub const DIVISOR: u8 = 13;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Word length
pub enum WordLength {
    FIVE = 0,
    SIX = 1,
    SEVEN = 2,
    EIGHT = 3,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Number of stop bits
pub enum StopBits {
    ONE = 0,
    TWO = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity bits
pub enum Parity {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Parity select
pub enum ParitySelect {
    EVEN = 0,
    ODD = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Break
pub enum Break {
    DISABLE = 0,
    ENABLE = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// Divisor latch access bit
pub enum DLAB {
    CLEAR = 0,
    SET = 1,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// DMA mode select
pub enum DMAMode {
    MODE0 = 0,
    MODE1 = 1,
}
