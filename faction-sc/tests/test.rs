use elrond_wasm::{elrond_codec::multi_types::OptionalValue, types::Address};
use elrond_wasm_debug::{managed_address, rust_biguint, testing_framework::*, DebugApi};
use faction_sc::*;

const WASM_PATH: &'static str = "output/faction_sc.wasm";

struct ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> faction_sc::ContractObj<DebugApi>,
{
    pub blockchain_wrapper: BlockchainStateWrapper,
    pub owner_address: Address,
    pub contract_wrapper: ContractObjWrapper<faction_sc::ContractObj<DebugApi>, ContractObjBuilder>,
}

fn setup_contract<ContractObjBuilder>(
    cf_builder: ContractObjBuilder,
) -> ContractSetup<ContractObjBuilder>
where
    ContractObjBuilder: 'static + Copy + Fn() -> faction_sc::ContractObj<DebugApi>,
{
    let rust_zero = rust_biguint!(0u64);
    let mut blockchain_wrapper = BlockchainStateWrapper::new();
    let owner_address = blockchain_wrapper.create_user_account(&rust_zero);
    let cf_wrapper = blockchain_wrapper.create_sc_account(
        &rust_zero,
        Some(&owner_address),
        cf_builder,
        WASM_PATH,
    );

    blockchain_wrapper
        .execute_tx(&owner_address, &cf_wrapper, &rust_zero, |sc| {
            sc.init();
        })
        .assert_ok();

    blockchain_wrapper.add_mandos_set_account(cf_wrapper.address_ref());

    ContractSetup {
        blockchain_wrapper,
        owner_address,
        contract_wrapper: cf_wrapper,
    }
}

#[test]
fn deploy_test() {
    let _ = setup_contract(faction_sc::contract_obj);
}

#[test]
fn withdraw_card_test() {
    let mut setup = setup_contract(faction_sc::contract_obj);

    setup.blockchain_wrapper.set_nft_balance(
        setup.contract_wrapper.address_ref(),
        FACTION_NFT_ID,
        1,
        &rust_biguint!(1),
        &(),
    );

    let owner_addr = setup.owner_address.clone();
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.withdraw_card_to(managed_address!(&owner_addr), 1);
            },
        )
        .assert_ok();

    setup.blockchain_wrapper.check_nft_balance::<()>(
        &setup.owner_address,
        FACTION_NFT_ID,
        1,
        &rust_biguint!(1),
        None,
    )
}

#[test]
fn withdraw_egld_test() {
    let mut setup = setup_contract(faction_sc::contract_obj);

    setup
        .blockchain_wrapper
        .set_egld_balance(setup.contract_wrapper.address_ref(), &rust_biguint!(100));

    let owner_addr = setup.owner_address.clone();
    setup
        .blockchain_wrapper
        .execute_tx(
            &setup.owner_address,
            &setup.contract_wrapper,
            &rust_biguint!(0u64),
            |sc| {
                sc.withdraw_egld_to(managed_address!(&owner_addr), OptionalValue::None);
            },
        )
        .assert_ok();

    setup
        .blockchain_wrapper
        .check_egld_balance(&setup.owner_address, &rust_biguint!(100));
}
