
//! Autogenerated weights for `pallet_babe`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/dfinn-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet=pallet_babe
// --extrinsic=*
// --steps
// 50
// --repeat
// 20
// --output=benchout/pallet_babe.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_babe.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_babe::WeightInfo for WeightInfo<T> {
	fn plan_config_change() -> Weight {
		(88_327_000 as Weight).saturating_add((10_000 as Weight))
	}
	fn report_equivocation(x: u32, ) -> Weight {
		(88_327_000 as Weight)
			// Standard Error: 97_000
			.saturating_add((10_000 as Weight).saturating_mul(x as Weight))
	}
	
}
