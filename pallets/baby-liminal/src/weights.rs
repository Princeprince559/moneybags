//! Autogenerated weights for pallet_baby_liminal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-12, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("chainspec.json"), DB CACHE: 1024

// Executed Command:
// target/release/aleph-node
// benchmark
// pallet
// --chain=chainspec.json
// --pallet=pallet_baby_liminal
// --extrinsic=*
// --steps=20
// --repeat=10
// --template=.maintain/pallet-weight-template.hbs
// --execution=wasm
// --wasm-execution=compiled
// --output=pallets/baby-liminal/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_baby_liminal.
pub trait WeightInfo {
    fn store_key(key_length: u32) -> Weight;
    fn overwrite_equal_key(key_length: u32) -> Weight;
    fn overwrite_key(key_length: u32) -> Weight;
    fn delete_key(key_length: u32) -> Weight;
    fn verify() -> Weight;
    fn verify_data_too_long(excess: u32) -> Weight;
    fn verify_data_deserializing_fails(data_length: u32) -> Weight;
    fn verify_key_deserializing_fails(key_length: u32) -> Weight;
    fn poseidon_one_to_one_wasm() -> Weight;
    fn poseidon_two_to_one_wasm() -> Weight;
    fn poseidon_four_to_one_wasm() -> Weight;
    fn poseidon_one_to_one_host() -> Weight;
    fn poseidon_two_to_one_host() -> Weight;
    fn poseidon_four_to_one_host() -> Weight;
}

impl<I: BenchmarkInfo> WeightInfo for I {
    fn store_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::store_key(key_length)
    }

    fn overwrite_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::overwrite_key(key_length)
    }

    fn overwrite_equal_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::overwrite_equal_key(key_length)
    }

    fn delete_key(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::delete_key(key_length)
    }

    fn verify() -> Weight {
        <I as BenchmarkInfo>::verify_groth16_xor()
            .max(<I as BenchmarkInfo>::verify_groth16_linear_equation())
            .max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_8())
            .max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_64())
            .max(<I as BenchmarkInfo>::verify_groth16_merkle_tree_1024())
    }

    fn verify_data_too_long(excess: u32) -> Weight {
        <I as BenchmarkInfo>::verify_data_too_long(excess)
    }

    fn verify_data_deserializing_fails(data_length: u32) -> Weight {
        <I as BenchmarkInfo>::verify_data_deserializing_fails(data_length)
    }

    fn verify_key_deserializing_fails(key_length: u32) -> Weight {
        <I as BenchmarkInfo>::verify_key_deserializing_fails(key_length)
    }

    fn poseidon_one_to_one_wasm() -> Weight {
        <I as BenchmarkInfo>::poseidon_one_to_one_wasm()
    }

    fn poseidon_two_to_one_wasm() -> Weight {
        <I as BenchmarkInfo>::poseidon_two_to_one_wasm()
    }

    fn poseidon_four_to_one_wasm() -> Weight {
        <I as BenchmarkInfo>::poseidon_four_to_one_wasm()
    }

    fn poseidon_one_to_one_host() -> Weight {
        <I as BenchmarkInfo>::poseidon_one_to_one_host()
    }

    fn poseidon_two_to_one_host() -> Weight {
        <I as BenchmarkInfo>::poseidon_two_to_one_host()
    }

    fn poseidon_four_to_one_host() -> Weight {
        <I as BenchmarkInfo>::poseidon_four_to_one_host()
    }
    // TODO: compile results from benchmarks
}

