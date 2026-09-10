#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Used to configure the TX slot."]
pub struct TX_SLOT_CONFIG {
    conf0: CONF0,
    conf1: CONF1,
    config: CONFIG,
    plcp0: PLCP0,
}
impl TX_SLOT_CONFIG {
    #[doc = "0x00 - Three 10 bit fields written by mac_tx_set_htsig, bit 31 set by mac_tx_set_txop_q; printed as CONF2 by lmacProcessTxComplete"]
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    #[doc = "0x04 - Bit 3 is cleared by hal_mac_tx_set_ppdu, meaning unknown"]
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    #[doc = "0x08 - Config"]
    #[inline(always)]
    pub const fn config(&self) -> &CONFIG {
        &self.config
    }
    #[doc = "0x0c - PLCP0"]
    #[inline(always)]
    pub const fn plcp0(&self) -> &PLCP0 {
        &self.plcp0
    }
}
#[doc = "CONF0 (rw) register accessor: Three 10 bit fields written by mac_tx_set_htsig, bit 31 set by mac_tx_set_txop_q; printed as CONF2 by lmacProcessTxComplete\n\nYou can [`read`](crate::Reg::read) this register and get [`conf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf0`] module"]
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
#[doc = "Three 10 bit fields written by mac_tx_set_htsig, bit 31 set by mac_tx_set_txop_q; printed as CONF2 by lmacProcessTxComplete"]
pub mod conf0;
#[doc = "CONF1 (rw) register accessor: Bit 3 is cleared by hal_mac_tx_set_ppdu, meaning unknown\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf1`] module"]
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
#[doc = "Bit 3 is cleared by hal_mac_tx_set_ppdu, meaning unknown"]
pub mod conf1;
#[doc = "CONFIG (rw) register accessor: Config\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Config"]
pub mod config;
#[doc = "PLCP0 (rw) register accessor: PLCP0\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp0`] module"]
pub type PLCP0 = crate::Reg<plcp0::PLCP0_SPEC>;
#[doc = "PLCP0"]
pub mod plcp0;
