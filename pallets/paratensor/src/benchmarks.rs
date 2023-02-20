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

  benchmark_epoch_with_weights { 
    let caller: T::AccountId = whitelisted_caller::<AccountIdOf<T>>(); 
    let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));
    Paratensor::<T>::create_network_with_weights(
      caller_origin.clone(), 
      11u16.into(), // netuid
      4096u16.into(), // n
      1000u16.into(), // tempo
      100u16.into(), // n_vals
      1000u16.into() // n_weights
    );
  }: _( RawOrigin::Signed( caller.clone() ) )

  benchmark_epoch_without_weights { 
    let caller: T::AccountId = whitelisted_caller::<AccountIdOf<T>>(); 
    let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));
    Paratensor::<T>::create_network(
      caller_origin.clone(), 
      11u16.into(), // netuid
      4096u16.into(), // n
      1000u16.into(), // tempo
    );
  }: _( RawOrigin::Signed( caller.clone() ) )

  benchmark_sudo_register {
    let caller: T::AccountId = whitelisted_caller::<AccountIdOf<T>>(); 
    let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));
    Paratensor::<T>::create_network_with_weights(
      caller_origin.clone(), 
      11u16.into(), // netuid
      4096u16.into(), // n
      1000u16.into(), // tempo
      100u16.into(), // n_vals
      1000u16.into() // n_weights
    );
  }: sudo_register( RawOrigin::Signed( caller.clone() ),  11u16.into(), account("Alice", 0, 1), account("Bob", 0, 2), 100u64.into(), 120u64.into() )
  // Add individual benchmarks here
  // benchmark_drain_emission { 

  //   // This is a whitelisted caller who can make transaction without weights.
  //   let caller: T::AccountId = whitelisted_caller::<AccountIdOf<T>>(); 
  //   let caller_origin = <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(caller.clone()));

  //   // Lets create a single network.
  //   let n: u16 = 4096;
  //   let netuid: u16 = 11; //11 is the benchmark network.
  //   let tempo: u16 = 1;
  //   let modality: u16 = 0;
  //   Paratensor::<T>::do_add_network( caller_origin.clone(), netuid.try_into().unwrap(), tempo.into(), modality.into());
	// 	Paratensor::<T>::set_max_allowed_uids( netuid, n ); 
  //   Paratensor::<T>::set_tempo( netuid, tempo );

  //   // Lets fill the network with 100 UIDS and no weights.
  //   let mut SEED : u32 = 1;
  //   let mut emission: Vec<(T::AccountId, u64)> = vec![];
  //   for uid in 0..n as u16 {
  //       let block_number: u64 = Paratensor::<T>::get_current_block_as_u64();
  //       let hotkey: T::AccountId = account("Alice", 0, SEED);
  //       Paratensor::<T>::append_neuron( netuid, &hotkey, block_number );
  //       SEED = SEED + 1;
  //       emission.push( ( hotkey, 1 ) );
  //   }
  //   Paratensor::<T>::sink_emission( netuid, emission );

  // }: _( RawOrigin::Signed( caller.clone() ) )
}
