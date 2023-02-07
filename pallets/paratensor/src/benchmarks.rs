//! Paratensor pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

use crate::*;
use super::*;
use crate::Pallet as Paratensor;
use sp_runtime::traits::Bounded;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite, account};
use frame_system::RawOrigin;
use frame_support::sp_std::vec;
use frame_support::inherent::Vec;
pub use pallet::*;
//use mock::{Test, new_test_ext};

benchmarks! {
   
  // Add individual benchmarks here
  sudo_benchmark_block_step { 

    // This is a whitelisted caller who can make transaction without weights.
    let caller: T::AccountId = whitelisted_caller::<AccountIdOf<T>>(); 
    let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

    // Lets create a single network.
    let netuid: u16 = 11; //11 is the benchmark network.
    let tempo: u16 = 0;
    let modality: u16 = 0;
    Paratensor::<T>::do_add_network( caller_origin.clone(), netuid.try_into().unwrap(), tempo.into(), modality.into());

    // Maybe?
    Paratensor::<T>::set_difficulty( netuid, 1);

    // Lets fill the network with 100 UIDS and no weights.
    let mut SEED : u32 = 1;
    for uid in 0..4066 as u16 {
        let block_number: u64 = Paratensor::<T>::get_current_block_as_u64();
        let hotkey: T::AccountId = account("Alice", 0, SEED);
        let coldkey: T::AccountId = account("Test", 0, SEED);
        let start_nonce: u64 = (39420842u64 + 100u64*uid as u64).into();
        let (nonce, work): (u64, Vec<u8>) = Paratensor::<T>::create_work_for_block_number( uid, block_number, start_nonce );
        Paratensor::<T>::register( caller_origin.clone(), uid, block_number, nonce, work, hotkey, coldkey );
        SEED = SEED + 1;
    }

  }: _( RawOrigin::Signed( caller.clone() ) )

  verify {
     /* optional verification */
     assert_eq!( Paratensor::<T>::get_blocks_since_last_step(0), 0 );
  }
}

/* impl_benchmark_test_suite!(
    Paratensor,
    mock::new_test_ext(),
    mock::Test,
   ); */
