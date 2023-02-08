use super::*;
use substrate_fixed::types::I110F18;
use substrate_fixed::types::I64F64;
use frame_support::inherent::Vec;
use frame_support::storage::IterableStorageMap;
use frame_support::storage::IterableStorageDoubleMap;

impl<T: Config> Pallet<T> { 

    pub fn block_step() {
        log::debug!("block_step for block: {:?} ", Self::get_current_block_as_u64() );
        // --- 1. Adjust difficulties.
		Self::adjust_registration_difficulties( );
		// --- 2. Distribute emission.
		Self::distribute_pending_emission_onto_networks( );
        // --- 3. Run epochs.
        Self::run_epochs_and_emit( );
    }

    /// Distributes pending emission onto each network based on the emission vector.
    ///
    pub fn distribute_pending_emission_onto_networks( ) {
        // --- 1. We iterate across each network and add the emission value onto the network's pending emission.
        // The pending emission will acrue until this network runs its epoch function.
        for (netuid_i, _) in <SubnetworkN<T> as IterableStorageMap<u16, u16>>::iter(){ 
            // --- 2. Get the emission value for this network which is a value < block emission
            // and all emission values sum to block_emission() 
            let new_emission = EmissionValues::<T>::get( netuid_i );
            PendingEmission::<T>::mutate(netuid_i, |val| *val += new_emission);
            log::debug!("netuid_i: {:?} new_emission: +{:?} ", netuid_i, new_emission );
        }
    }

    pub fn sink_emission( netuid: u16, emission: Vec<(T::AccountId, u64)> )  {
        let mut emission_to_sink: Vec<(T::AccountId, u64)> = emission.clone();
        if LoadedEmission::<T>::contains_key( netuid ) {
            let mut already_sunk_emission: Vec<(T::AccountId, u64)> = LoadedEmission::<T>::get( netuid ).unwrap();
            emission_to_sink.append( &mut already_sunk_emission );
        }
        LoadedEmission::<T>::insert( netuid, emission_to_sink );
    }

    pub fn drain_emission( netuid: u16 )  {
        if LoadedEmission::<T>::contains_key( netuid ) {
            let mut emission_to_sink: Vec<(T::AccountId, u64)> = LoadedEmission::<T>::get( netuid ).unwrap();
            let tempo: u16 = Tempo::<T>::get( netuid );
            log::info!( "tempo: {:?}", tempo );

            let block_number: u64 = Self::get_current_block_as_u64();
            log::info!( "block_number: {:?}", block_number );

            let blocks_to_emit: u64 = (tempo as u64 - ( block_number + 1 ) % ( tempo as u64 + 1 ) ) / 2;
            log::info!( "blocks_until_next_epoch: {:?}", blocks_to_emit );

            let total_emits_to_drain: u64 = emission_to_sink.len() as u64;
            log::info!( "total_emits_to_drain: {:?}", total_emits_to_drain );

            let items_required_per_block: u64;
            if blocks_to_emit == 0 {
                items_required_per_block  = total_emits_to_drain;
            } else {
                items_required_per_block  = total_emits_to_drain / blocks_to_emit;
            }
            log::info!( "items_required_per_block: {:?}", items_required_per_block );

            for _ in 0..items_required_per_block { 
                if emission_to_sink.len() > 0 {
                    let (hotkey, amount): (T::AccountId, u64) = emission_to_sink.pop().unwrap();
                    Self::emit_inflation_through_hotkey_account( &hotkey, amount );
                } else { break; }
            }
            LoadedEmission::<T>::insert( netuid, emission_to_sink );
        }
    }


