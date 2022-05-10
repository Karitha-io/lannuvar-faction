#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod staking_interactor;

use crate::staking_interactor::GENESIS_NFT_ID;
pub const FACTION_NFT_ID: &[u8] = b"ROKFACTION-4a5232";

#[elrond_wasm::contract]
pub trait FactionContract: crate::staking_interactor::StakingIntModule {
    #[init]
    fn init(&self) {}

    #[only_owner]
    #[endpoint(withdrawCardTo)]
    fn withdraw_card_to(&self, to: ManagedAddress, card_nonce: u64) {
        self.send().direct(
            &to,
            &TokenIdentifier::from_esdt_bytes(FACTION_NFT_ID),
            card_nonce,
            &BigUint::from(1u64),
            &[],
        )
    }

    #[only_owner]
    #[endpoint(withdrawGenesisTo)]
    fn withdraw_genesis_to(&self, to: ManagedAddress, genesis_nonce: u64) {
        self.send().direct(
            &to,
            &TokenIdentifier::from_esdt_bytes(GENESIS_NFT_ID),
            genesis_nonce,
            &BigUint::from(1u64),
            &[],
        )
    }

    #[only_owner]
    #[endpoint(withdrawEgldTo)]
    fn withdraw_egld_to(&self, to: ManagedAddress, #[var_args] amount: OptionalValue<BigUint>) {
        let amount = amount.into_option().unwrap_or_else(|| {
            self.blockchain()
                .get_balance(&self.blockchain().get_sc_address())
        });

        self.send().direct_egld(&to, &amount, &[]);
    }

    #[only_owner]
    #[endpoint(buyCardFromDeadrare)]
    fn buy_card_from_deadrare(&self, _deadrare_address: ManagedAddress, _card_nonce: u64) {
        todo!()
    }
}
