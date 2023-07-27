use crate::utils::{
    decode_hex,
};

use crate::vmapi::eosio::{
    slice_copy,
    check,
};

use crate::name::{ Name };

use crate::print::{ 
    Printable,
    printui128,
    // printhex,
};

use crate::serializer::{
    Packer,
    Encoder,
    Decoder,
};

use crate::varint::{
    VarUint32,
};

use crate::{
    vec::Vec,
    string::String,
};

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Float128 {
    ///
    pub data: [u8; 16],
}

impl Float128 {
    ///
    pub fn new(data: [u8;16]) -> Self {
        Self {
            data: data
        }
    }

    ///
    pub fn data(&self) -> &[u8; 16] {
        return &self.data;
    }
}

impl Packer for Float128 {
    fn size(&self) -> usize {
        return 16;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        slice_copy(data, &self.data);
        self.size()
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let size = self.size();
        check(raw.len() >= size, "Float128.unpack: buffer overflow!");
        slice_copy(&mut self.data, &raw[..size]);
        return self.size();
    }
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Checksum160 {
    pub data: [u8; 20],
}

impl Checksum160 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 40, "Checksum160: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        slice_copy(&mut ret.data, &data);
        return ret;
    }
}

impl Packer for Checksum160 {
    fn size(&self) -> usize {
        return 20;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        slice_copy(data, &self.data);
        self.size()
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let size = self.size();
        check(raw.len() >= size, "Checksum160.unpack: buffer overflow!");
        slice_copy(&mut self.data, &raw[..size]);
        return size;
    }
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct Checksum256 {
    ///
    pub data: [u8; 32],
}

impl Checksum256 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 64, "Checksum256: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        slice_copy(&mut ret.data, &data);
        return ret;
    }
}

impl Packer for Checksum256 {
    fn size(&self) -> usize {
        return 32;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        slice_copy(data, &self.data);
        self.size()
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let size = self.size();
        check(raw.len() >= size, "Checksum256.unpack: buffer overflow!");
        slice_copy(&mut self.data, &raw[..size]);
        return self.size();
    }
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Checksum512 {
    pub data: [u8; 64],
}

impl Checksum512 {
    ///
    pub fn from_hex(s: &str) -> Self {
        check(s.len() == 128, "Checksum512: bad hex string length");
        let data = decode_hex(s);
        let mut ret = Self::default();
        slice_copy(&mut ret.data, &data);
        return ret;
    }
}

impl Default for Checksum512 {
    ///
    #[inline]
    fn default() -> Self {
        Checksum512{data: [0; 64]}
    }
}

impl Packer for Checksum512 {
    fn size(&self) -> usize {
        return 64;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        slice_copy(data, &self.data);
        self.size()
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let size = self.size();
        check(raw.len() >= size, "Checksum512.unpack: buffer overflow!");
        slice_copy(&mut self.data, &raw[..size]);
        return size;
    }
}


///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ECCPublicKey {
    ///
    pub data: [u8; 33],
}

impl ECCPublicKey {
    ///
    pub fn from_hex(s: &str) -> Self {
        let mut ret = Self::default();
        check(s.len() == 33*2, "ECCPublicKey: bad hex string length");
        let data = decode_hex(s);
        slice_copy(&mut ret.data, &data);
        return ret;
    }
}

impl Default for ECCPublicKey {
    ///
    #[inline]
    fn default() -> Self {
        ECCPublicKey{data: [0; 33]}
    }
}

impl Packer for ECCPublicKey {
    fn size(&self) -> usize {
        return 33;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let data = enc.alloc(self.size());
        slice_copy(data, &self.data);
        self.size()
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let size = self.size();
        check(raw.len() >= size, "EccPublicKey.unpack: buffer overflow!");
        slice_copy(&mut self.data, &raw[..size]);
        return size;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum UserPresence {
    ///
    None = 0,
    ///
    Present = 1,
    ///
    Verified = 2,
}

impl Default for UserPresence {
    fn default() -> Self {
        UserPresence::None
    }
}

impl Packer for UserPresence {
    ///
    fn size(&self) -> usize {
        return 1;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        (*self as u8).pack(enc)
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= 1, "UserPresence.unpack: buffer overflow");
        match data[0] {
            0 => {
                *self = UserPresence::None;
            },
            1 => {
                *self = UserPresence::Present;
            },
            2 => {
                *self = UserPresence::Verified;
            }
            _ => {
                check(false, "not a UserPresence type");
            }
        }
        return 1;
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq, Default)]
pub struct WebAuthNPublicKey {
    ///
    pub key: ECCPublicKey,
    ///
    pub user_presence: UserPresence,
    ///
    pub rpid: String,
}

impl WebAuthNPublicKey {
    ///
    pub fn new(key: ECCPublicKey, user_presence: UserPresence, rpid: String) -> Self {
        Self { key, user_presence, rpid }
    }
}

impl Packer for WebAuthNPublicKey {
    ///
    fn size(&self) -> usize {
        self.key.size() + self.user_presence.size() + self.rpid.size()
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        
        self.key.pack(enc);
        self.user_presence.pack(enc);
        self.rpid.pack(enc);
        
        enc.get_size() - pos
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        check(data.len() >= 33, "WebAuthNPublicKey.unpack: buffer overflow!");
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.key);
        dec.unpack(&mut self.user_presence);
        dec.unpack(&mut self.rpid);
        return dec.get_pos();
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub enum PublicKey {
    ///
    K1(ECCPublicKey),
    ///
    R1(ECCPublicKey),
    ///
    WebAuth(WebAuthNPublicKey),
}

impl Default for PublicKey {
    ///
    #[inline]
    fn default() -> Self {
        PublicKey::K1(ECCPublicKey::default())
    }
}

impl Packer for PublicKey {
    fn size(&self) -> usize {
        match self {
            PublicKey::K1(x) => x.size() + 1,
            PublicKey::R1(x) => x.size() + 1,
            PublicKey::WebAuth(x) => x.size() + 1,
        }
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        match self {
            PublicKey::K1(x) => {
                0u8.pack(enc);
                x.pack(enc);
            }
            PublicKey::R1(x) => {
                1u8.pack(enc);
                x.pack(enc);
            }
            PublicKey::WebAuth(x) => {
                2u8.pack(enc);
                x.pack(enc);
            }
        }
        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= 34, "PublicKey.unpack: buffer overflow!");

