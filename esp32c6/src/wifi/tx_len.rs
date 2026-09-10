#[doc = "Register `TX_LEN%s` reader"]
pub type R = crate::R<TX_LEN_SPEC>;
#[doc = "Register `TX_LEN%s` writer"]
pub type W = crate::W<TX_LEN_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPDU length (bits 0..13), bandwidth (bits 22..23) and rate index (bits 28..31) written by mac_tx_set_len for HT and HE frames\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_len::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_len::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_LEN_SPEC;
impl crate::RegisterSpec for TX_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_len::R`](R) reader structure"]
impl crate::Readable for TX_LEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_len::W`](W) writer structure"]
impl crate::Writable for TX_LEN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_LEN%s to value 0"]
impl crate::Resettable for TX_LEN_SPEC {}
