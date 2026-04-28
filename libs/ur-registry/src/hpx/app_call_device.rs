use crate::error::{URError};
use crate::registry_types::RegistryType;
use crate::registry_types::HPX_APP_CALL_DEVICE;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

use crate::traits::{RegistryItem};
use crate::types::Bytes;

#[derive(Debug, Clone, Default)]
pub struct HpxAppCallDevice(Bytes);

impl HpxAppCallDevice {
    pub fn new(bytes: Bytes) -> Self {
        Self(bytes)
    }

    pub fn get_bytes(&self) -> Bytes {
        self.0.clone()
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn set_bytes(&mut self, bytes: Bytes) {
        self.0 = bytes;
    }
}

impl RegistryItem for HpxAppCallDevice {
    fn get_registry_type() -> RegistryType<'static> {
        HPX_APP_CALL_DEVICE
    }
}

impl<C> minicbor::Encode<C> for HpxAppCallDevice {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.0)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for HpxAppCallDevice {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        Ok(Self(d.input().to_vec()))
    }
}

impl TryFrom<Vec<u8>> for HpxAppCallDevice {
    type Error = URError;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(bytes))
    }
}

#[cfg(test)]
mod tests {
    use crate::hpx::app_call_device::HpxAppCallDevice;
    use crate::traits::{From as FromCbor};
    use alloc::string::String;
    extern crate std;
    #[test]
    fn test_decode() {
        let part =
            "ur:hpx-app-call-device/kgcpksiyjocpftcpetiaeyeneyeteneccpdwcpjnihjyisjliecpftcpkoihjpiniykkfpieiejpihjkjkcpdwcpjpihjskpihjkjygaiecpftcpetfgeefxetemfgfgehehememeefxehfgesenfpesesfgfwendyesetecdyececescpdwcpjohsjphsjnjkcpfthpkgcpiaishsinjtcpftcpfejyisihjpihkpjncpdwcpjohsjyiscpftcpjnhhdleeeedihhdlendydihhdldydihhdldyhhdldycpdwcpiaishsinjtgaiecpftcpehcpkihlkinsuylaba";
        let decode_data = ur::decode(part);
        let crypto = HpxAppCallDevice::try_from(decode_data.unwrap().1).unwrap();
        let s = String::from_utf8(crypto.0).unwrap();
        assert!(!s.is_empty())
    }
}
