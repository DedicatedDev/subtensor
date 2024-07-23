#![allow(unused, clippy::indexing_slicing, clippy::panic, clippy::unwrap_used)]
use codec::Encode;
use frame_support::weights::Weight;
use frame_support::{assert_err, assert_noop, assert_ok};
use frame_system::{Config, RawOrigin};
mod mock;
use mock::*;
use pallet_subtensor::*;
use sp_core::H256;
use sp_core::U256;

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_total_hotkey_coldkey_stakes_this_interval --exact --nocapture
#[test]
fn test_swap_total_hotkey_coldkey_stakes_this_interval() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);
        let stake = 100;
        let block = 42;

        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);
        TotalHotkeyColdkeyStakesThisInterval::<Test>::insert(hotkey, old_coldkey, (stake, block));

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert!(!TotalHotkeyColdkeyStakesThisInterval::<Test>::contains_key(
            hotkey,
            old_coldkey
        ));
        assert_eq!(
            TotalHotkeyColdkeyStakesThisInterval::<Test>::get(hotkey, new_coldkey),
            (stake, block)
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_subnet_owner --exact --nocapture
#[test]
fn test_swap_subnet_owner() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let netuid = 1u16;

        add_network(netuid, 1, 0);
        SubnetOwner::<Test>::insert(netuid, old_coldkey);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(SubnetOwner::<Test>::get(netuid), new_coldkey);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_stake --exact --nocapture
#[test]
fn test_swap_stake() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);
        let stake = 100;

        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);
        Stake::<Test>::insert(hotkey, old_coldkey, stake);
        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert!(!Stake::<Test>::contains_key(hotkey, old_coldkey));
        assert_eq!(Stake::<Test>::get(hotkey, new_coldkey), stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_total_coldkey_stake --exact --nocapture
#[test]
fn test_swap_total_coldkey_stake() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let stake = 100;

        TotalColdkeyStake::<Test>::insert(old_coldkey, stake);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_staking_hotkeys --exact --nocapture
#[test]
fn test_swap_staking_hotkeys() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);

        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert!(StakingHotkeys::<Test>::get(old_coldkey).is_empty());
        assert_eq!(StakingHotkeys::<Test>::get(new_coldkey), vec![hotkey]);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_hotkey_owners --exact --nocapture
#[test]
fn test_swap_hotkey_owners() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);

        Owner::<Test>::insert(hotkey, old_coldkey);
        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(Owner::<Test>::get(hotkey), new_coldkey);
        assert!(OwnedHotkeys::<Test>::get(old_coldkey).is_empty());
        assert_eq!(OwnedHotkeys::<Test>::get(new_coldkey), vec![hotkey]);
    });
}
// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_transfer_remaining_balance --exact --nocapture
#[test]
fn test_transfer_remaining_balance() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let balance = 100;

        SubtensorModule::add_balance_to_coldkey_account(&old_coldkey, balance);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(SubtensorModule::get_coldkey_balance(&old_coldkey), 0);
        assert_eq!(SubtensorModule::get_coldkey_balance(&new_coldkey), balance);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_no_stake --exact --nocapture
#[test]
fn test_swap_with_no_stake() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), 0);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_multiple_hotkeys --exact --nocapture
#[test]
fn test_swap_with_multiple_hotkeys() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);

        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert!(OwnedHotkeys::<Test>::get(old_coldkey).is_empty());
        assert_eq!(
            OwnedHotkeys::<Test>::get(new_coldkey),
            vec![hotkey1, hotkey2]
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_multiple_subnets --exact --nocapture
#[test]
fn test_swap_with_multiple_subnets() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let netuid1 = 1u16;
        let netuid2 = 2u16;

        add_network(netuid1, 1, 0);
        add_network(netuid2, 1, 0);
        SubnetOwner::<Test>::insert(netuid1, old_coldkey);
        SubnetOwner::<Test>::insert(netuid2, old_coldkey);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(SubnetOwner::<Test>::get(netuid1), new_coldkey);
        assert_eq!(SubnetOwner::<Test>::get(netuid2), new_coldkey);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_zero_balance --exact --nocapture
#[test]
fn test_swap_with_zero_balance() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(Balances::free_balance(old_coldkey), 0);
        assert_eq!(Balances::free_balance(new_coldkey), 0);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_idempotency --exact --nocapture
#[test]
fn test_swap_idempotency() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let stake = 100;

        TotalColdkeyStake::<Test>::insert(old_coldkey, stake);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_max_values --exact --nocapture
#[test]
fn test_swap_with_max_values() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let max_stake = u64::MAX;

        TotalColdkeyStake::<Test>::insert(old_coldkey, max_stake);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), max_stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_non_existent_new_coldkey --exact --nocapture
