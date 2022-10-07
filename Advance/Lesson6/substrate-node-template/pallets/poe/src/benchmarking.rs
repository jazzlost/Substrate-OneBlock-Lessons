//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PoePallet;
use frame_benchmarking::{benchmarks, whitelisted_caller, whitelist_account, account};
use frame_system::RawOrigin;

const SEED: u32 = 0;

benchmarks! {
	create_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec!{0; d as usize};
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), claim)

	transfer_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec!{0; d as usize};
		let caller: T::AccountId = whitelisted_caller();

		let origin = RawOrigin::Signed(caller.clone()).into();
		let _ = PoePallet::<T>::create_claim(origin, claim.clone());

		let des: T::AccountId = account::<T::AccountId>("des", 1, SEED);
		whitelist_account!(des);

	}: _(RawOrigin::Signed(caller), claim, des)

	revoke_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec!{0; d as usize};
		let caller: T::AccountId = whitelisted_caller();

		let origin = RawOrigin::Signed(caller.clone()).into();
		let _ = PoePallet::<T>::create_claim(origin, claim.clone());
	}: _(RawOrigin::Signed(caller), claim)

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}
