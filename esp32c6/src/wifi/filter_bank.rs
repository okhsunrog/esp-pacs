#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces. Unlike the Wi-Fi 4 MAC there are no mask registers, the enable bit lives in ADDR_HIGH."]
pub struct FILTER_BANK {
    addr_low: (),
    _reserved1: [u8; 0x04],
    addr_high: (),
    _reserved_end: [u8; 0x58],
}
impl FILTER_BANK {
    #[doc = "0x00..0x10 - First 4 bytes of the MAC address filter"]
    #[inline(always)]
    pub const fn addr_low(&self, n: usize) -> &ADDR_LOW {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - First 4 bytes of the MAC address filter"]
    #[inline(always)]
    pub fn addr_low_iter(&self) -> impl Iterator<Item = &ADDR_LOW> {
        (0..4).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8 * n).cast() })
    }
    #[doc = "0x04..0x14 - Last 2 bytes of the MAC address filter and its enable bits"]
    #[inline(always)]
    pub const fn addr_high(&self, n: usize) -> &ADDR_HIGH {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x14 - Last 2 bytes of the MAC address filter and its enable bits"]
    #[inline(always)]
    pub fn addr_high_iter(&self) -> impl Iterator<Item = &ADDR_HIGH> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(8 * n)
                .cast()
        })
    }
}
#[doc = "ADDR_LOW (rw) register accessor: First 4 bytes of the MAC address filter\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_low`] module"]
pub type ADDR_LOW = crate::Reg<addr_low::ADDR_LOW_SPEC>;
#[doc = "First 4 bytes of the MAC address filter"]
pub mod addr_low;
#[doc = "ADDR_HIGH (rw) register accessor: Last 2 bytes of the MAC address filter and its enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_high`] module"]
pub type ADDR_HIGH = crate::Reg<addr_high::ADDR_HIGH_SPEC>;
#[doc = "Last 2 bytes of the MAC address filter and its enable bits"]
pub mod addr_high;
