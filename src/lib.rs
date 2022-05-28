#![deny(clippy::all)]

#[macro_use]
extern crate lazy_static;
use napi_derive::*;
use std::sync::Mutex;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

static ID_SIZE: usize = 21;
static BUFFER_SIZE: usize = (2 << 8) * ID_SIZE;
static ALPHABET: [char; 64] = nanoid::alphabet::SAFE;
static MASK: usize = ALPHABET.len().next_power_of_two() - 1;

lazy_static! {
  static ref BUFFER: Mutex<Vec<u8>> = Mutex::new(nanoid::rngs::default(BUFFER_SIZE));
  static ref POINTER: Mutex<usize> = Mutex::new(0);
}

fn format(random: fn(usize) -> Vec<u8>) -> String {
  assert!(
    ALPHABET.len() <= u8::max_value() as usize,
    "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
  );

  // Assert that the masking does not truncate the alphabet. (See #9)
  debug_assert!(ALPHABET.len() <= MASK + 1);

  let mut id = String::with_capacity(ID_SIZE);

  let start = *POINTER.lock().unwrap();
  let end = start + ID_SIZE;

  let bytes = &mut BUFFER.lock().unwrap();

  for i in start..end {
    let byte = bytes[i] as usize & MASK;
    id.push(ALPHABET[byte]);
  }

  if (end + ID_SIZE) <= BUFFER_SIZE {
    *POINTER.lock().unwrap() += ID_SIZE;
  } else {
    *POINTER.lock().unwrap() = 0;
    let buf = random(BUFFER_SIZE);
    for i in 0..BUFFER_SIZE {
      bytes[i] = buf[i];
    }
  }

  id
}

#[napi]
pub fn nanoid() -> String {
  format(nanoid::rngs::default)
}

#[napi]
pub fn nanoid_non_secure() -> String {
  format(nanoid::rngs::non_secure)
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
