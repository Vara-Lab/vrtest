use gprimitives::ActorId;
use primitive_types::H256;
use parity_scale_codec::{Encode, Decode, MaxEncodedLen, Input, Output, Error};
use primitive_types::U256;
use scale_info::TypeInfo;
use common::Origin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Encode, Decode, TypeInfo, MaxEncodedLen)]
pub struct ActorId32([u8; 32]);

impl ActorId32 {
    pub fn into_actor_id(self) -> ActorId {
        // gprimitives::ActorId tiene From<[u8;32]> (derive_more::From)
        ActorId::from(self.0)
    }
    pub fn as_bytes(&self) -> &[u8;32] { &self.0 }
}

impl From<[u8; 32]> for ActorId32 {
    fn from(b: [u8;32]) -> Self { Self(b) }
}
// impl From<u64> for ActorId32 {
//     // fn from(value: u64) -> Self {
//     //     let mut id = Self::zero();
//     //     id.0[12..20].copy_from_slice(&value.to_le_bytes()[..]);
//     //     id
//     // }

//     fn from(value: u64) -> Self {
//         let mut id = [0u8; 32];
//         // b[..8].copy_from_slice(&x.to_le_bytes());
//         id[12..20].copy_from_slice(&value.to_le_bytes()[..]);
//         Self(id)
//     }
// }

impl From<u64> for ActorId32 {
    #[inline]
    fn from(x: u64) -> Self {
        // usa EXACTAMENTE el mismo mapping del runtime
        let aid: ActorId = x.into_origin().into();
        ActorId32::from(aid)
    }
}


impl From<U256> for ActorId32 {
    fn from(x: U256) -> Self {
        let mut b = [0u8; 32];
        x.to_little_endian(&mut b);
        Self(b)
    }
}

impl From<H256> for ActorId32 {
    fn from(value: H256) -> Self {
        Self(value.to_fixed_bytes())
    }
}

impl From<ActorId> for ActorId32 {
    #[inline]
    fn from(a: ActorId) -> Self {
        let arr: [u8; 32] = a.into(); // consume ActorId => [u8;32]
        ActorId32(arr)
    }
}

/* SCALE: codifica/decodifica como 32 bytes crudos (igual que ActorId) */
// impl Encode for ActorId32 {
//     fn size_hint(&self) -> usize { 32 }
//     fn encode_to<T: Output>(&self, dest: &mut T) {
//         self.0.encode_to(dest);
//     }
// }

// impl Decode for ActorId32 {
//     fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
//         let arr: [u8;32] = Decode::decode(input)?;
//         Ok(Self(arr))
//     }
// }

/* (Opcional) permitir usarlo donde pidan ActorId vía Into */
impl From<ActorId32> for ActorId {
    fn from(a: ActorId32) -> Self { ActorId::from(a.0) }
}