#[test]
fn test_swap_with_non_existent_new_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let stake = 100;

        TotalColdkeyStake::<Test>::insert(old_coldkey, stake);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_overflow_in_stake_addition --exact --nocapture
#[test]
fn test_swap_with_overflow_in_stake_addition() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let max_stake = u64::MAX;

        TotalColdkeyStake::<Test>::insert(old_coldkey, max_stake);
        TotalColdkeyStake::<Test>::insert(new_coldkey, 1);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), max_stake);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_max_hotkeys --exact --nocapture
#[test]
fn test_swap_with_max_hotkeys() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let max_hotkeys = 1000;
        let hotkeys: Vec<U256> = (0..max_hotkeys).map(U256::from).collect();

        OwnedHotkeys::<Test>::insert(old_coldkey, hotkeys.clone());

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert!(OwnedHotkeys::<Test>::get(old_coldkey).is_empty());
        assert_eq!(OwnedHotkeys::<Test>::get(new_coldkey), hotkeys);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_effect_on_delegated_stake --exact --nocapture
#[test]
fn test_swap_effect_on_delegated_stake() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let delegator = U256::from(3);
        let hotkey = U256::from(4);
        let stake = 100;

        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);
        StakingHotkeys::<Test>::insert(delegator, vec![hotkey]);
        Stake::<Test>::insert(hotkey, old_coldkey, stake);
        Stake::<Test>::insert(hotkey, delegator, stake);

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(Stake::<Test>::get(hotkey, new_coldkey), stake);
        assert_eq!(Stake::<Test>::get(hotkey, delegator), stake);
        assert_eq!(Stake::<Test>::get(hotkey, old_coldkey), 0);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_concurrent_modifications --exact --nocapture
