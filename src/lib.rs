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

static BUFFER_SIZE: usize = (2 << 8) * 21;
static ID_SIZE: usize = 21;

lazy_static! {
  static ref bytes: Mutex<Vec<u8>> = Mutex::new(secure_random(BUFFER_SIZE));
  static ref pointer: Mutex<usize> = Mutex::new(0);
}

fn secure_random(size: usize) -> Vec<u8> {
  nanoid::rngs::default(size)
}

fn non_secure_random(size: usize) -> Vec<u8> {
  nanoid::rngs::non_secure(size)
}

fn generate(random: fn(usize) -> Vec<u8>) -> String {
  let alphabet = nanoid::alphabet::SAFE;

  assert!(
    alphabet.len() <= u8::max_value() as usize,
    "The alphabet cannot be longer than a `u8` (to comply with the `random` function)"
  );

  let mask = alphabet.len().next_power_of_two() - 1;

  // Assert that the masking does not truncate the alphabet. (See #9)
  debug_assert!(alphabet.len() <= mask + 1);

  let mut id = String::with_capacity(21);

  let start = *pointer.lock().unwrap();
  let end = start + 21;
  let bytes_ref = &mut bytes.lock().unwrap();
  for i in start..end {
    let byte = bytes_ref[i] as usize & mask;

    id.push(alphabet[byte]);
  }

  *pointer.lock().unwrap() += ID_SIZE;

  if (end + 21) > BUFFER_SIZE {
    *pointer.lock().unwrap() = 0;
    let buf = random(BUFFER_SIZE);
    for idx in 0..BUFFER_SIZE {
      bytes_ref[idx] = buf[idx];
    }
  }
  id
}

#[napi]
pub fn nanoid() -> String {
  generate(secure_random)
}

#[napi]
pub fn nanoid_non_secure() -> String {
  generate(non_secure_random)
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
