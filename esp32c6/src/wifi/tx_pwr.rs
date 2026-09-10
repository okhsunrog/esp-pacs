#[doc = "Register `TX_PWR%s` reader"]
pub type R = crate::R<TX_PWR_SPEC>;
#[doc = "Register `TX_PWR%s` writer"]
pub type W = crate::W<TX_PWR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "TX power per rate, one byte each for the data frame, its alternative, the RTS and its alternative. Written by hal_mac_tx_set_ppdu from the table hal_init_tx_pwr fills.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_PWR_SPEC;
impl crate::RegisterSpec for TX_PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_pwr::R`](R) reader structure"]
impl crate::Readable for TX_PWR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_pwr::W`](W) writer structure"]
impl crate::Writable for TX_PWR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_PWR%s to value 0"]
impl crate::Resettable for TX_PWR_SPEC {}
