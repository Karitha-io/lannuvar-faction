elrond_wasm::imports!();

pub const GENESIS_NFT_ID: &[u8] = b"REALM-579543";

mod staking_poxy {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait Staking {
        #[payable("*")]
        #[endpoint(stakeNFT)]
        fn stake_nft(&self);

        #[endpoint(claimRewards)]
        fn claim_rewards(
            &self,
            #[var_args] nonces: MultiValueEncoded<u16>,
        ) -> MultiValueEncoded<BigUint>;

        #[endpoint(unstakeNFT)]
        fn unstake_nft(
            &self,
            #[var_args] nonces: MultiValueEncoded<u16>,
        ) -> MultiValueEncoded<BigUint>;
    }
}

#[elrond_wasm::module]
pub trait StakingIntModule {
    #[proxy]
    fn staking_poxy(&self, to: ManagedAddress) -> staking_poxy::Proxy<Self::Api>;

    #[only_owner]
    #[endpoint(stakeGenesisNft)]
    fn stake_genesis_nft(&self, nonce: u64) {
        let bussiness_token_id = TokenIdentifier::from_esdt_bytes(GENESIS_NFT_ID);

        let balance = self.blockchain().get_sc_balance(&bussiness_token_id, nonce);
        require!(balance == BigUint::from(1u64), "no nft to send");

        self.staking_poxy(self.staking_address().get())
            .stake_nft()
            .add_token_transfer(bussiness_token_id, nonce, balance)
            .execute_on_dest_context_ignore_result();
    }

    #[only_owner]
    #[endpoint(unstakeGenesisNft)]
    fn unstake_genesis_nft(&self, nonce: u16) {
        let bussiness_token_id = TokenIdentifier::from_esdt_bytes(GENESIS_NFT_ID);

        self.staking_poxy(self.staking_address().get())
            .unstake_nft(MultiValueEncoded::from(ManagedVec::from_single_item(nonce)))
            .execute_on_dest_context_ignore_result();

        let balance = self
            .blockchain()
            .get_sc_balance(&bussiness_token_id, nonce as u64);

        require!(balance == BigUint::from(1u64), "did not receive nft");
    }

    #[only_owner]
    #[endpoint(claimRewardsGenesis)]
    fn claim_rewards_genesis(&self, nonce: u16) {
        self.staking_poxy(self.staking_address().get())
            .unstake_nft(MultiValueEncoded::from(ManagedVec::from_single_item(nonce)))
            .execute_on_dest_context_ignore_result();
    }

    #[only_owner]
    #[endpoint(setStakingAddress)]
    fn set_staking_address(&self, address: ManagedAddress) {
        self.staking_address().set(address);
    }

    #[view(getStakingAddress)]
    #[storage_mapper("staking_address")]
    fn staking_address(&self) -> SingleValueMapper<ManagedAddress>;
}
