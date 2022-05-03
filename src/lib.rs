#![deny(clippy::all)]

#[warn(unused_imports)]
use napi_derive::napi;
use random_fast_rng::{FastRng, Random};

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn random(size: usize) -> Vec<u8> {
  let mut fast_rng = FastRng::new();
  let mut result: Vec<u8> = vec![0; size];

  fast_rng.fill_bytes(&mut result[..]);
  result
}

#[napi_derive::napi]
pub fn nanoid() -> String {
  nanoid::format(random, &nanoid::alphabet::SAFE, 21)
}
