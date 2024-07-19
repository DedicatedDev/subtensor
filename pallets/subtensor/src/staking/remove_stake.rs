use super::*;

impl<T: Config> Pallet<T> {
    /// ---- The implementation for the extrinsic remove_stake: Removes stake from a hotkey account and adds it onto a coldkey.
    ///
    /// # Args:
    /// * 'origin': (<T as frame_system::Config>RuntimeOrigin):
    ///     -  The signature of the caller's coldkey.
    ///
    /// * 'hotkey' (T::AccountId):
    ///     -  The associated hotkey account.
    ///
    /// * 'stake_to_be_added' (u64):
    ///     -  The amount of stake to be added to the hotkey staking account.
    ///
    /// # Event:
    /// * StakeRemoved;
    ///     -  On the successfully removing stake from the hotkey account.
    ///
    /// # Raises:
    /// * 'NotRegistered':
    ///     -  Thrown if the account we are attempting to unstake from is non existent.
    ///
    /// * 'NonAssociatedColdKey':
    ///     -  Thrown if the coldkey does not own the hotkey we are unstaking from.
    ///
    /// * 'NotEnoughStakeToWithdraw':
    ///     -  Thrown if there is not enough stake on the hotkey to withdwraw this amount.
    ///
    /// * 'TxRateLimitExceeded':
    ///     -  Thrown if key has hit transaction rate limit
    ///
    pub fn do_remove_stake(
        origin: T::RuntimeOrigin,
        hotkey: T::AccountId,
        netuid: u16,
        alpha_to_be_removed: u64,
    ) -> dispatch::DispatchResult {
        // We check the transaction is signed by the caller and retrieve the T::AccountId coldkey information.
        let coldkey = ensure_signed(origin)?;
        log::info!(
            "do_remove_stake( origin:{:?} hotkey:{:?}, alpha_to_be_removed:{:?} )",
            coldkey,
            hotkey,
            alpha_to_be_removed
        );

        // Ensure that the hotkey account exists this is only possible through registration.
        ensure!(
            Self::hotkey_account_exists(&hotkey),
            Error::<T>::HotKeyAccountNotExists
        );

        // Ensure that the hotkey allows delegation or that the hotkey is owned by the calling coldkey.
        ensure!(
            Self::hotkey_is_delegate(&hotkey) || Self::coldkey_owns_hotkey(&coldkey, &hotkey),
            Error::<T>::HotKeyNotDelegateAndSignerNotOwnHotKey
        );

        // Ensure that the stake amount to be removed is above zero.
        ensure!(alpha_to_be_removed > 0, Error::<T>::StakeToWithdrawIsZero);

        // Ensure that the hotkey has enough stake to withdraw.
        ensure!(
            Self::has_enough_stake(&coldkey, &hotkey, alpha_to_be_removed),
            Error::<T>::NotEnoughStakeToWithdraw
        );

        // Ensure we don't exceed stake rate limit
        let unstakes_this_interval =
            Self::get_stakes_this_interval_for_coldkey_hotkey(&coldkey, &hotkey);
        ensure!(
            unstakes_this_interval < Self::get_target_stakes_per_interval(),
            Error::<T>::UnstakeRateLimitExceeded
        );

        let mechid: u16 = SubnetMechanism::<T>::get( netuid );
        let tao_unstaked: u64;
        if mechid == 2 { // STAO
            tao_unstaked = Self::alpha_to_tao( alpha_to_be_removed, netuid );
        } else { // ROOT and other.
            tao_unstaked = alpha_to_be_removed
        }

        // Increment counters.
        TotalStake::<T>::put(
            TotalStake::<T>::get().saturating_sub( tao_unstaked )
        );
        SubnetAlpha::<T>::insert(
            netuid,
            SubnetAlpha::<T>::get(netuid).saturating_sub( alpha_to_be_removed ),
        );
        SubnetTAO::<T>::insert(
            netuid,
            SubnetTAO::<T>::get(netuid).saturating_sub( tao_unstaked ),
        );
        // TotalColdkeyStake::<T>::insert(
        //     coldkey,
        //     TotalColdkeyStake::<T>::get(coldkey).saturating_sub( tao_unstaked ),
        // );
        // TotalHotkeyStake::<T>::insert(
        //     hotkey,
        //     TotalHotkeyStake::<T>::get(hotkey).saturating_sub( tao_unstaked ),
        // );
        Stake::<T>::insert(
            &hotkey,
            &coldkey,
            Stake::<T>::get( &hotkey, &coldkey ).saturating_sub( tao_unstaked ),
        );
        TotalHotkeyAlpha::<T>::insert(
            &hotkey,
            &netuid,
            TotalHotkeyAlpha::<T>::get( &hotkey, netuid ).saturating_sub( alpha_to_be_removed ),
        );
        Alpha::<T>::insert(
            (&hotkey, &coldkey, netuid),
            Alpha::<T>::get((&hotkey, &coldkey, netuid)).saturating_sub( alpha_to_be_removed ),
        );


        // We add the balance to the coldkey.  If the above fails we will not credit this coldkey.
        Self::add_balance_to_coldkey_account(&coldkey, tao_unstaked);

        // If the stake is below the minimum, we clear the nomination from storage.
        // This only applies to nominator stakes.
        // If the coldkey does not own the hotkey, it's a nominator stake.
        // TODO: add back in.
        // let new_stake = Self::get_stake_for_coldkey_and_hotkey(&coldkey, &hotkey);
        // Self::clear_small_nomination_if_required(&hotkey, &coldkey, new_stake);

        // Set last block for rate limiting
        let block: u64 = Self::get_current_block_as_u64();
        Self::set_last_tx_block(&coldkey, block);

        // Emit the unstaking event.
        Self::set_stakes_this_interval_for_coldkey_hotkey(
            &coldkey,
            &hotkey,
            unstakes_this_interval.saturating_add(1),
            block,
        );
        log::info!(
            "StakeRemoved( hotkey:{:?}, stake_to_be_removed:{:?} )",
            hotkey.clone(),
            alpha_to_be_removed
        );
        Self::deposit_event(Event::StakeRemoved(hotkey.clone(), alpha_to_be_removed));

        // Done and ok.
        Ok(())
    }
}
