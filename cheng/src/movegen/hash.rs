use crate::board::BoardMask;

#[inline]
pub fn magic_hash(magic: u64, occupancy: BoardMask, nbits: u32) -> usize {
    let (garbage, _) = magic.overflowing_mul(u64::from(occupancy));
    let shifted_garbage = garbage >> (64 - nbits);
    shifted_garbage.try_into().unwrap()
}
