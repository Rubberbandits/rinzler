use crate::{mock::*};
mod mock;

#[test]
fn test_registration_difficulty_adjustment() {
	new_test_ext().execute_with(|| { 

        // Create Net 1
		let netuid: u16 = 1;
        let tempo: u16 = 1;
        let modality: u16 = 1;
        add_network( netuid, tempo, modality );
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 10000 ); // Check initial difficulty.
        assert_eq!( ParatensorModule::get_last_adjustment_block( netuid ), 0 ); // Last adjustment block starts at 0.
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 0 ); // No registrations this block.
        assert_eq!( ParatensorModule::get_target_registrations_per_interval( netuid ), 2 ); // Target is default.
        assert_eq!( ParatensorModule::get_adjustment_interval( netuid ), 100 ); // Default adustment intrerval.
        
        // Set values and check.
        ParatensorModule::set_difficulty( netuid, 20000 );
        ParatensorModule::set_adjustment_interval( netuid, 1 );
        ParatensorModule::set_target_registrations_per_interval( netuid, 1 );
        ParatensorModule::set_max_registrations_per_block( netuid, 3 );
        ParatensorModule::set_max_allowed_uids( netuid, 3 );
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 20000 ); // Check set difficutly.
        assert_eq!( ParatensorModule::get_adjustment_interval( netuid ), 1 ); // Check set adjustment interval.
        assert_eq!( ParatensorModule::get_target_registrations_per_interval( netuid ), 1 ); // Check set adjustment interval.
        assert_eq!( ParatensorModule::get_max_registrations_per_block( netuid ), 3 ); // Check set registrations per block.
        assert_eq!( ParatensorModule::get_max_allowed_uids( netuid ), 3 ); // Check set registrations per block.

        // Lets register 3 neurons...
        let hotkey0: u64 = 0;
        let hotkey1: u64 = 100;
        let hotkey2: u64 = 2000;
        let coldkey0: u64 = 0;
        let coldkey1: u64 = 1000;
        let coldkey2: u64 = 20000;
    	register_ok_neuron( netuid, hotkey0, coldkey0, 39420842 );
    	register_ok_neuron( netuid, hotkey1, coldkey1, 12412392 );
		register_ok_neuron( netuid, hotkey2, coldkey2, 21813123 );
        assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 0 ).unwrap(), hotkey0 );
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 1 ).unwrap(), hotkey1 );
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 2 ).unwrap(), hotkey2 );

		assert_eq!( ParatensorModule::get_subnetwork_n(netuid), 3); // All 3 are registered.
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 3 ); // 3 Registrations.
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 3 ); // 3 Registrations this interval.

        // Fast forward 1 block.
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 20000 ); // Difficulty is unchanged.
        step_block( 1 );
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 0 ); // Registrations have been erased.
        assert_eq!( ParatensorModule::get_last_adjustment_block( netuid ), 1 ); // We just adjusted on the first block.
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 40000 ); // Difficulty is increased ( 20000 * ( 3 + 1 ) / ( 1 + 1 ) ) = 80_000
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 0 ); // Registrations this interval has been wiped.

        // Lets change the adjustment interval
        ParatensorModule::set_adjustment_interval( netuid, 3 );
        assert_eq!( ParatensorModule::get_adjustment_interval( netuid ), 3 ); // Check set adjustment interval.

        // Register 3 more 
        register_ok_neuron( netuid, hotkey0 + 1, coldkey0 + 1, 3942084 );
    	register_ok_neuron( netuid, hotkey1 + 1, coldkey1 + 1, 1241239 );
		register_ok_neuron( netuid, hotkey2 + 1, coldkey2 + 1, 2181312 );
        assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 0 ).unwrap(), hotkey0 + 1); // replace 0
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 1 ).unwrap(), hotkey1 + 1); // replace 1
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 2 ).unwrap(), hotkey2 + 1); // replace 2
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 3 ); // Registrations have been erased.
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 3 ); // Registrations this interval = 3

        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_last_adjustment_block( netuid ), 1 ); // Still previous adjustment block.
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 0 ); // Registrations have been erased.
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 3 ); // Registrations this interval = 3

        // Register 3 more.
    	register_ok_neuron( netuid, hotkey0 + 2, coldkey0 + 2, 394208420 );
    	register_ok_neuron( netuid, hotkey1 + 2, coldkey1 + 2, 124123920 );
		register_ok_neuron( netuid, hotkey2 + 2, coldkey2 + 2, 218131230 );
        assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 0 ).unwrap(), hotkey0 + 2); // replace 0
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 1 ).unwrap(), hotkey1 + 2); // replace 1
		assert_eq!( ParatensorModule::get_hotkey_for_net_and_uid( netuid, 2 ).unwrap(), hotkey2 + 2); // replace 2
        assert_eq!( ParatensorModule::get_registrations_this_block( netuid ), 3 ); // Registrations have been erased.

        // We have 6 registrations this adjustment interval.
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 6 ); // Registrations this interval = 6
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 40000 ); // Difficulty unchanged.
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 140_000 ); // Difficulty changed ( 40000 ) * ( 6 + 1 / 1 + 1 ) = 40000 * 3.5 = 140_000
        assert_eq!( ParatensorModule::get_registrations_this_interval( netuid ), 0 ); // Registrations this interval drops to 0.

        // Test min value.
        ParatensorModule::set_min_difficulty( netuid, 1 );
        ParatensorModule::set_difficulty( netuid, 4 );
        assert_eq!( ParatensorModule::get_min_difficulty( netuid ), 1 ); 
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 4 ); 
        ParatensorModule::set_adjustment_interval( netuid, 1 );
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 2 ); // Difficulty dropped 4 * ( 0 + 1 ) / (1 + 1) = 1/2 = 2
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 1 ); // Difficulty dropped 2 * ( 0 + 1 ) / (1 + 1) = 1/2 = 1
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 1 ); // Difficulty dropped 2 * ( 0 + 1 ) / (1 + 1) = 1/2 = max(0.5, 1) 

        // Test max value.
        ParatensorModule::set_max_difficulty( netuid, 10 );
        ParatensorModule::set_difficulty( netuid, 5 );
        assert_eq!( ParatensorModule::get_max_difficulty( netuid ), 10); 
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 5); 
        ParatensorModule::set_adjustment_interval( netuid, 1 );
        register_ok_neuron( netuid, hotkey0 + 3, coldkey0 + 3, 294208420 );
    	register_ok_neuron( netuid, hotkey1 + 3, coldkey1 + 3, 824123920 );
        register_ok_neuron( netuid, hotkey2 + 3, coldkey2 + 3, 324123920 );
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 10 ); // Difficulty increased 5 * ( 3 + 1 ) / (1 + 1) = 2 * 5 = 10

        register_ok_neuron( netuid, hotkey0 + 4, coldkey0 + 4, 124208420 );
    	register_ok_neuron( netuid, hotkey1 + 4, coldkey1 + 4, 314123920 );
        register_ok_neuron( netuid, hotkey2 + 4, coldkey2 + 4, 834123920 );
        step_block( 1 ); // Step
        assert_eq!( ParatensorModule::get_difficulty_as_u64( netuid ), 10 ); // Difficulty increased 10 * ( 3 + 1 ) / (1 + 1) = min( 10, 2 * 10 ) = 10

	});
}