    /// Runs each network epoch function based on tempo.
    ///
    pub fn run_epochs_and_emit() {
        // --- 1. First get the current block number which will be used to determine which networks 
        // we will be draining of pending emission.
        let block_number = Self::get_current_block_as_u64();  

        // --- 2. Next we will iterate over all active networks via tempo and distribute the 
        // emission if it is the networks time to run the epoch.
        for ( netuid_i, tempo_i )  in <Tempo<T> as IterableStorageMap<u16, u16>>::iter() {

            Self::drain_emission( netuid_i );
            
            // --- 3. Check to see if this network has hit its tempo.
            // Ee check ( block_number + 1 ) % ( tempo_i as u64 + 1 ) == 0
            // We begin on the first block.
            // tempo = 0, run every block.
            // tempo = 1, skip 1 block then run
            // tempo = 2, skip 2 blocks then run ...
            log::debug!("netuid_i: {:?} tempo_i: {:?} block_number: {:?} ", netuid_i, tempo_i, block_number );
            if ( block_number + 1 ) % ( tempo_i as u64 + 1 ) == 0 {

                // --- 4. We attain the pending emission and drain it. 
                let emission_to_drain:u64 = PendingEmission::<T>::get( netuid_i );
                PendingEmission::<T>::insert( netuid_i, 0 );

                // --- 5. Run the epoch mechanism and return the tao_emission which will later be sunk.
                let tao_emission: Vec<(T::AccountId, u64)> = Self::epoch( netuid_i, emission_to_drain );

                // --- 6. Check the total emission for sanity. If we are not emitting more than
                // we are allowed, we are pushing it to the Emission storage to be sunk next step.
                let emission_sum: u128 = tao_emission.iter().map( |(h,e)| *e as u128 ).sum();
                if emission_sum <= emission_to_drain as u128 {
                    Self::sink_emission( netuid_i, tao_emission );
                }

                // --- 8. Drain blocks and set epoch counters.
                Self::set_blocks_since_last_step( netuid_i, 0 );
                Self::set_last_mechanism_step_block( netuid_i, block_number );
                log::debug!("netuid_i: {:?} emission_to_drain: {:?} ", netuid_i, emission_to_drain );

            } else {

                // --- 9. No epoch, then increase blocks since last step.
                Self::set_blocks_since_last_step( netuid_i, Self::get_blocks_since_last_step( netuid_i ) + 1 );
            }

        }
    }


    /// Distributes token inflation through the hotkey based on emission. The call ensures that the inflation
    /// is distributed onto the accounts in proportion of the stake delegated minus the take. This function
    /// is called after an epoch to distribute the newly minted stake according to delegation.
    ///
    pub fn emit_inflation_through_hotkey_account( hotkey: &T::AccountId, emission: u64) {
        
        // --- 1. Check if the hotkey is a delegate. If not, we simply pass the stake through to the 
        // coldkye - hotkey account as normal.
        if !Self::hotkey_is_delegate( hotkey ) { 
            Self::increase_stake_on_hotkey_account( &hotkey, emission ); 
            return; 
        }

        // --- 2. The hotkey is a delegate. We first distribute a proportion of the emission to the hotkey
        // directly as a function of its 'take'
        let total_hotkey_stake: u64 = Self::get_total_stake_for_hotkey( hotkey );
        let delegate_take: u64 = Self::calculate_delegate_proportional_take( hotkey, emission );
        let remaining_emission: u64 = emission - delegate_take;

        // 3. -- The remaining emission goes to the owners in proportion to the stake delegated.
        for ( owning_coldkey_i, stake_i ) in < Stake<T> as IterableStorageDoubleMap<T::AccountId, T::AccountId, u64 >>::iter_prefix( hotkey ) {
            
            // --- 4. The emission proportion is remaining_emission * ( stake / total_stake ).
            let stake_proportion: u64 = Self::calculate_stake_proportional_emission( stake_i, total_hotkey_stake, remaining_emission );
            Self::increase_stake_on_coldkey_hotkey_account( &owning_coldkey_i, &hotkey, stake_proportion );
            log::debug!("owning_coldkey_i: {:?} hotkey: {:?} emission: +{:?} ", owning_coldkey_i, hotkey, stake_proportion );

        }

        // --- 5. Last increase final account balance of delegate after 4, since 5 will change the stake proportion of 
        // the delegate and effect calculation in 4.
        Self::increase_stake_on_hotkey_account( &hotkey, delegate_take );
        log::debug!("delkey: {:?} delegate_take: +{:?} ", hotkey,delegate_take );
    }