#[test]
fn test_swap_concurrent_modifications() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);
        let netuid: u16 = 1;
        let initial_stake = 100;
        let additional_stake = 50;

        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);
        Stake::<Test>::insert(hotkey, old_coldkey, initial_stake);

        // Simulate concurrent stake addition
        add_network(netuid, 1, 1);
        SubtensorModule::add_balance_to_coldkey_account(&new_coldkey, additional_stake);
        register_ok_neuron(netuid, hotkey, new_coldkey, 1001000);
        assert_ok!(SubtensorModule::add_stake(
            <<Test as Config>::RuntimeOrigin>::signed(new_coldkey),
            hotkey,
            netuid,
            additional_stake
        ));

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        assert_eq!(
            Stake::<Test>::get(hotkey, new_coldkey),
            initial_stake + additional_stake - 1
        );
        assert!(!Stake::<Test>::contains_key(hotkey, old_coldkey));
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_swap_with_invalid_subnet_ownership --exact --nocapture
#[test]
fn test_swap_with_invalid_subnet_ownership() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let netuid = 1u16;

        SubnetOwner::<Test>::insert(netuid, old_coldkey);

        // Simulate an invalid state where the subnet owner doesn't match the old_coldkey
        SubnetOwner::<Test>::insert(netuid, U256::from(3));

        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &old_coldkey,
            &new_coldkey,
            &mut weight
        ));

        // The swap should not affect the mismatched subnet ownership
        assert_eq!(SubnetOwner::<Test>::get(netuid), U256::from(3));
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=debug cargo test --test swap_coldkey -- test_do_swap_coldkey_success --exact --nocapture
#[test]
fn test_do_swap_coldkey_success() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);
        let netuid = 1u16;
        let stake_amount1 = 1000u64;
        let stake_amount2 = 2000u64;
        let swap_cost = SubtensorModule::get_key_swap_cost();
        let free_balance_old = 12345u64 + swap_cost;

        // Setup initial state
        add_network(netuid, 13, 0);
        register_ok_neuron(netuid, hotkey1, old_coldkey, 0);
        register_ok_neuron(netuid, hotkey2, old_coldkey, 0);

        // Add balance to old coldkey
        SubtensorModule::add_balance_to_coldkey_account(
            &old_coldkey,
            stake_amount1 + stake_amount2 + free_balance_old,
        );

        // Log initial state
        log::info!(
            "Initial total stake: {}",
            SubtensorModule::get_total_stake()
        );
        log::info!(
            "Initial old coldkey stake: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&old_coldkey, netuid)
        );
        log::info!(
            "Initial new coldkey stake: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&new_coldkey, netuid)
        );

        // Add stake to the neurons
        assert_ok!(SubtensorModule::add_stake(
            <<Test as Config>::RuntimeOrigin>::signed(old_coldkey),
            hotkey1,
            netuid,
            stake_amount1
        ));
        assert_ok!(SubtensorModule::add_stake(
            <<Test as Config>::RuntimeOrigin>::signed(old_coldkey),
            hotkey2,
            netuid,
            stake_amount2
        ));

        // Log state after adding stake
        log::info!(
            "Total stake after adding: {}",
            SubtensorModule::get_total_stake()
        );
        log::info!(
            "Old coldkey stake after adding: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&old_coldkey, netuid)
        );
        log::info!(
            "New coldkey stake after adding: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&new_coldkey, netuid)
        );

        // Record total stake before swap
        let total_stake_before_swap = SubtensorModule::get_total_stake();

        // Perform the swap
        assert_ok!(SubtensorModule::do_swap_coldkey(
            <<Test as Config>::RuntimeOrigin>::signed(old_coldkey),
            &new_coldkey
        ));

        // Log state after swap
        log::info!(
            "Total stake after swap: {}",
            SubtensorModule::get_total_stake()
        );
        log::info!(
            "Old coldkey stake after swap: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&old_coldkey, netuid)
        );
        log::info!(
            "New coldkey stake after swap: {}",
            SubtensorModule::get_stake_for_coldkey_on_subnet(&new_coldkey, netuid)
        );

        // Verify the swap
        assert_eq!(Owner::<Test>::get(hotkey1), new_coldkey);
        assert_eq!(Owner::<Test>::get(hotkey2), new_coldkey);
        assert_eq!(
            TotalColdkeyStake::<Test>::get(new_coldkey),
            stake_amount1 + stake_amount2
        );
        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);
        assert_eq!(Stake::<Test>::get(hotkey1, new_coldkey), stake_amount1);
        assert_eq!(Stake::<Test>::get(hotkey2, new_coldkey), stake_amount2);
        assert!(!Stake::<Test>::contains_key(hotkey1, old_coldkey));
        assert!(!Stake::<Test>::contains_key(hotkey2, old_coldkey));

        // Verify OwnedHotkeys
        let new_owned_hotkeys = OwnedHotkeys::<Test>::get(new_coldkey);
        assert!(new_owned_hotkeys.contains(&hotkey1));
        assert!(new_owned_hotkeys.contains(&hotkey2));
        assert_eq!(new_owned_hotkeys.len(), 2);
        assert!(!OwnedHotkeys::<Test>::contains_key(old_coldkey));

        // Verify balance transfer
        assert_eq!(
            SubtensorModule::get_coldkey_balance(&new_coldkey),
            free_balance_old - swap_cost
        );
        assert_eq!(SubtensorModule::get_coldkey_balance(&old_coldkey), 0);

        // Verify total stake remains unchanged
        assert_eq!(
            SubtensorModule::get_total_stake(),
            total_stake_before_swap,
            "Total stake changed unexpectedly"
        );

        // Verify event emission
        System::assert_last_event(
            Event::ColdkeySwapped {
                old_coldkey,
                new_coldkey,
            }
            .into(),
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap -- test_swap_stake_for_coldkey --exact --nocaptur
#[test]
fn test_swap_stake_for_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);
        let stake_amount1 = 1000u64;
        let stake_amount2 = 2000u64;
        let stake_amount3 = 3000u64;
        let total_stake = stake_amount1 + stake_amount2;
        let mut weight = Weight::zero();

        // Setup initial state
        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);
        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);
        Stake::<Test>::insert(hotkey1, old_coldkey, stake_amount1);
        Stake::<Test>::insert(hotkey2, old_coldkey, stake_amount2);
        assert_eq!(Stake::<Test>::get(hotkey1, old_coldkey), stake_amount1);
        assert_eq!(Stake::<Test>::get(hotkey1, old_coldkey), stake_amount1);

        // Insert existing for same hotkey1
        Stake::<Test>::insert(hotkey1, new_coldkey, stake_amount3);
        StakingHotkeys::<Test>::insert(new_coldkey, vec![hotkey1]);

        TotalHotkeyStake::<Test>::insert(hotkey1, stake_amount1);
        TotalHotkeyStake::<Test>::insert(hotkey2, stake_amount2);
        TotalColdkeyStake::<Test>::insert(old_coldkey, total_stake);

        // Set up total issuance
        TotalIssuance::<Test>::put(total_stake);
        TotalStake::<Test>::put(total_stake);

        // Record initial values
        let initial_total_issuance = SubtensorModule::get_total_issuance();
        let initial_total_stake = SubtensorModule::get_total_stake();

        // Perform the swap
        SubtensorModule::perform_swap_coldkey(&old_coldkey, &new_coldkey, &mut weight);

        // Verify stake is additive, not replaced
        assert_eq!(
            Stake::<Test>::get(hotkey1, new_coldkey),
            stake_amount1 + stake_amount3
        );

        // Verify ownership transfer
        assert_eq!(
            SubtensorModule::get_owned_hotkeys(&new_coldkey),
            vec![hotkey1, hotkey2]
        );
        assert_eq!(SubtensorModule::get_owned_hotkeys(&old_coldkey), vec![]);

        // Verify stake transfer
        assert_eq!(Stake::<Test>::get(hotkey2, new_coldkey), stake_amount2);
        assert_eq!(Stake::<Test>::get(hotkey1, old_coldkey), 0);
        assert_eq!(Stake::<Test>::get(hotkey2, old_coldkey), 0);

        // Verify TotalColdkeyStake
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), total_stake);
        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);

        // Verify TotalHotkeyStake remains unchanged
        assert_eq!(TotalHotkeyStake::<Test>::get(hotkey1), stake_amount1);
        assert_eq!(TotalHotkeyStake::<Test>::get(hotkey2), stake_amount2);

        // Verify total stake and issuance remain unchanged
        assert_eq!(
            SubtensorModule::get_total_stake(),
            initial_total_stake,
            "Total stake changed unexpectedly"
        );
        assert_eq!(
            SubtensorModule::get_total_issuance(),
            initial_total_issuance,
            "Total issuance changed unexpectedly"
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_swap_staking_hotkeys_for_coldkey --exact --nocapture
#[test]
fn test_swap_staking_hotkeys_for_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);
        let stake_amount1 = 1000u64;
        let stake_amount2 = 2000u64;
        let total_stake = stake_amount1 + stake_amount2;
        let mut weight = Weight::zero();

        // Setup initial state
        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);
        Stake::<Test>::insert(hotkey1, old_coldkey, stake_amount1);
        Stake::<Test>::insert(hotkey2, old_coldkey, stake_amount2);
        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);
        TotalHotkeyStake::<Test>::insert(hotkey1, stake_amount1);
        TotalHotkeyStake::<Test>::insert(hotkey2, stake_amount2);
        TotalColdkeyStake::<Test>::insert(old_coldkey, total_stake);

        // Set up total issuance
        TotalIssuance::<Test>::put(total_stake);
        TotalStake::<Test>::put(total_stake);

        // Perform the swap
        SubtensorModule::perform_swap_coldkey(&old_coldkey, &new_coldkey, &mut weight);

        // Verify StakingHotkeys transfer
        assert_eq!(
            StakingHotkeys::<Test>::get(new_coldkey),
            vec![hotkey1, hotkey2]
        );
        assert_eq!(StakingHotkeys::<Test>::get(old_coldkey), vec![]);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_swap_delegated_stake_for_coldkey --exact --nocapture
