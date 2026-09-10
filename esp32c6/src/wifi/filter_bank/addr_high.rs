#[doc = "Register `ADDR_HIGH%s` reader"]
pub type R = crate::R<ADDR_HIGH_SPEC>;
#[doc = "Register `ADDR_HIGH%s` writer"]
pub type W = crate::W<ADDR_HIGH_SPEC>;
#[doc = "Field `ADDR` reader - "]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - "]
pub type ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RA_ENABLED` reader - Enables the filter in the RA bank (set by hal_mac_set_addr)"]
pub type RA_ENABLED_R = crate::BitReader;
#[doc = "Field `RA_ENABLED` writer - Enables the filter in the RA bank (set by hal_mac_set_addr)"]
pub type RA_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSSID_ENABLED` reader - Enables the filter in the BSSID bank (set by hal_mac_set_bssid)"]
pub type BSSID_ENABLED_R = crate::BitReader;
#[doc = "Field `BSSID_ENABLED` writer - Enables the filter in the BSSID bank (set by hal_mac_set_bssid)"]
pub type BSSID_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enables the filter in the RA bank (set by hal_mac_set_addr)"]
    #[inline(always)]
    pub fn ra_enabled(&self) -> RA_ENABLED_R {
        RA_ENABLED_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the filter in the BSSID bank (set by hal_mac_set_bssid)"]
    #[inline(always)]
    pub fn bssid_enabled(&self) -> BSSID_ENABLED_R {
        BSSID_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_HIGH")
            .field("addr", &self.addr())
            .field("ra_enabled", &self.ra_enabled())
            .field("bssid_enabled", &self.bssid_enabled())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<'_, ADDR_HIGH_SPEC> {
        ADDR_W::new(self, 0)
    }
    #[doc = "Bit 16 - Enables the filter in the RA bank (set by hal_mac_set_addr)"]
    #[inline(always)]
    pub fn ra_enabled(&mut self) -> RA_ENABLED_W<'_, ADDR_HIGH_SPEC> {
        RA_ENABLED_W::new(self, 16)
    }
    #[doc = "Bit 31 - Enables the filter in the BSSID bank (set by hal_mac_set_bssid)"]
    #[inline(always)]
    pub fn bssid_enabled(&mut self) -> BSSID_ENABLED_W<'_, ADDR_HIGH_SPEC> {
        BSSID_ENABLED_W::new(self, 31)
    }
}
#[doc = "Last 2 bytes of the MAC address filter and its enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_HIGH_SPEC;
impl crate::RegisterSpec for ADDR_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_high::R`](R) reader structure"]
impl crate::Readable for ADDR_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_high::W`](W) writer structure"]
impl crate::Writable for ADDR_HIGH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR_HIGH%s to value 0"]
impl crate::Resettable for ADDR_HIGH_SPEC {}
