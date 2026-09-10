#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_rx_ctrl: [u8; 0xb8],
    _reserved1: [u8; 0x20],
    filter_control: [FILTER_CONTROL; 4],
    _reserved2: [u8; 0x10],
    rx_ctrl_filter: [RX_CTRL_FILTER; 4],
    _reserved3: [u8; 0x06f8],
    crypto_control: CRYPTO_CONTROL,
    _reserved4: [u8; 0x0430],
    mac_interrupt: MAC_INTERRUPT,
    _reserved5: [u8; 0x58],
    ctrl: CTRL,
    txq_state: TXQ_STATE,
    _reserved7: [u8; 0x64],
    tx_slot_config: [TX_SLOT_CONFIG; 5],
    _reserved8: [u8; 0x0548],
    plcp1: (),
    _reserved9: [u8; 0x04],
    tx_threshold: (),
    _reserved10: [u8; 0x04],
    tx_pti: (),
    _reserved11: [u8; 0x04],
    ht_sig: (),
    _reserved12: [u8; 0x18],
    tx_pwr: (),
    _reserved13: [u8; 0x04],
    ht_unknown: (),
    _reserved14: [u8; 0x08],
    tx_len: (),
    _reserved15: [u8; 0x04],
    plcp2: (),
    _reserved16: [u8; 0x04],
    duration: (),
    _reserved17: [u8; 0x28],
    pmd: (),
    _reserved18: [u8; 0x04e8],
    crypto_key_slot: [CRYPTO_KEY_SLOT; 25],
    _reserved19: [u8; 0x7418],
    mac_time: MAC_TIME,
    _reserved20: [u8; 0x10],
    tsf_ctrl: TSF_CTRL,
    tsf_load_low: TSF_LOAD_LOW,
    tsf_load_high: TSF_LOAD_HIGH,
    tsf_time_low: TSF_TIME_LOW,
    tsf_time_high: TSF_TIME_HIGH,
    _reserved25: [u8; 0x04],
    tbtt_start: TBTT_START,
    _reserved26: [u8; 0x20],
    tsf_cfg: (),
    _reserved27: [u8; 0x04],
    tbtt_cfg: (),
    _reserved28: [u8; 0x20],
    tsf_timer_cfg: (),
    _reserved29: [u8; 0x04],
    tsf_timer_target: (),
    _reserved30: [u8; 0x30],
    pwr_int_enable: PWR_INT_ENABLE,
    _reserved31: [u8; 0x04],
    pwr_interrupt: PWR_INTERRUPT,
}
impl RegisterBlock {
    #[doc = "0x00..0xb8 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces. Unlike the Wi-Fi 4 MAC there are no mask registers, the enable bit lives in ADDR_HIGH."]
    #[inline(always)]
    pub const fn filter_bank(&self, n: usize) -> &FILTER_BANK {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0xb8 - Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces. Unlike the Wi-Fi 4 MAC there are no mask registers, the enable bit lives in ADDR_HIGH."]
    #[inline(always)]
    pub fn filter_bank_iter(&self) -> impl Iterator<Item = &FILTER_BANK> {
        (0..2).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(92 * n).cast() })
    }
    #[doc = "0x80 - Controls the reception of frames"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RX_CTRL {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(128).cast() }
    }
    #[doc = "0x84..0x90 - RX_DMA_LIST"]
    #[inline(always)]
    pub const fn rx_dma_list(&self) -> &RX_DMA_LIST {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(132).cast() }
    }
    #[doc = "0xd8..0xe8 - Controls the RX filter for an interface"]
    #[inline(always)]
    pub const fn filter_control(&self, n: usize) -> &FILTER_CONTROL {
        &self.filter_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd8..0xe8 - Controls the RX filter for an interface"]
    #[inline(always)]
    pub fn filter_control_iter(&self) -> impl Iterator<Item = &FILTER_CONTROL> {
        self.filter_control.iter()
    }
    #[doc = "0xf8..0x108 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub const fn rx_ctrl_filter(&self, n: usize) -> &RX_CTRL_FILTER {
        &self.rx_ctrl_filter[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf8..0x108 - Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
    #[inline(always)]
    pub fn rx_ctrl_filter_iter(&self) -> impl Iterator<Item = &RX_CTRL_FILTER> {
        self.rx_ctrl_filter.iter()
    }
    #[doc = "0x800..0x818 - Control registers for hardware crypto"]
    #[inline(always)]
    pub const fn crypto_control(&self) -> &CRYPTO_CONTROL {
        &self.crypto_control
    }
    #[doc = "0xc48..0xc50 - Status and clear for the WIFI_MAC interrupt"]
    #[inline(always)]
    pub const fn mac_interrupt(&self) -> &MAC_INTERRUPT {
        &self.mac_interrupt
    }
    #[doc = "0xca8 - Exact name and meaning unknown, used for initializing the MAC"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0xcac..0xcbc - State of transmission queues"]
    #[inline(always)]
    pub const fn txq_state(&self) -> &TXQ_STATE {
        &self.txq_state
    }
    #[doc = "0xd20..0xd70 - Used to configure the TX slot."]
    #[inline(always)]
    pub const fn tx_slot_config(&self, n: usize) -> &TX_SLOT_CONFIG {
        &self.tx_slot_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd20..0xd70 - Used to configure the TX slot."]
    #[inline(always)]
    pub fn tx_slot_config_iter(&self) -> impl Iterator<Item = &TX_SLOT_CONFIG> {
        self.tx_slot_config.iter()
    }
    #[doc = "0x12b8..0x12cc - PLCP1"]
    #[inline(always)]
    pub const fn plcp1(&self, n: usize) -> &PLCP1 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4792)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12b8..0x12cc - PLCP1"]
    #[inline(always)]
    pub fn plcp1_iter(&self) -> impl Iterator<Item = &PLCP1> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4792)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12bc..0x12d0 - Printed as \"Thres\" by lmacProcessTxComplete, presumably the RTS threshold of the slot"]
    #[inline(always)]
    pub const fn tx_threshold(&self, n: usize) -> &TX_THRESHOLD {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4796)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12bc..0x12d0 - Printed as \"Thres\" by lmacProcessTxComplete, presumably the RTS threshold of the slot"]
    #[inline(always)]
    pub fn tx_threshold_iter(&self) -> impl Iterator<Item = &TX_THRESHOLD> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4796)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12c0..0x12d4 - Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
    #[inline(always)]
    pub const fn tx_pti(&self, n: usize) -> &TX_PTI {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4800)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c0..0x12d4 - Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
    #[inline(always)]
    pub fn tx_pti_iter(&self) -> impl Iterator<Item = &TX_PTI> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4800)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12c4..0x12d8 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub const fn ht_sig(&self, n: usize) -> &HT_SIG {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4804)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12c4..0x12d8 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub fn ht_sig_iter(&self) -> impl Iterator<Item = &HT_SIG> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4804)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12dc..0x12f0 - TX power per rate, one byte each for the data frame, its alternative, the RTS and its alternative. Written by hal_mac_tx_set_ppdu from the table hal_init_tx_pwr fills."]
    #[inline(always)]
    pub const fn tx_pwr(&self, n: usize) -> &TX_PWR {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4828)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12dc..0x12f0 - TX power per rate, one byte each for the data frame, its alternative, the RTS and its alternative. Written by hal_mac_tx_set_ppdu from the table hal_init_tx_pwr fills."]
    #[inline(always)]
    pub fn tx_pwr_iter(&self) -> impl Iterator<Item = &TX_PWR> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4828)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12e0..0x12f4 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub const fn ht_unknown(&self, n: usize) -> &HT_UNKNOWN {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4832)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12e0..0x12f4 - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub fn ht_unknown_iter(&self) -> impl Iterator<Item = &HT_UNKNOWN> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4832)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12e8..0x12fc - PPDU length (bits 0..13), bandwidth (bits 22..23) and rate index (bits 28..31) written by mac_tx_set_len for HT and HE frames"]
    #[inline(always)]
    pub const fn tx_len(&self, n: usize) -> &TX_LEN {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4840)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12e8..0x12fc - PPDU length (bits 0..13), bandwidth (bits 22..23) and rate index (bits 28..31) written by mac_tx_set_len for HT and HE frames"]
    #[inline(always)]
    pub fn tx_len_iter(&self) -> impl Iterator<Item = &TX_LEN> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4840)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12ec..0x1300 - PLCP2"]
    #[inline(always)]
    pub const fn plcp2(&self, n: usize) -> &PLCP2 {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4844)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12ec..0x1300 - PLCP2"]
    #[inline(always)]
    pub fn plcp2_iter(&self) -> impl Iterator<Item = &PLCP2> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4844)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x12f0..0x1304 - duration of the frame exchange"]
    #[inline(always)]
    pub const fn duration(&self, n: usize) -> &DURATION {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4848)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x12f0..0x1304 - duration of the frame exchange"]
    #[inline(always)]
    pub fn duration_iter(&self) -> impl Iterator<Item = &DURATION> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4848)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x1318..0x132c - TX result of the slot, read by hal_mac_get_txq_pmd and hal_mac_get_txq_complete"]
    #[inline(always)]
    pub const fn pmd(&self, n: usize) -> &PMD {
        #[allow(clippy::no_effect)]
        [(); 5][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4888)
                .add(116 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1318..0x132c - TX result of the slot, read by hal_mac_get_txq_pmd and hal_mac_get_txq_complete"]
    #[inline(always)]
    pub fn pmd_iter(&self) -> impl Iterator<Item = &PMD> {
        (0..5).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4888)
                .add(116 * n)
                .cast()
        })
    }
    #[doc = "0x1800..0x1be8 - Cryptographic keys for MPDU encapsulation and decapsulation"]
    #[inline(always)]
    pub const fn crypto_key_slot(&self, n: usize) -> &CRYPTO_KEY_SLOT {
        &self.crypto_key_slot[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x1be8 - Cryptographic keys for MPDU encapsulation and decapsulation"]
    #[inline(always)]
    pub fn crypto_key_slot_iter(&self) -> impl Iterator<Item = &CRYPTO_KEY_SLOT> {
        self.crypto_key_slot.iter()
    }
    #[doc = "0x9000 - Current value of the MAC timer"]
    #[inline(always)]
    pub const fn mac_time(&self) -> &MAC_TIME {
        &self.mac_time
    }
    #[doc = "0x9014 - Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time."]
    #[inline(always)]
    pub const fn tsf_ctrl(&self) -> &TSF_CTRL {
        &self.tsf_ctrl
    }
    #[doc = "0x9018 - Low word of the value loaded into a TSF counter"]
    #[inline(always)]
    pub const fn tsf_load_low(&self) -> &TSF_LOAD_LOW {
        &self.tsf_load_low
    }
    #[doc = "0x901c - High word of the value loaded into a TSF counter"]
    #[inline(always)]
    pub const fn tsf_load_high(&self) -> &TSF_LOAD_HIGH {
        &self.tsf_load_high
    }
    #[doc = "0x9020 - Low word of the latched TSF counter"]
    #[inline(always)]
    pub const fn tsf_time_low(&self) -> &TSF_TIME_LOW {
        &self.tsf_time_low
    }
    #[doc = "0x9024 - High word of the latched TSF counter"]
    #[inline(always)]
    pub const fn tsf_time_high(&self) -> &TSF_TIME_HIGH {
        &self.tsf_time_high
    }
    #[doc = "0x902c - TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START"]
    #[inline(always)]
    pub const fn tbtt_start(&self) -> &TBTT_START {
        &self.tbtt_start
    }
    #[doc = "0x9050..0x9060 - TSF and TBTT configuration of an interface"]
    #[inline(always)]
    pub const fn tsf_cfg(&self, n: usize) -> &TSF_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36944)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9050..0x9060 - TSF and TBTT configuration of an interface"]
    #[inline(always)]
    pub fn tsf_cfg_iter(&self) -> impl Iterator<Item = &TSF_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36944)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x9054..0x9064 - TBTT interval and early time of an interface"]
    #[inline(always)]
    pub const fn tbtt_cfg(&self, n: usize) -> &TBTT_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36948)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9054..0x9064 - TBTT interval and early time of an interface"]
    #[inline(always)]
    pub fn tbtt_cfg_iter(&self) -> impl Iterator<Item = &TBTT_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36948)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x9074..0x9084 - Configuration of a TSF timer"]
    #[inline(always)]
    pub const fn tsf_timer_cfg(&self, n: usize) -> &TSF_TIMER_CFG {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36980)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9074..0x9084 - Configuration of a TSF timer"]
    #[inline(always)]
    pub fn tsf_timer_cfg_iter(&self) -> impl Iterator<Item = &TSF_TIMER_CFG> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36980)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x9078..0x9088 - Target of a TSF timer, written by tsf_hal_set_timer_target"]
    #[inline(always)]
    pub const fn tsf_timer_target(&self, n: usize) -> &TSF_TIMER_TARGET {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36984)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9078..0x9088 - Target of a TSF timer, written by tsf_hal_set_timer_target"]
    #[inline(always)]
    pub fn tsf_timer_target_iter(&self) -> impl Iterator<Item = &TSF_TIMER_TARGET> {
        (0..4).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(36984)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x90a8 - Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (3 - n), TSF timer n is bit (7 - n)."]
    #[inline(always)]
    pub const fn pwr_int_enable(&self) -> &PWR_INT_ENABLE {
        &self.pwr_int_enable
    }
    #[doc = "0x90b0..0x90b8 - Status and clear for the WIFI_PWR interrupt"]
    #[inline(always)]
    pub const fn pwr_interrupt(&self) -> &PWR_INTERRUPT {
        &self.pwr_interrupt
    }
}
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces. Unlike the Wi-Fi 4 MAC there are no mask registers, the enable bit lives in ADDR_HIGH."]
pub use self::filter_bank::FILTER_BANK;
#[doc = r"Cluster"]
#[doc = "Filter banks for frame reception. Bank zero is for the BSSID and bank one for the RA. Each filter bank has registers for four interfaces. Unlike the Wi-Fi 4 MAC there are no mask registers, the enable bit lives in ADDR_HIGH."]
pub mod filter_bank;
#[doc = "RX_CTRL (rw) register accessor: Controls the reception of frames\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`] module"]
pub type RX_CTRL = crate::Reg<rx_ctrl::RX_CTRL_SPEC>;
#[doc = "Controls the reception of frames"]
pub mod rx_ctrl;
#[doc = "RX_DMA_LIST"]
pub use self::rx_dma_list::RX_DMA_LIST;
#[doc = r"Cluster"]
#[doc = "RX_DMA_LIST"]
pub mod rx_dma_list;
#[doc = "FILTER_CONTROL (rw) register accessor: Controls the RX filter for an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@filter_control`] module"]
pub type FILTER_CONTROL = crate::Reg<filter_control::FILTER_CONTROL_SPEC>;
#[doc = "Controls the RX filter for an interface"]
pub mod filter_control;
#[doc = "RX_CTRL_FILTER (rw) register accessor: Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl_filter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl_filter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl_filter`] module"]
pub type RX_CTRL_FILTER = crate::Reg<rx_ctrl_filter::RX_CTRL_FILTER_SPEC>;
#[doc = "Configures which control frames pass the RX filter. Setting a bit lets that frame type pass the filter."]
pub mod rx_ctrl_filter;
#[doc = "Control registers for hardware crypto"]
pub use self::crypto_control::CRYPTO_CONTROL;
#[doc = r"Cluster"]
#[doc = "Control registers for hardware crypto"]
pub mod crypto_control;
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub use self::mac_interrupt::MAC_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_MAC interrupt"]
pub mod mac_interrupt;
#[doc = "CTRL (rw) register accessor: Exact name and meaning unknown, used for initializing the MAC\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Exact name and meaning unknown, used for initializing the MAC"]
pub mod ctrl;
#[doc = "State of transmission queues"]
pub use self::txq_state::TXQ_STATE;
#[doc = r"Cluster"]
#[doc = "State of transmission queues"]
pub mod txq_state;
#[doc = "Used to configure the TX slot."]
pub use self::tx_slot_config::TX_SLOT_CONFIG;
#[doc = r"Cluster"]
#[doc = "Used to configure the TX slot."]
pub mod tx_slot_config;
#[doc = "PLCP1 (rw) register accessor: PLCP1\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp1`] module"]
pub type PLCP1 = crate::Reg<plcp1::PLCP1_SPEC>;
#[doc = "PLCP1"]
pub mod plcp1;
#[doc = "TX_THRESHOLD (rw) register accessor: Printed as \"Thres\" by lmacProcessTxComplete, presumably the RTS threshold of the slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_threshold`] module"]
pub type TX_THRESHOLD = crate::Reg<tx_threshold::TX_THRESHOLD_SPEC>;
#[doc = "Printed as \"Thres\" by lmacProcessTxComplete, presumably the RTS threshold of the slot"]
pub mod tx_threshold;
#[doc = "TX_PTI (rw) register accessor: Coexistence priorities of the TX slot, written by hal_set_tx_pti\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pti`] module"]
pub type TX_PTI = crate::Reg<tx_pti::TX_PTI_SPEC>;
#[doc = "Coexistence priorities of the TX slot, written by hal_set_tx_pti"]
pub mod tx_pti;
#[doc = "HT_SIG (rw) register accessor: HT-SIG field in HT preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_sig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_sig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_sig`] module"]
pub type HT_SIG = crate::Reg<ht_sig::HT_SIG_SPEC>;
#[doc = "HT-SIG field in HT preamble"]
pub mod ht_sig;
#[doc = "TX_PWR (rw) register accessor: TX power per rate, one byte each for the data frame, its alternative, the RTS and its alternative. Written by hal_mac_tx_set_ppdu from the table hal_init_tx_pwr fills.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_pwr`] module"]
pub type TX_PWR = crate::Reg<tx_pwr::TX_PWR_SPEC>;
#[doc = "TX power per rate, one byte each for the data frame, its alternative, the RTS and its alternative. Written by hal_mac_tx_set_ppdu from the table hal_init_tx_pwr fills."]
pub mod tx_pwr;
#[doc = "HT_UNKNOWN (rw) register accessor: exact meaning and name unknown, related to HT\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_unknown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_unknown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_unknown`] module"]
pub type HT_UNKNOWN = crate::Reg<ht_unknown::HT_UNKNOWN_SPEC>;
#[doc = "exact meaning and name unknown, related to HT"]
pub mod ht_unknown;
#[doc = "TX_LEN (rw) register accessor: PPDU length (bits 0..13), bandwidth (bits 22..23) and rate index (bits 28..31) written by mac_tx_set_len for HT and HE frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_len::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_len::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_len`] module"]
pub type TX_LEN = crate::Reg<tx_len::TX_LEN_SPEC>;
#[doc = "PPDU length (bits 0..13), bandwidth (bits 22..23) and rate index (bits 28..31) written by mac_tx_set_len for HT and HE frames"]
pub mod tx_len;
#[doc = "PLCP2 (rw) register accessor: PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp2`] module"]
pub type PLCP2 = crate::Reg<plcp2::PLCP2_SPEC>;
#[doc = "PLCP2"]
pub mod plcp2;
#[doc = "DURATION (rw) register accessor: duration of the frame exchange\n\nYou can [`read`](crate::Reg::read) this register and get [`duration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duration`] module"]
pub type DURATION = crate::Reg<duration::DURATION_SPEC>;
#[doc = "duration of the frame exchange"]
pub mod duration;
#[doc = "PMD (rw) register accessor: TX result of the slot, read by hal_mac_get_txq_pmd and hal_mac_get_txq_complete\n\nYou can [`read`](crate::Reg::read) this register and get [`pmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmd`] module"]
pub type PMD = crate::Reg<pmd::PMD_SPEC>;
#[doc = "TX result of the slot, read by hal_mac_get_txq_pmd and hal_mac_get_txq_complete"]
pub mod pmd;
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub use self::crypto_key_slot::CRYPTO_KEY_SLOT;
#[doc = r"Cluster"]
#[doc = "Cryptographic keys for MPDU encapsulation and decapsulation"]
pub mod crypto_key_slot;
#[doc = "MAC_TIME (rw) register accessor: Current value of the MAC timer\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_time`] module"]
pub type MAC_TIME = crate::Reg<mac_time::MAC_TIME_SPEC>;
#[doc = "Current value of the MAC timer"]
pub mod mac_time;
#[doc = "TSF_CTRL (rw) register accessor: Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_ctrl`] module"]
pub type TSF_CTRL = crate::Reg<tsf_ctrl::TSF_CTRL_SPEC>;
#[doc = "Control for the per-interface TSF counters. Written by tsf_hal_get_counter_value, tsf_hal_set_counter_value and tsf_hal_set_tbtt_start_time."]
pub mod tsf_ctrl;
#[doc = "TSF_LOAD_LOW (rw) register accessor: Low word of the value loaded into a TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_load_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_load_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_load_low`] module"]
pub type TSF_LOAD_LOW = crate::Reg<tsf_load_low::TSF_LOAD_LOW_SPEC>;
#[doc = "Low word of the value loaded into a TSF counter"]
pub mod tsf_load_low;
#[doc = "TSF_LOAD_HIGH (rw) register accessor: High word of the value loaded into a TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_load_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_load_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_load_high`] module"]
pub type TSF_LOAD_HIGH = crate::Reg<tsf_load_high::TSF_LOAD_HIGH_SPEC>;
#[doc = "High word of the value loaded into a TSF counter"]
pub mod tsf_load_high;
#[doc = "TSF_TIME_LOW (rw) register accessor: Low word of the latched TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_time_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_time_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_time_low`] module"]
pub type TSF_TIME_LOW = crate::Reg<tsf_time_low::TSF_TIME_LOW_SPEC>;
#[doc = "Low word of the latched TSF counter"]
pub mod tsf_time_low;
#[doc = "TSF_TIME_HIGH (rw) register accessor: High word of the latched TSF counter\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_time_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_time_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_time_high`] module"]
pub type TSF_TIME_HIGH = crate::Reg<tsf_time_high::TSF_TIME_HIGH_SPEC>;
#[doc = "High word of the latched TSF counter"]
pub mod tsf_time_high;
#[doc = "TBTT_START (rw) register accessor: TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbtt_start`] module"]
pub type TBTT_START = crate::Reg<tbtt_start::TBTT_START_SPEC>;
#[doc = "TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START"]
pub mod tbtt_start;
#[doc = "TSF_CFG (rw) register accessor: TSF and TBTT configuration of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_cfg`] module"]
pub type TSF_CFG = crate::Reg<tsf_cfg::TSF_CFG_SPEC>;
#[doc = "TSF and TBTT configuration of an interface"]
pub mod tsf_cfg;
#[doc = "TBTT_CFG (rw) register accessor: TBTT interval and early time of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbtt_cfg`] module"]
pub type TBTT_CFG = crate::Reg<tbtt_cfg::TBTT_CFG_SPEC>;
#[doc = "TBTT interval and early time of an interface"]
pub mod tbtt_cfg;
#[doc = "TSF_TIMER_CFG (rw) register accessor: Configuration of a TSF timer\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_timer_cfg`] module"]
pub type TSF_TIMER_CFG = crate::Reg<tsf_timer_cfg::TSF_TIMER_CFG_SPEC>;
#[doc = "Configuration of a TSF timer"]
pub mod tsf_timer_cfg;
#[doc = "TSF_TIMER_TARGET (rw) register accessor: Target of a TSF timer, written by tsf_hal_set_timer_target\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_timer_target::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_timer_target::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsf_timer_target`] module"]
pub type TSF_TIMER_TARGET = crate::Reg<tsf_timer_target::TSF_TIMER_TARGET_SPEC>;
#[doc = "Target of a TSF timer, written by tsf_hal_set_timer_target"]
pub mod tsf_timer_target;
#[doc = "PWR_INT_ENABLE (rw) register accessor: Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (3 - n), TSF timer n is bit (7 - n).\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_int_enable`] module"]
pub type PWR_INT_ENABLE = crate::Reg<pwr_int_enable::PWR_INT_ENABLE_SPEC>;
#[doc = "Interrupt enable for the WIFI_PWR interrupt. TBTT of interface n is bit (3 - n), TSF timer n is bit (7 - n)."]
pub mod pwr_int_enable;
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub use self::pwr_interrupt::PWR_INTERRUPT;
#[doc = r"Cluster"]
#[doc = "Status and clear for the WIFI_PWR interrupt"]
pub mod pwr_interrupt;
