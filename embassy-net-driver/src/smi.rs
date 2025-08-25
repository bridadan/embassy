/// Serial Management Interface (SMI) on an ethernet PHY
pub trait SerialManagement {
    /// Read a register over SMI.
    fn smi_read(&mut self, phy_addr: u8, reg: u8) -> u16;
    /// Write a register over SMI.
    fn smi_write(&mut self, phy_addr: u8, reg: u8, val: u16);
}
