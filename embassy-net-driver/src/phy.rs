use core::task::Context;

use crate::smi::StationManagement;

/// Trait for an Ethernet PHY
pub trait Phy {
    /// Reset PHY and wait for it to come out of reset.
    fn phy_reset<S: StationManagement>(&mut self, sm: &mut S);
    /// PHY initialisation.
    fn phy_init<S: StationManagement>(&mut self, sm: &mut S);
    /// Poll link to see if it is up and FD with 100Mbps
    fn poll_link<S: StationManagement>(&mut self, sm: &mut S, cx: &mut Context) -> bool;
}
