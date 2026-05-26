use crate::{ZxError, ZxResult};

/// Returns the BDF address without the bottom two bits masked off.
pub fn pci_bdf_raw_addr(bus: u8, dev: u8, func: u8, offset: u8) -> u32 {
    ((bus as u32 & 0xff) << 16)         // bits 23-16 bus
        | ((dev as u32 & 0x1f) << 11)   // bits 15-11 device
        | ((func as u32 & 0x7) << 8)    // bits 10-8 func
        | (offset as u32 & 0xff) // bits 7-2 reg, with bottom 2 bits as well
}

pub fn pmio_config_read_addr(_addr: u32, _width: usize) -> ZxResult<u32> {
    Err(ZxError::NOT_SUPPORTED)
}
pub fn pmio_config_write_addr(_addr: u32, _val: u32, _width: usize) -> ZxResult {
    Err(ZxError::NOT_SUPPORTED)
}

pub fn pio_config_read(bus: u8, dev: u8, func: u8, offset: u8, width: usize) -> ZxResult<u32> {
    pmio_config_read_addr(pci_bdf_raw_addr(bus, dev, func, offset), width)
}

pub fn pio_config_write(
    bus: u8,
    dev: u8,
    func: u8,
    offset: u8,
    val: u32,
    width: usize,
) -> ZxResult {
    pmio_config_write_addr(pci_bdf_raw_addr(bus, dev, func, offset), val, width)
}
