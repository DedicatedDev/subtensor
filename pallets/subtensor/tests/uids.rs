#![allow(clippy::unwrap_used)]

use crate::mock::*;
use frame_support::assert_ok;
use frame_system::Config;
use sp_core::U256;

mod mock;

/********************************************
    tests for uids.rs file
*********************************************/

/********************************************
    tests uids::replace_neuron()
*********************************************/

//  SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test uids -- test_replace_neuron --exact --nocapture
#[test]
fn test_replace_neuron() {
    new_test_ext(1).execute_with(|| {
        let block_number: u64 = 0;
        let netuid: u16 = 1;
        let tempo: u16 = 13;
        let hotkey_account_id = U256::from(1);
        let (nonce, work): (u64, Vec<u8>) = SubtensorModule::create_work_for_block_number(
            netuid,
            block_number,
            111111,
            &hotkey_account_id,
        );
        let coldkey_account_id = U256::from(1234);

        let new_hotkey_account_id = U256::from(2);
        let _new_colkey_account_id = U256::from(12345);

        //add network
        add_network(netuid, tempo, 0);

        // Register a neuron.
        assert_ok!(SubtensorModule::register(
            <<Test as Config>::RuntimeOrigin>::signed(hotkey_account_id),
            netuid,
            block_number,
            nonce,
            work,
            hotkey_account_id,
            coldkey_account_id
        ));

        // Get UID
        let neuron_uid = SubtensorModule::get_uid_for_net_and_hotkey(netuid, &hotkey_account_id);
        assert_ok!(neuron_uid);

        // Replace the neuron.
        SubtensorModule::replace_neuron(
            netuid,
            neuron_uid.unwrap(),
            &new_hotkey_account_id,
            block_number,
        );

        // Check old hotkey is not registered on any network.
        assert!(SubtensorModule::get_uid_for_net_and_hotkey(netuid, &hotkey_account_id).is_err());
        assert!(!SubtensorModule::is_hotkey_registered_on_any_network(
            &hotkey_account_id
        ));

        let curr_hotkey = SubtensorModule::get_hotkey_for_net_and_uid(netuid, neuron_uid.unwrap());
        assert_ok!(curr_hotkey);
        assert_ne!(curr_hotkey.unwrap(), hotkey_account_id);

        // Check new hotkey is registered on the network.
        assert!(
            SubtensorModule::get_uid_for_net_and_hotkey(netuid, &new_hotkey_account_id).is_ok()
        );
        assert!(SubtensorModule::is_hotkey_registered_on_any_network(
            &new_hotkey_account_id
        ));
        assert_eq!(curr_hotkey.unwrap(), new_hotkey_account_id);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test uids -- test_replace_neuron_multiple_subnets --exact --nocapture
#[test]
fn test_replace_neuron_multiple_subnets() {
    new_test_ext(1).execute_with(|| {
        let block_number: u64 = 0;
        let netuid: u16 = 1;
        let netuid1: u16 = 2;
        let tempo: u16 = 13;
        let hotkey_account_id = U256::from(1);
        let new_hotkey_account_id = U256::from(2);

        let (nonce, work): (u64, Vec<u8>) = SubtensorModule::create_work_for_block_number(
            netuid,
            block_number,
            111111,
            &hotkey_account_id,
        );
        let (nonce1, work1): (u64, Vec<u8>) = SubtensorModule::create_work_for_block_number(
            netuid1,
            block_number,
            111111 * 5,
            &hotkey_account_id,
        );

        let coldkey_account_id = U256::from(1234);

        let _new_colkey_account_id = U256::from(12345);

        //add network
        add_network(netuid, tempo, 0);
        add_network(netuid1, tempo, 0);

        // Register a neuron on both networks.
        assert_ok!(SubtensorModule::register(
            <<Test as Config>::RuntimeOrigin>::signed(hotkey_account_id),
            netuid,
            block_number,
            nonce,
            work,
            hotkey_account_id,
            coldkey_account_id
        ));
        assert_ok!(SubtensorModule::register(
            <<Test as Config>::RuntimeOrigin>::signed(hotkey_account_id),
            netuid1,
            block_number,
            nonce1,
            work1,
            hotkey_account_id,
            coldkey_account_id
        ));

        // Get UID
        let neuron_uid = SubtensorModule::get_uid_for_net_and_hotkey(netuid, &hotkey_account_id);
        assert_ok!(neuron_uid);

        // Verify neuron is registered on both networks.
        assert!(SubtensorModule::is_hotkey_registered_on_network(
            netuid,
            &hotkey_account_id
        ));
        assert!(SubtensorModule::is_hotkey_registered_on_network(
            netuid1,
            &hotkey_account_id
        ));
        assert!(SubtensorModule::is_hotkey_registered_on_any_network(
            &hotkey_account_id
        ));

        // Replace the neuron.
        // Only replace on ONE network.
        SubtensorModule::replace_neuron(
            netuid,
            neuron_uid.unwrap(),
            &new_hotkey_account_id,
            block_number,
        );

        // Check old hotkey is not registered on netuid network.
        assert!(SubtensorModule::get_uid_for_net_and_hotkey(netuid, &hotkey_account_id).is_err());

        // Verify still registered on netuid1 network.
        assert!(SubtensorModule::is_hotkey_registered_on_any_network(
            &hotkey_account_id
        ));
        assert!(SubtensorModule::is_hotkey_registered_on_network(
            netuid1,
            &hotkey_account_id
        ));
    });
}