        let mut dec = Decoder::new(raw);
        let mut ty: u8 = 0;
        dec.unpack(&mut ty);
        match ty {
            0 => {
                let mut pub_key = ECCPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::K1(pub_key);
            },
            1 => {
                let mut pub_key = ECCPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::R1(pub_key);
            },
            2 => {
                let mut pub_key = WebAuthNPublicKey::default();
                dec.unpack(&mut pub_key);
                *self = PublicKey::WebAuth(pub_key);
            }
            _ => {
                check(false, "invalid public key type");
            }
        }
        return dec.get_pos();
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Clone, Eq, PartialEq)]
pub struct Signature {
    /// Signature type
	ty: u8,
    ///
	data: [u8; 65],
}

impl Signature {
    ///
    pub fn from_hex(s: &str) -> Self {
        let mut ret = Self::default();
        check(s.len() == 65*2, "Signature: bad hex string length");
        let data = decode_hex(s);
        ret.ty = 0;
        slice_copy(&mut ret.data, &data);
        return ret;
    }
}

impl Default for Signature {
    fn default() -> Self {
        Self { ty: 0, data: [0; 65] }
    }
}

impl Packer for Signature {
    ///
    fn size(&self) -> usize {
        return 66;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        self.ty.pack(enc);
        let data = enc.alloc(self.data.len());
        slice_copy(data, &self.data);
        self.size()
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        let size = self.size();
        check(data.len() >= size, "Signature::unpack: buffer overflow");
        self.ty = data[0];
        check(self.ty == 0, "bad signature type");
        slice_copy(&mut self.data, &data[1..size]);
        return self.size();
    }
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uint128 {
    ///
    pub lo: u64,
    ///
    pub hi: u64,
}

impl Default for Uint128 {
    ///
    #[inline]
    fn default() -> Self {
        Self {
            lo: 0,
            hi: 0,
        }
    }
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default)]
pub struct Int128 {
    ///
    pub lo: u64,
    ///
    pub hi: u64,
}

///
#[repr(C, align(8))]
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uint256 {
    ///
    pub data: [u128; 2],
}

impl Uint256 {
    ///
    pub fn new(lo: u128, hi: u128) -> Self {
        Self { data: [lo, hi] }
    }

    ///
    pub fn swap(&self) -> Self {
        Self { data: [self.data[1], self.data[0]] }
    }
}

impl Packer for Uint256 {
    ///
    fn size(&self) -> usize {
        return 32;
    }

    ///
    fn pack(&self, enc: &mut Encoder) -> usize {
        self.data[0].pack(enc);
        self.data[1].pack(enc);
        self.size()
    }

    ///
    fn unpack(&mut self, data: &[u8]) -> usize {
        let mut dec = Decoder::new(data);
        dec.unpack(&mut self.data[0]);
        dec.unpack(&mut self.data[1]);
        return dec.get_pos();
    }
}

impl Printable for Uint256 {
    fn print(&self) {
        if self.data[1] == 0 {
            printui128(self.data[0]);
        } else {
            crate::vmapi::print::printhex(self.data.as_ptr() as *mut u8, 32);
        }
    }
}

impl Default for Uint256 {
    ///
    #[inline]
    fn default() -> Self {
        Self {
            data: [Default::default(); 2],
        }
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default, PartialEq)]
pub struct TimePoint {
    /// elapsed in microseconds
    pub elapsed: u64,
}

impl Packer for TimePoint {
    fn size(&self) -> usize {
        return 8;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        self.elapsed.pack(enc)
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "TimePoint.unpack: buffer overflow!");
        return self.elapsed.unpack(raw);
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct TimePointSec {
    ///
    pub seconds: u32,
}

impl TimePointSec {
    pub fn new(seconds: u32) -> Self{
        Self{ seconds }
    }

