use crate::generic::*;
#[doc = "Voltage select. Per bank control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VoltageSelect(pub u32);
impl VoltageSelect {
    pub const fn voltage_select(&self) -> super::vals::VoltageSelect {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::VoltageSelect(val as u8)
    }
    pub fn set_voltage_select(&mut self, val: super::vals::VoltageSelect) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for VoltageSelect {
    fn default() -> VoltageSelect {
        VoltageSelect(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0(pub u32);
impl GpioQspiSd0 {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSd0Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSd0Drive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSd0Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd0 {
    fn default() -> GpioQspiSd0 {
        GpioQspiSd0(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSs(pub u32);
impl GpioQspiSs {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSsDrive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSsDrive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSsDrive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSs {
    fn default() -> GpioQspiSs {
        GpioQspiSs(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3(pub u32);
impl GpioQspiSd3 {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSd3Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSd3Drive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSd3Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd3 {
    fn default() -> GpioQspiSd3 {
        GpioQspiSd3(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclk(pub u32);
impl GpioQspiSclk {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSclkDrive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSclkDrive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSclkDrive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSclk {
    fn default() -> GpioQspiSclk {
        GpioQspiSclk(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2(pub u32);
impl GpioQspiSd2 {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSd2Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSd2Drive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSd2Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd2 {
    fn default() -> GpioQspiSd2 {
        GpioQspiSd2(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1(pub u32);
impl GpioQspiSd1 {
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::vals::GpioQspiSd1Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::GpioQspiSd1Drive(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::vals::GpioQspiSd1Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd1 {
    fn default() -> GpioQspiSd1 {
        GpioQspiSd1(0)
    }
}
