#![deny(clippy::all)]

use napi_derive::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

const POOL_SIZE_MULTIPLIER: usize = 128;
const DEFAULT_SIZE: usize = 21;
const POOL_SIZE: usize = DEFAULT_SIZE * POOL_SIZE_MULTIPLIER;

static POOL: Lazy<Mutex<[u8; POOL_SIZE]>> = Lazy::new(|| Mutex::new([0; POOL_SIZE]));
static POOL_OFFSET: Lazy<Mutex<usize>> = Lazy::new(|| Mutex::new(POOL_SIZE));

fn format(random: fn(usize) -> Vec<u8>, alphabet: &[char], size: usize) -> String {
  assert!(
    alphabet.len() <= u8::max_value() as usize,
    "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
  );

  assert!(size <= POOL_SIZE, "The size should smaller than pool size");

  let bytes = &mut POOL.lock().unwrap();
  let mask = alphabet.len().next_power_of_two() - 1;
  // Assert that the masking does not truncate the alphabet. (See #9)
  debug_assert!(alphabet.len() <= mask + 1);

  let mut pointer = *POOL_OFFSET.lock().unwrap();

  let mut id = String::with_capacity(size);

  while id.len() < size {
    if pointer == POOL_SIZE {
      let buf = random(POOL_SIZE);
      for i in 0..POOL_SIZE {
        bytes[i] = buf[i];
      }
      pointer = 0;
    }
    let byte = bytes[pointer] as usize & mask;
    if alphabet.len() > byte {
      id.push(alphabet[byte]);
    }
    pointer += 1;
  }

  *POOL_OFFSET.lock().unwrap() = pointer;

  id
}

#[napi]
pub fn nanoid() -> String {
  format(nanoid::rngs::default, &nanoid::alphabet::SAFE, 21)
}

#[napi]
pub fn non_secure() -> String {
  format(nanoid::rngs::non_secure, &nanoid::alphabet::SAFE, 21)
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
