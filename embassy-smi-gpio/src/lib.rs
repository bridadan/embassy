#![no_std]

use embassy_net_driver::smi::StationManagement;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::delay::DelayNs;

pub struct SmiGpio<'d, Mdc, Mdio> {
    mdc: Mdc,
    mdio: Mdio,
    delay: &'d mut dyn DelayNs,
}

impl<'d, Mdc, Mdio> SmiGpio<'d, Mdc, Mdio>
where
    Mdc: OutputPin,
    Mdio: InputPin + OutputPin,
{
    pub fn new(mdc: Mdc, mdio: Mdio, delay: &'d mut dyn DelayNs) -> Self {
        Self { mdc, mdio, delay }
    }
}

impl<'d, Mdc, Mdio> StationManagement for SmiGpio<'d, Mdc, Mdio>
where
    Mdc: OutputPin,
    Mdio: InputPin + OutputPin,
{
    fn smi_read(&mut self, phy_addr: u8, reg: u8) -> u16 {
        // Implementation will go here
        0
    }

    fn smi_write(&mut self, phy_addr: u8, reg: u8, val: u16) {
        // Implementation will go here
    }
}