#[test]
fn test_swap_delegated_stake_for_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);
        let stake_amount1 = 1000u64;
        let stake_amount2 = 2000u64;
        let total_stake = stake_amount1 + stake_amount2;
        let mut weight = Weight::zero();

        // Notice hotkey1 and hotkey2 are not in OwnedHotkeys
        // coldkey therefore delegates stake to them

        // Setup initial state
        StakingHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);
        Stake::<Test>::insert(hotkey1, old_coldkey, stake_amount1);
        Stake::<Test>::insert(hotkey2, old_coldkey, stake_amount2);
        TotalHotkeyStake::<Test>::insert(hotkey1, stake_amount1);
        TotalHotkeyStake::<Test>::insert(hotkey2, stake_amount2);
        TotalColdkeyStake::<Test>::insert(old_coldkey, total_stake);

        // Set up total issuance
        TotalIssuance::<Test>::put(total_stake);
        TotalStake::<Test>::put(total_stake);

        // Record initial values
        let initial_total_issuance = SubtensorModule::get_total_issuance();
        let initial_total_stake = SubtensorModule::get_total_stake();

        // Perform the swap
        SubtensorModule::perform_swap_coldkey(&old_coldkey, &new_coldkey, &mut weight);

        // Verify stake transfer
        assert_eq!(Stake::<Test>::get(hotkey1, new_coldkey), stake_amount1);
        assert_eq!(Stake::<Test>::get(hotkey2, new_coldkey), stake_amount2);
        assert_eq!(Stake::<Test>::get(hotkey1, old_coldkey), 0);
        assert_eq!(Stake::<Test>::get(hotkey2, old_coldkey), 0);

        // Verify TotalColdkeyStake
        assert_eq!(TotalColdkeyStake::<Test>::get(new_coldkey), total_stake);
        assert_eq!(TotalColdkeyStake::<Test>::get(old_coldkey), 0);

        // Verify TotalHotkeyStake remains unchanged
        assert_eq!(TotalHotkeyStake::<Test>::get(hotkey1), stake_amount1);
        assert_eq!(TotalHotkeyStake::<Test>::get(hotkey2), stake_amount2);

        // Verify total stake and issuance remain unchanged
        assert_eq!(
            SubtensorModule::get_total_stake(),
            initial_total_stake,
            "Total stake changed unexpectedly"
        );
        assert_eq!(
            SubtensorModule::get_total_issuance(),
            initial_total_issuance,
            "Total issuance changed unexpectedly"
        );
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_swap_total_hotkey_coldkey_stakes_this_interval_for_coldkey --exact --nocapture
#[test]
fn test_swap_total_hotkey_coldkey_stakes_this_interval_for_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey1 = U256::from(3);
        let hotkey2 = U256::from(4);
        let stake1 = (1000u64, 100u64);
        let stake2 = (2000u64, 200u64);
        let mut weight = Weight::zero();

        // Initialize TotalHotkeyColdkeyStakesThisInterval for old_coldkey
        TotalHotkeyColdkeyStakesThisInterval::<Test>::insert(hotkey1, old_coldkey, stake1);
        TotalHotkeyColdkeyStakesThisInterval::<Test>::insert(hotkey2, old_coldkey, stake2);

        // Populate OwnedHotkeys map
        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey1, hotkey2]);

        // Perform the swap
        SubtensorModule::perform_swap_coldkey(&old_coldkey, &new_coldkey, &mut weight);

        // Verify the swap
        assert_eq!(
            TotalHotkeyColdkeyStakesThisInterval::<Test>::get(hotkey1, new_coldkey),
            stake1
        );
        assert_eq!(
            TotalHotkeyColdkeyStakesThisInterval::<Test>::get(hotkey2, new_coldkey),
            stake2
        );
        assert!(!TotalHotkeyColdkeyStakesThisInterval::<Test>::contains_key(
            old_coldkey,
            hotkey1
        ));
        assert!(!TotalHotkeyColdkeyStakesThisInterval::<Test>::contains_key(
            old_coldkey,
            hotkey2
        ));
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_swap_subnet_owner_for_coldkey --exact --nocapture
#[test]
fn test_swap_subnet_owner_for_coldkey() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let netuid1 = 1u16;
        let netuid2 = 2u16;
        let mut weight = Weight::zero();

        // Initialize SubnetOwner for old_coldkey
        add_network(netuid1, 13, 0);
        add_network(netuid2, 14, 0);
        SubnetOwner::<Test>::insert(netuid1, old_coldkey);
        SubnetOwner::<Test>::insert(netuid2, old_coldkey);

        // Set up TotalNetworks
        TotalNetworks::<Test>::put(3);

        // Perform the swap
        SubtensorModule::perform_swap_coldkey(&old_coldkey, &new_coldkey, &mut weight);

        // Verify the swap
        assert_eq!(SubnetOwner::<Test>::get(netuid1), new_coldkey);
        assert_eq!(SubnetOwner::<Test>::get(netuid2), new_coldkey);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_do_swap_coldkey_with_subnet_ownership --exact --nocapture
