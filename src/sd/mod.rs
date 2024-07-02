use self::{
    err::CardError,
    ops::{read_block, write_block},
};

mod cmd;
pub mod err;
mod ops;
mod reg;
mod sd_reg;
mod utils;
pub struct SdHost;
impl SdHost {
    pub fn init(&self) -> Result<(), CardError> {
        ops::init_card()
    }
    pub fn read_block(&self, addr: u32, buf: &mut [u8; 512]) -> Result<(), CardError> {
        read_block(buf, addr)
    }
    pub fn write_block(&self, addr: u32, buf: &[u8; 512]) -> Result<(), CardError> {
        write_block(buf, addr)
    }
}
