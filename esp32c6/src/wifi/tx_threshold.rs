#[doc = "Register `TX_THRESHOLD%s` reader"]
pub type R = crate::R<TX_THRESHOLD_SPEC>;
#[doc = "Register `TX_THRESHOLD%s` writer"]
pub type W = crate::W<TX_THRESHOLD_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Printed as \"Thres\" by lmacProcessTxComplete, presumably the RTS threshold of the slot\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_THRESHOLD_SPEC;
impl crate::RegisterSpec for TX_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_threshold::R`](R) reader structure"]
impl crate::Readable for TX_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_threshold::W`](W) writer structure"]
impl crate::Writable for TX_THRESHOLD_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_THRESHOLD%s to value 0"]
impl crate::Resettable for TX_THRESHOLD_SPEC {}
