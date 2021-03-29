use crate::generic::*;
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SmExecctrlStatusSel(pub u8);
impl SmExecctrlStatusSel {
    #[doc = "All-ones if TX FIFO level < N, otherwise all-zeroes"]
    pub const TXLEVEL: Self = Self(0);
    #[doc = "All-ones if RX FIFO level < N, otherwise all-zeroes"]
    pub const RXLEVEL: Self = Self(0x01);
}
