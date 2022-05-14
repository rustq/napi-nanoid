#![deny(clippy::all)]

use napi_derive::*;
use random_fast_rng::{FastRng, Random};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn default_random(size: usize) -> Vec<u8> {
  let mut fast_rng = FastRng::new();
  let mut result: Vec<u8> = vec![0; size];

  fast_rng.fill_bytes(&mut result[..]);
  result
}

#[napi]
pub fn nanoid() -> String {
  nanoid::format(default_random, &nanoid::alphabet::SAFE, 21)
}

/* custom method won't be added into 0.0.1 yet until the napi case be resolved */

/*

#[napi]
pub fn custom_size(_size: Option<u32>) -> String {
  let size = if let Some(size) = _size {
    size as usize
  } else {
    21
  };

  nanoid::format(default_random, &nanoid::alphabet::SAFE, size)
}

#[napi]
pub fn custom_alphabet(_size: Option<u32>, _alphabet: Option<&str>) -> String {
  let size = if let Some(size) = _size {
    size as usize
  } else {
    21
  };

  let mut custom_alphabet = vec![];
  let alphabet = if let Some(alp) = _alphabet {
    custom_alphabet = alp.to_string().chars().collect::<Vec<_>>();
    &custom_alphabet[..]
  } else {
    &nanoid::alphabet::SAFE
  };

  nanoid::format(default_random, alphabet, size)
}

*/
