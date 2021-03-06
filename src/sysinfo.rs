use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Sysinfo(pub *mut u8);
unsafe impl Send for Sysinfo {}
unsafe impl Sync for Sysinfo {}
impl Sysinfo {
    #[doc = "JEDEC JEP-106 compliant chip identifier."]
    pub fn chip_id(self) -> Reg<regs::ChipId, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Platform register. Allows software to know what environment it is running in."]
    pub fn platform(self) -> Reg<regs::Platform, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Git hash of the chip source. Used to identify chip version."]
    pub fn gitref_rp2040(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(64usize)) }
    }
}
pub mod regs;
