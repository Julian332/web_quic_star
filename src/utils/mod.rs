pub mod api_wrapper;
#[cfg(feature = "eth_mode")]
pub mod contracts;
pub mod file;

// pub fn byte_is_zero(buf: &[u8]) -> bool {
//     //safety: align_to
//     let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };
//
//     prefix.iter().all(|&x| x == 0)
//         && suffix.iter().all(|&x| x == 0)
//         && aligned.iter().all(|&x| x == 0)
// }