    /// Returns emission awarded to a hotkey as a function of its proportion of the total stake.
    ///
    pub fn calculate_stake_proportional_emission( stake: u64, total_stake:u64, emission: u64 ) -> u64 {
        if total_stake == 0 { return 0 };
        let stake_proportion: I64F64 = I64F64::from_num( stake ) / I64F64::from_num( total_stake );
        let proportional_emission: I64F64 = I64F64::from_num( emission ) * stake_proportion;
        return proportional_emission.to_num::<u64>();
    }

    /// Returns the delegated stake 'take' assigend to this key. (If exists, otherwise 0)
    ///
    pub fn calculate_delegate_proportional_take( hotkey: &T::AccountId, emission: u64 ) -> u64 {
        if Self::hotkey_is_delegate( hotkey ) {
            let take_proportion: I64F64 = I64F64::from_num( Delegates::<T>::get( hotkey ) ) / I64F64::from_num( u16::MAX );
            let take_emission: I64F64 = take_proportion * I64F64::from_num( emission );
            return take_emission.to_num::<u64>();
        } else {
            return 0;
        }
    }

    /// Adjusts the network difficulty of every active network. Reseting state parameters.
    ///
    pub fn adjust_registration_difficulties( ) {
        
        // --- 1. Iterate through each network.
        for ( netuid, _ )  in <NetworksAdded<T> as IterableStorageMap<u16, bool>>::iter(){

            // --- 2. Pull counters for network difficulty.
            let last_adjustment_block: u64 = Self::get_last_adjustment_block( netuid );
            let adjustment_interval: u16 = Self::get_adjustment_interval( netuid );
            let current_block: u64 = Self::get_current_block_as_u64( ); 
            log::debug!("netuid: {:?} last_adjustment_block: {:?} adjustment_interval: {:?} current_block: {:?}", 
                netuid,
                last_adjustment_block,
                adjustment_interval,
                current_block
            );

            // --- 3. Check if we are at the adjustment interval for this network.
            // If so, we need to adjust the registration difficulty based on target and actual registrations.
            if ( current_block - last_adjustment_block ) >= adjustment_interval as u64 {
                let current_difficulty: u64 = Self::get_difficulty_as_u64( netuid );
                let registrations_this_interval: u16 = Self::get_registrations_this_interval( netuid );
                let target_registrations_this_interval: u16 = Self::get_target_registrations_per_interval( netuid );

                // --- 4. Adjust network registration interval. 
                // next_dif = next_dif * ( reg_actual + reg_target / reg_target * reg_target )
                let adjusted_difficulty: u64 = Self::get_next_difficulty( 
                    netuid,
                    current_difficulty,
                    registrations_this_interval,
                    target_registrations_this_interval
                );
                Self::set_difficulty( netuid, adjusted_difficulty );
                Self::set_last_adjustment_block( netuid, current_block );
                Self::set_registrations_this_interval( netuid, 0 );
                log::debug!("netuid: {:?} current_difficulty: {:?} interval_regs: {:?} target: {:?} adjusted_difficulty: {:?}", 
                    netuid,
                    current_difficulty,
                    registrations_this_interval,
                    target_registrations_this_interval,
                    adjusted_difficulty
                );
            }

            // --- 5. Drain block registrations for each network. Needed for registration rate limits.
            Self::set_registrations_this_block( netuid, 0 );
        }
    }

    /// Performs the difficutly adjustment by multiplying the current difficulty by the ratio ( reg_actual + reg_target / reg_target * reg_target )
    /// We use I110F18 to avoid any overflows on u64. Also min_difficulty and max_difficutly bound the range.
    ///
    pub fn get_next_difficulty( 
        netuid: u16,
        current_difficulty: u64, 
        registrations_this_interval: u16, 
        target_registrations_per_interval: u16 
    ) -> u64 {
        let next_value: I110F18 = I110F18::from_num( current_difficulty ) * I110F18::from_num( registrations_this_interval + target_registrations_per_interval ) / I110F18::from_num( target_registrations_per_interval + target_registrations_per_interval );
        if next_value >= I110F18::from_num( Self::get_max_difficulty( netuid ) ){
            return Self::get_max_difficulty( netuid );
        } else if next_value <= I110F18::from_num( Self::get_min_difficulty( netuid ) ) {
            return Self::get_min_difficulty( netuid );
        } else {
            return next_value.to_num::<u64>();
        }
    }

}