/// Benchmark results for pallet_baby_liminal.
trait BenchmarkInfo {
	fn store_key(l: u32, ) -> Weight;
	fn overwrite_equal_key(l: u32, ) -> Weight;
	fn overwrite_key(l: u32, ) -> Weight;
	fn delete_key(l: u32, ) -> Weight;
	fn verify_groth16_xor() -> Weight;
	fn verify_groth16_linear_equation() -> Weight;
	fn verify_groth16_merkle_tree_8() -> Weight;
	fn verify_groth16_merkle_tree_64() -> Weight;
	fn verify_groth16_merkle_tree_1024() -> Weight;
	fn verify_data_too_long(e: u32, ) -> Weight;
	fn verify_data_deserializing_fails(l: u32, ) -> Weight;
	fn verify_key_deserializing_fails(l: u32, ) -> Weight;
	fn poseidon_one_to_one_wasm() -> Weight;
	fn poseidon_two_to_one_wasm() -> Weight;
	fn poseidon_four_to_one_wasm() -> Weight;
	fn poseidon_one_to_one_host() -> Weight;
	fn poseidon_two_to_one_host() -> Weight;
	fn poseidon_four_to_one_host() -> Weight;
}

/// Weights for pallet_baby_liminal using the Substrate node and recommended hardware.
pub struct AlephWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> BenchmarkInfo for AlephWeight<T> {
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyOwners (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 33_270 nanoseconds.
		Weight::from_ref_time(36_539_015_u64)
			// Standard Error: 56
			.saturating_add(Weight::from_ref_time(669_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_equal_key(l: u32, ) -> Weight {
		// Minimum execution time: 39_501 nanoseconds.
		Weight::from_ref_time(44_208_753_u64)
			// Standard Error: 279
			.saturating_add(Weight::from_ref_time(1_189_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 9999]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 37_732 nanoseconds.
		Weight::from_ref_time(43_852_538_u64)
			// Standard Error: 61
			.saturating_add(Weight::from_ref_time(775_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn delete_key(_l: u32, ) -> Weight {
		// Minimum execution time: 30_814 nanoseconds.
		Weight::from_ref_time(34_440_507_u64)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_xor() -> Weight {
		// Minimum execution time: 41_293_392 nanoseconds.
		Weight::from_ref_time(41_448_382_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_linear_equation() -> Weight {
		// Minimum execution time: 32_466_859 nanoseconds.
		Weight::from_ref_time(32_557_109_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_8() -> Weight {
		// Minimum execution time: 43_192_946 nanoseconds.
		Weight::from_ref_time(43_379_903_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_64() -> Weight {
		// Minimum execution time: 43_343_747 nanoseconds.
		Weight::from_ref_time(43_538_873_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_1024() -> Weight {
		// Minimum execution time: 42_962_146 nanoseconds.
		Weight::from_ref_time(44_068_421_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `e` is `[1, 10000000]`.
	fn verify_data_too_long(_e: u32, ) -> Weight {
		// Minimum execution time: 16_815 nanoseconds.
		Weight::from_ref_time(20_977_281_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_data_deserializing_fails(_l: u32, ) -> Weight {
		// Minimum execution time: 21_731 nanoseconds.
		Weight::from_ref_time(24_058_546_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_key_deserializing_fails(l: u32, ) -> Weight {
		// Minimum execution time: 5_489_799 nanoseconds.
		Weight::from_ref_time(5_681_917_698_u64)
			// Standard Error: 8_383
			.saturating_add(Weight::from_ref_time(76_571_u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_wasm() -> Weight {
		// Minimum execution time: 5_718_589 nanoseconds.
		Weight::from_ref_time(6_089_141_701_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_wasm() -> Weight {
		// Minimum execution time: 9_014_496 nanoseconds.
		Weight::from_ref_time(9_137_855_634_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_wasm() -> Weight {
		// Minimum execution time: 16_951_808 nanoseconds.
		Weight::from_ref_time(18_149_939_233_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_host() -> Weight {
		// Minimum execution time: 913_823 nanoseconds.
		Weight::from_ref_time(994_920_187_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_host() -> Weight {
		// Minimum execution time: 1_473_379 nanoseconds.
		Weight::from_ref_time(1_605_239_651_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_host() -> Weight {
		// Minimum execution time: 2_753_121 nanoseconds.
		Weight::from_ref_time(2_909_130_711_u64)
	}
}

// For backwards compatibility and tests
impl BenchmarkInfo for () {
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyOwners (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn store_key(l: u32, ) -> Weight {
		// Minimum execution time: 33_270 nanoseconds.
		Weight::from_ref_time(36_539_015_u64)
			// Standard Error: 56
			.saturating_add(Weight::from_ref_time(669_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn overwrite_equal_key(l: u32, ) -> Weight {
		// Minimum execution time: 39_501 nanoseconds.
		Weight::from_ref_time(44_208_753_u64)
			// Standard Error: 279
			.saturating_add(Weight::from_ref_time(1_189_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 9999]`.
	fn overwrite_key(l: u32, ) -> Weight {
		// Minimum execution time: 37_732 nanoseconds.
		Weight::from_ref_time(43_852_538_u64)
			// Standard Error: 61
			.saturating_add(Weight::from_ref_time(775_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeyOwners (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeyOwners (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeyDeposits (r:1 w:1)
	// Proof Skipped: BabyLiminal VerificationKeyDeposits (max_values: None, max_size: None, mode: Measured)
	// Storage: BabyLiminal VerificationKeys (r:0 w:1)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn delete_key(_l: u32, ) -> Weight {
		// Minimum execution time: 30_814 nanoseconds.
		Weight::from_ref_time(34_440_507_u64)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_xor() -> Weight {
		// Minimum execution time: 41_293_392 nanoseconds.
		Weight::from_ref_time(41_448_382_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_linear_equation() -> Weight {
		// Minimum execution time: 32_466_859 nanoseconds.
		Weight::from_ref_time(32_557_109_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_8() -> Weight {
		// Minimum execution time: 43_192_946 nanoseconds.
		Weight::from_ref_time(43_379_903_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_64() -> Weight {
		// Minimum execution time: 43_343_747 nanoseconds.
		Weight::from_ref_time(43_538_873_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	fn verify_groth16_merkle_tree_1024() -> Weight {
		// Minimum execution time: 42_962_146 nanoseconds.
		Weight::from_ref_time(44_068_421_000_u64)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `e` is `[1, 10000000]`.
	fn verify_data_too_long(_e: u32, ) -> Weight {
		// Minimum execution time: 16_815 nanoseconds.
		Weight::from_ref_time(20_977_281_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_data_deserializing_fails(_l: u32, ) -> Weight {
		// Minimum execution time: 21_731 nanoseconds.
		Weight::from_ref_time(24_058_546_u64)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: BabyLiminal VerificationKeys (r:1 w:0)
	// Proof Skipped: BabyLiminal VerificationKeys (max_values: None, max_size: None, mode: Measured)
	/// The range of component `l` is `[1, 10000]`.
	fn verify_key_deserializing_fails(l: u32, ) -> Weight {
		// Minimum execution time: 5_489_799 nanoseconds.
		Weight::from_ref_time(5_681_917_698_u64)
			// Standard Error: 8_383
			.saturating_add(Weight::from_ref_time(76_571_u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_wasm() -> Weight {
		// Minimum execution time: 5_718_589 nanoseconds.
		Weight::from_ref_time(6_089_141_701_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_wasm() -> Weight {
		// Minimum execution time: 9_014_496 nanoseconds.
		Weight::from_ref_time(9_137_855_634_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_wasm() -> Weight {
		// Minimum execution time: 16_951_808 nanoseconds.
		Weight::from_ref_time(18_149_939_233_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	fn poseidon_one_to_one_host() -> Weight {
		// Minimum execution time: 913_823 nanoseconds.
		Weight::from_ref_time(994_920_187_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	fn poseidon_two_to_one_host() -> Weight {
		// Minimum execution time: 1_473_379 nanoseconds.
		Weight::from_ref_time(1_605_239_651_u64)
	}
	/// The range of component `x` is `[0, 4294967295]`.
	/// The range of component `y` is `[0, 4294967295]`.
	/// The range of component `w` is `[0, 4294967295]`.
	/// The range of component `z` is `[0, 4294967295]`.
	fn poseidon_four_to_one_host() -> Weight {
		// Minimum execution time: 2_753_121 nanoseconds.
		Weight::from_ref_time(2_909_130_711_u64)
	}
}
