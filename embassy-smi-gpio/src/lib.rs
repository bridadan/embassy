#![no_std]

use embassy_net_driver::smi::StationManagement;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

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
    /// Create a new SMI driver.
    ///
    /// The MDIO pin must be configured in a way that allows reading the pin
    /// state while it is in output mode, or it must be an open-drain pin.
    /// When this driver is used with a push-pull output pin, the bus will
    /// be shorted when the PHY tries to drive the MDIO line.
    pub fn new(mdc: Mdc, mdio: Mdio, delay: &'d mut dyn DelayNs) -> Self {
        Self { mdc, mdio, delay }
    }

    fn write_bits(&mut self, data: u32, len: u8) {
        // Reverse it due to MSB first
        for i in (0..len).rev() {
            self.mdc.set_low().ok();

            if (data >> i) & 1 != 0 {
                self.mdio.set_high().ok();
            } else {
                self.mdio.set_low().ok();
            }

            self.delay.delay_us(1);

            self.mdc.set_high().ok();
            self.delay.delay_us(1);
        }
    }

    fn read_data(&mut self) -> u16 {
        let mut data = 0u16;
        for _ in 0..16 {
            data <<= 1;

            self.mdc.set_low().ok();
            self.delay.delay_us(1);

            if self.mdio.is_high().unwrap_or(false) {
                data |= 1;
            }

            self.mdc.set_high().ok();
            self.delay.delay_us(1);
        }
        data
    }

    fn turnaround_read(&mut self) {
        self.mdc.set_low().ok();
        self.delay.delay_us(1);
        self.mdio.set_high().ok(); // high-Z
        self.mdc.set_high().ok();
        self.delay.delay_us(1);
    }

    fn turnaround_write(&mut self) {
        self.write_bits(0b10, 2);
    }
}

impl<'d, Mdc, Mdio> StationManagement for SmiGpio<'d, Mdc, Mdio>
where
    Mdc: OutputPin,
    Mdio: InputPin + OutputPin,
{
    fn smi_read(&mut self, phy_addr: u8, reg: u8) -> u16 {
        // Preamble
        self.write_bits(0xFFFFFFFF, 32);

        // Start of Frame + Opcode (read)
        self.write_bits(0b0110, 4);

        // PHY Address + Register Address
        self.write_bits(u32::from(phy_addr), 5);
        self.write_bits(u32::from(reg), 5);

        // Turnaround
        self.turnaround_read();

        // Read data
        let data = self.read_data();

        // End with clock low, bus idle.
        self.mdc.set_low().ok();

        data
    }

    fn smi_write(&mut self, phy_addr: u8, reg: u8, val: u16) {
        // Preamble
        self.write_bits(0xFFFFFFFF, 32);

        // Start of Frame + Opcode (write)
        self.write_bits(0b0101, 4);

        // PHY Address + Register Address
        self.write_bits(u32::from(phy_addr), 5);
        self.write_bits(u32::from(reg), 5);

        // Turnaround
        self.turnaround_write();

        // Write data
        self.write_bits(u32::from(val), 16);

        // End with clock low, bus idle.
        self.mdc.set_low().ok();
    }
}
