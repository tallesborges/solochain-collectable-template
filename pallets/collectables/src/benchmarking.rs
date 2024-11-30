//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_kitty() {
		let caller: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		create_kitty(RawOrigin::Signed(caller.clone()));

		let count = CountForKitties::<T>::get();
		assert_eq!(count, 1);

		let owned = KittiesOwned::<T>::get(caller);
		assert_eq!(owned.len(), 1);
	}

	impl_benchmark_test_suite!(Template, crate::tests::new_test_ext(), crate::tests::TestRuntime);
}
