//! Paratensor pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]
//mod benchmarking;

use crate::*;
use crate::Pallet as Paratensor;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;
pub use pallet::*;
use frame_support::assert_ok;
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
    let netuid: u16 = 1;
    let tempo: u16 = 0;
    let modality: u16 = 0;
        let stake: u64 = 10;
        let balance: u64 = 1000000000;

    assert_ok!( Paratensor::<T>::do_add_network( RawOrigin::Root.into(), netuid.try_into().unwrap(), tempo.into(), modality.into()));
    Paratensor::<T>::set_max_allowed_uids( netuid, 4096 ); 
    assert_eq!(Paratensor::<T>::get_max_allowed_uids(netuid), 4096);

    let mut seed : u32 = 1;
    let block_number: u64 = Paratensor::<T>::get_current_block_as_u64();
    let hotkey: T::AccountId = account("Alice", 0, seed);
    let coldkey: T::AccountId = account("Test", 0, seed);

    let amoun_to_be_staked = Paratensor::<T>::u64_to_balance( balance );
    Paratensor::<T>::add_balance_to_coldkey_account(&coldkey.clone(), amoun_to_be_staked.unwrap());

  }: sudo_register(RawOrigin::<AccountIdOf<T>>::Root, netuid, hotkey, coldkey, stake, balance)
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