    pub fn seconds(&self) -> u32 {
        return self.seconds;
    }
}

impl Packer for TimePointSec {
    fn size(&self) -> usize {
        return 4;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        self.seconds.pack(enc)
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "TimePointSec.unpack: buffer overflow!");
        return self.seconds.unpack(raw);
    }
}

///
#[cfg_attr(feature = "std", derive(eosio_scale_info::TypeInfo))]
#[derive(Copy, Clone, Default)]
pub struct BlockTimeStampType {
    ///
    pub slot: u32,
}

impl Packer for BlockTimeStampType {
    fn size(&self) -> usize {
        return 4;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        self.slot.pack(enc)
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        check(raw.len() >= self.size(), "BlockTimeStampType.unpack: buffer overflow!");
        return self.slot.unpack(raw);
    }
}

///
#[derive(Clone, Eq, PartialEq)]
pub struct ProducerKey {
    ///
    pub producer_name: Name,
    ///
    pub block_signing_key: PublicKey,
}

impl ProducerKey {
    ///
    pub fn new(producer_name: Name, block_signing_key: PublicKey) -> Self {
        ProducerKey {
            producer_name,
            block_signing_key,
        }
    }
}

impl Packer for ProducerKey {
    fn size(&self) -> usize {
        8 + self.block_signing_key.size()
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.producer_name.pack(enc);
        self.block_signing_key.pack(enc);

        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.producer_name);
        dec.unpack(&mut self.block_signing_key);
        return dec.get_pos();
    }
}

impl Default for ProducerKey {
    ///
    #[inline]
    fn default() -> Self {
        ProducerKey { producer_name: Default::default(), block_signing_key: Default::default() }
    }
}

#[derive(Default)]
pub struct KeyWeight {
    pub key: PublicKey,
    pub weight: u16,
}

impl Packer for KeyWeight {
    fn size(&self) -> usize {
        return 2 + self.key.size();
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.key.pack(enc);
        self.weight.pack(enc);

        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.key);
        dec.unpack(&mut self.weight);
        return dec.get_pos();
    }
}

#[derive(Default)]
pub struct BlockSigningAuthorityV0 {
    /**
     * minimum threshold of accumulated weights from component keys that satisfies this authority
     *
     * @brief minimum threshold of accumulated weights from component keys that satisfies this authority
     */
    pub threshold: u32,

    /**
     * component keys and their associated weights
     *
     * @brief component keys and their associated weights
     */
    pub keys: Vec<KeyWeight>,
}

impl Packer for BlockSigningAuthorityV0 {
    fn size(&self) -> usize {
        let mut size = 2 + VarUint32::new(self.keys.len() as u32).size();
        for key in &self.keys {
            size += key.size();
        }
        return size;
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.threshold.pack(enc);
        self.keys.pack(enc);

        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.threshold);
        dec.unpack(&mut self.keys);
        return dec.get_pos();
    }
}

pub enum BlockSigningAuthority {
    V0(BlockSigningAuthorityV0)
}

impl Default for BlockSigningAuthority {
    fn default() -> Self {
        BlockSigningAuthority::V0(Default::default())
    }
}

impl Packer for BlockSigningAuthority {
    fn size(&self) -> usize {
        return 1 + match self {
            BlockSigningAuthority::V0(x) => x.size(),
        };
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();
        0u8.pack(enc);
        match self {
            BlockSigningAuthority::V0(x) => {
                x.pack(enc);
            }
        }
        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        let mut ty = 0u8;
        dec.unpack(&mut ty);
        if ty == 0 {
            let mut v0 = BlockSigningAuthorityV0::default();
            dec.unpack(&mut v0);
            *self = BlockSigningAuthority::V0(v0);
        } else {
            check(false, "bad BlockSigningAuthority type");
        }
        return dec.get_pos();
    }
}


#[derive(Default)]
pub struct ProducerAuthority {

    /**
     * Name of the producer
     *
     * @brief Name of the producer
     */
    pub producer_name: Name,

    /**
     * The block signing authority used by this producer
     */
    pub authority: BlockSigningAuthority,
}

impl Packer for ProducerAuthority {
    fn size(&self) -> usize {
        return self.producer_name.size() + self.authority.size();
    }

    fn pack(&self, enc: &mut Encoder) -> usize {
        let pos = enc.get_size();

        self.producer_name.pack(enc);
        self.authority.pack(enc);

        enc.get_size() - pos
    }

    fn unpack(&mut self, raw: &[u8]) -> usize {
        let mut dec = Decoder::new(raw);
        dec.unpack(&mut self.producer_name);
        dec.unpack(&mut self.authority);
        return dec.get_pos();
    }
}
