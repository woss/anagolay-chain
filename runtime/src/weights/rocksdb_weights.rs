// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

pub mod constants {
  use frame_support::{
    parameter_types,
    weights::{constants, RuntimeDbWeight},
  };

  parameter_types! {
    /// By default, Substrate uses `RocksDB`, so this will be the weight used throughout
    /// the runtime.
    pub const RocksDbWeight: RuntimeDbWeight = RuntimeDbWeight {
      read: 25_000 * constants::WEIGHT_PER_NANOS.ref_time(),
      write: 100_000 * constants::WEIGHT_PER_NANOS.ref_time(),
    };
  }

  #[cfg(test)]
  mod test_db_weights {
    use super::constants::RocksDbWeight as W;
    use frame_support::weights::constants;

    /// Checks that all weights exist and have sane values.
    // NOTE: If this test fails but you are sure that the generated values are fine,
    // you can delete it.
    #[test]
    fn sane() {
      // At least 1 µs.
      assert!(
        W::get().reads(1).ref_time() >= constants::WEIGHT_PER_MICROS.ref_time(),
        "Read weight should be at least 1 µs."
      );
      assert!(
        W::get().writes(1).ref_time() >= constants::WEIGHT_PER_MICROS.ref_time(),
        "Write weight should be at least 1 µs."
      );
      // At most 1 ms.
      assert!(
        W::get().reads(1).ref_time() <= constants::WEIGHT_PER_MILLIS.ref_time(),
        "Read weight should be at most 1 ms."
      );
      assert!(
        W::get().writes(1).ref_time() <= constants::WEIGHT_PER_MILLIS.ref_time(),
        "Write weight should be at most 1 ms."
      );
    }
  }
}