#[test]
fn test_do_swap_coldkey_with_subnet_ownership() {
    new_test_ext(1).execute_with(|| {
        let old_coldkey = U256::from(1);
        let new_coldkey = U256::from(2);
        let hotkey = U256::from(3);
        let netuid = 1u16;
        let stake_amount: u64 = 1000u64;
        let swap_cost = SubtensorModule::get_key_swap_cost();

        // Setup initial state
        add_network(netuid, 13, 0);
        register_ok_neuron(netuid, hotkey, old_coldkey, 0);

        // Set TotalNetworks because swap relies on it
        pallet_subtensor::TotalNetworks::<Test>::set(1);

        SubtensorModule::add_balance_to_coldkey_account(&old_coldkey, stake_amount + swap_cost);
        SubnetOwner::<Test>::insert(netuid, old_coldkey);

        // Populate OwnedHotkeys map
        OwnedHotkeys::<Test>::insert(old_coldkey, vec![hotkey]);

        // Perform the swap
        assert_ok!(SubtensorModule::do_swap_coldkey(
            <<Test as Config>::RuntimeOrigin>::signed(old_coldkey),
            &new_coldkey
        ));

        // Verify subnet ownership transfer
        assert_eq!(SubnetOwner::<Test>::get(netuid), new_coldkey);
    });
}
// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_coldkey_has_associated_hotkeys --exact --nocapture
#[test]
fn test_coldkey_has_associated_hotkeys() {
    new_test_ext(1).execute_with(|| {
        let coldkey = U256::from(1);
        let hotkey = U256::from(2);
        let netuid = 1u16;

        // Setup initial state
        add_network(netuid, 13, 0);
        register_ok_neuron(netuid, hotkey, coldkey, 0);
        SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_swap_senate_member --exact --nocapture
#[test]
fn test_swap_senate_member() {
    new_test_ext(1).execute_with(|| {
        let old_hotkey = U256::from(1);
        let new_hotkey = U256::from(2);
        let non_member_hotkey = U256::from(3);
        let mut weight = Weight::zero();

        // Setup: Add old_hotkey as a Senate member
        assert_ok!(SenateMembers::add_member(
            RawOrigin::Root.into(),
            old_hotkey
        ));

        // Test 1: Successful swap
        assert_ok!(SubtensorModule::swap_senate_member(
            &old_hotkey,
            &new_hotkey,
            &mut weight
        ));
        assert!(Senate::is_member(&new_hotkey));
        assert!(!Senate::is_member(&old_hotkey));

        // Verify weight update
        let expected_weight = <Test as frame_system::Config>::DbWeight::get().reads_writes(2, 2);
        assert_eq!(weight, expected_weight);

        // Reset weight for next test
        weight = Weight::zero();

        // Test 2: Swap with non-member (should not change anything)
        assert_ok!(SubtensorModule::swap_senate_member(
            &non_member_hotkey,
            &new_hotkey,
            &mut weight
        ));
        assert!(Senate::is_member(&new_hotkey));
        assert!(!Senate::is_member(&non_member_hotkey));

        // Verify weight update (should only have read operations)
        let expected_weight = <Test as frame_system::Config>::DbWeight::get().reads(1);
        assert_eq!(weight, expected_weight);
    });
}

// SKIP_WASM_BUILD=1 RUST_LOG=info cargo test --test swap_coldkey -- test_coldkey_delegations --exact --nocapture
#[test]
fn test_coldkey_delegations() {
    new_test_ext(1).execute_with(|| {
        let new_coldkey = U256::from(0);
        let owner = U256::from(1);
        let coldkey = U256::from(4);
        let delegate = U256::from(2);
        let netuid = 1u16;
        add_network(netuid, 13, 0);
        register_ok_neuron(netuid, delegate, owner, 0);
        SubtensorModule::add_balance_to_coldkey_account(&coldkey, 1000);
        assert_ok!(SubtensorModule::do_become_delegate(
            <<Test as Config>::RuntimeOrigin>::signed(owner),
            delegate,
            u16::MAX / 10
        ));
        assert_ok!(SubtensorModule::add_stake(
            <<Test as Config>::RuntimeOrigin>::signed(coldkey),
            delegate,
            netuid,
            100
        ));
        let mut weight = Weight::zero();
        assert_ok!(SubtensorModule::perform_swap_coldkey(
            &coldkey,
            &new_coldkey,
            &mut weight
        ));
        assert_eq!(SubtensorModule::get_stake_for_hotkey_on_subnet(&delegate, netuid), 100);
        assert_eq!(SubtensorModule::get_stake_for_coldkey_on_subnet(&coldkey, netuid), 0);
        assert_eq!(
            SubtensorModule::get_stake_for_coldkey_on_subnet(&new_coldkey, netuid),
            100
        );
        assert_eq!(Stake::<Test>::get(delegate, new_coldkey), 100);
        assert_eq!(Stake::<Test>::get(delegate, coldkey), 0);
    });
}
