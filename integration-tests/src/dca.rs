#![cfg(test)]

use crate::polkadot_test_net::*;
use frame_support::assert_ok;
use std::ops::RangeInclusive;

use crate::{assert_balance, assert_reserved_balance};
use frame_system::RawOrigin;
use hydradx_runtime::Balances;
use hydradx_runtime::Currencies;
use hydradx_runtime::Omnipool;
use hydradx_runtime::RuntimeOrigin;
use hydradx_runtime::Tokens;
use hydradx_traits::router::PoolType;
use orml_traits::MultiCurrency;
use orml_traits::MultiReservableCurrency;
use pallet_dca::types::{Order, Schedule};
use pallet_route_executor::Trade;
use polkadot_primitives::v2::BlockNumber;
use primitives::{AssetId, Balance};
use sp_runtime::traits::ConstU32;
use sp_runtime::Permill;
use sp_runtime::{BoundedVec, FixedU128};
use xcm_emulator::TestExt;
const TREASURY_ACCOUNT_INIT_BALANCE: Balance = 1000 * UNITS;
const DCA_EXECUTION_FEE: Balance = 2_735_493_790_063;
const DCA_EXECUTION_FEE_IN_LRNA: Balance = 1_367_746_897_216;

#[test]
fn create_schedule_should_work() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let block_id = 11;
		set_relaychain_block_number(block_id);

		let budget = 1000 * UNITS;
		let schedule1 = schedule_fake_with_buy_order(HDX, DAI, 100 * UNITS, budget);

		//Act
		assert_ok!(hydradx_runtime::DCA::schedule(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			schedule1,
			None
		));

		//Assert
		let schedule_id = 0;
		let schedule = hydradx_runtime::DCA::schedules(schedule_id);
		assert!(schedule.is_some());

		let next_block_id = block_id + 1;
		let schedule = hydradx_runtime::DCA::schedule_ids_per_block(next_block_id);
		assert!(!schedule.is_empty());
	});
}

#[test]
fn buy_schedule_execution_should_work_when_block_is_initialized() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let dca_budget = 1000 * UNITS;

		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE);

		let amount_out = 100 * UNITS;
		let schedule1 = schedule_fake_with_buy_order(HDX, DAI, amount_out, dca_budget);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		let amount_to_unreserve_for_trade = 143156588221183;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - amount_to_unreserve_for_trade);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + DCA_EXECUTION_FEE
		);
	});
}

#[test]
fn buy_schedule_should_be_retried_multiple_times_then_terminated() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let dca_budget = 1000 * UNITS;

		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE);

		let amount_out = 100 * UNITS;
		let schedule1 = Schedule {
			owner: AccountId::from(ALICE),
			period: 1u32,
			total_amount: dca_budget,
			order: Order::Buy {
				asset_in: HDX,
				asset_out: DAI,
				amount_out,
				max_limit: Balance::MIN,
				slippage: Some(Permill::from_percent(5)),
				route: create_bounded_vec(vec![Trade {
					pool: PoolType::Omnipool,
					asset_in: HDX,
					asset_out: DAI,
				}]),
			},
		};
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act and assert
		let schedule_id = 0;
		set_relaychain_block_number(11);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 1);

		set_relaychain_block_number(12);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - 2 * DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 2);

		set_relaychain_block_number(13);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - 3 * DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 3);

		//After this retry we terminate
		set_relaychain_block_number(14);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - 4 * DCA_EXECUTION_FEE);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);
		let schedule = hydradx_runtime::DCA::schedules(schedule_id);
		assert!(schedule.is_none());
	});
}

#[test]
fn buy_schedule_execution_should_work_when_asset_in_is_hub_asset() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let alice_init_hub_balance = 5000 * UNITS;
		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			LRNA,
			alice_init_hub_balance,
			0
		));

		let dca_budget = 2500 * UNITS;

		let amount_out = 100 * UNITS;
		let schedule1 = schedule_fake_with_buy_order(LRNA, DAI, amount_out, dca_budget);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), LRNA, alice_init_hub_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), LRNA, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		let amount_to_unreserve_for_trade = 71543186980834;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), LRNA, alice_init_hub_balance - dca_budget);
		assert_reserved_balance!(&ALICE.into(), LRNA, dca_budget - amount_to_unreserve_for_trade);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			LRNA,
			DCA_EXECUTION_FEE_IN_LRNA
		);
	});
}

#[test]
fn buy_schedule_execution_should_yield_same_result_as_direct_buy() {
	let amount_out = 100 * UNITS;

	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let dca_budget = 1000 * UNITS;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);

		let schedule1 = schedule_fake_with_buy_order(HDX, DAI, amount_out, dca_budget);
		create_schedule(ALICE, schedule1);

		//Act
		set_relaychain_block_number(11);

		//Assert
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
	});

	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE);

		//Act
		assert_ok!(hydradx_runtime::Omnipool::buy(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			DAI,
			HDX,
			amount_out,
			Balance::MAX,
		));

		//Assert
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
	});
}

#[test]
fn full_buy_dca_should_be_executed_then_completed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();

		let dca_budget = 1000 * UNITS;
		let schedule1 = schedule_fake_with_buy_order(HDX, DAI, 100 * UNITS, dca_budget);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(ALICE.into(), HDX, ALICE_INITIAL_NATIVE_BALANCE - dca_budget);
		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE
		);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		run_to_block(11, 40);

		//Assert
		let over_reservation_left_over = 138324776352520; //Because the remaining budget for the last trade is not enough, so it is returned
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + 600 * UNITS);
		assert_balance!(
			ALICE.into(),
			HDX,
			ALICE_INITIAL_NATIVE_BALANCE - dca_budget + over_reservation_left_over
		);

		assert_reserved_balance!(&ALICE.into(), HDX, 0);

		let fees = 19148456530441;
		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + fees
		);

		let schedule = hydradx_runtime::DCA::schedules(0);
		assert!(schedule.is_none());
	});
}

#[test]
fn sell_schedule_execution_should_work_when_block_is_initialized() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let dca_budget = 1100 * UNITS;
		let amount_to_sell = 100 * UNITS;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		let amount_out = 71_214_372_591_631;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - amount_to_sell - DCA_EXECUTION_FEE);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + DCA_EXECUTION_FEE
		);
	});
}

#[test]
fn sell_schedule_should_be_terminated_after_retries() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let dca_budget = 1100 * UNITS;
		let amount_to_sell = 100 * UNITS;
		let schedule1 = Schedule {
			owner: AccountId::from(ALICE),
			period: 1u32,
			total_amount: dca_budget,
			order: Order::Sell {
				asset_in: HDX,
				asset_out: DAI,
				amount_in: amount_to_sell,
				min_limit: Balance::MAX,
				slippage: Some(Permill::from_percent(1)),
				route: create_bounded_vec(vec![Trade {
					pool: PoolType::Omnipool,
					asset_in: HDX,
					asset_out: DAI,
				}]),
			},
		};
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act and Assert
		let schedule_id = 0;

		set_relaychain_block_number(11);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 1);

		set_relaychain_block_number(12);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - 2 * DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 2);

		set_relaychain_block_number(13);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget - 3 * DCA_EXECUTION_FEE);
		assert_eq!(hydradx_runtime::DCA::retries_on_error(schedule_id).unwrap(), 3);

		//At this point, the schedule will be terminated as retries max number of times
		set_relaychain_block_number(14);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - 4 * DCA_EXECUTION_FEE);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);
		let schedule = hydradx_runtime::DCA::schedules(schedule_id);
		assert!(schedule.is_none());
	});
}

#[test]
fn sell_schedule_execution_should_completed_after_one_trade_when_total_amount_is_amount_in_plus_fee() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let amount_to_sell = 1100 * UNITS;
		let dca_budget = amount_to_sell + DCA_EXECUTION_FEE;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);
		let schedule_id = 0;
		let schedule = hydradx_runtime::DCA::schedules(schedule_id);
		assert!(schedule.is_some());

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		let amount_out = 783357830013430;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + DCA_EXECUTION_FEE
		);

		let schedule = hydradx_runtime::DCA::schedules(schedule_id);
		assert!(schedule.is_none());
	});
}

#[test]
fn sell_schedule_execution_should_work_when_hub_asset_is_sold() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hub_balance = 5000 * UNITS;
		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			LRNA,
			alice_init_hub_balance,
			0
		));

		let dca_budget = 2500 * UNITS + 25 * DCA_EXECUTION_FEE_IN_LRNA;
		let amount_to_sell = 100 * UNITS;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, LRNA, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), LRNA, alice_init_hub_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), LRNA, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		let amount_out = 142499995765917;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), LRNA, alice_init_hub_balance - dca_budget);
		assert_reserved_balance!(
			&ALICE.into(),
			LRNA,
			dca_budget - amount_to_sell - DCA_EXECUTION_FEE_IN_LRNA
		);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			LRNA,
			DCA_EXECUTION_FEE_IN_LRNA
		);
	});
}

#[test]
fn sell_schedule_execution_should_yield_same_as_direct_omnipool_sell() {
	let amount_out = 71_214_372_591_631;
	let amount_to_sell = 100 * UNITS;

	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let dca_budget = 1100 * UNITS;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		set_relaychain_block_number(11);

		//Assert
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
	});

	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		//Act
		assert_ok!(hydradx_runtime::Omnipool::sell(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			HDX,
			DAI,
			amount_to_sell,
			0,
		));

		//Assert
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
	});
}

#[test]
fn full_sell_dca_should_be_executed_then_completed() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		init_omnipool_with_oracle_for_block_10();

		let number_of_trades = 11;
		let amount_to_sell = 100 * UNITS;
		let dca_budget = number_of_trades * amount_to_sell + 11 * DCA_EXECUTION_FEE;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);

		//Act
		run_to_block(11, 100);

		//Assert
		let amount_out = 783357835693308;
		let fee = number_of_trades * DCA_EXECUTION_FEE;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + fee
		);

		let schedule = hydradx_runtime::DCA::schedules(0);
		assert!(schedule.is_none());
	});
}

#[test]
fn full_sell_dca_should_be_executed_then_completed_for_multiple_users() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		let alice_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let bob_init_hdx_balance = 5000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			BOB.into(),
			bob_init_hdx_balance,
			0,
		));

		init_omnipool_with_oracle_for_block_10();

		let amount_to_sell = 100 * UNITS;
		let alice_number_of_trades = 11;
		let bob_number_of_trades = 13;
		let dca_budget = alice_number_of_trades * amount_to_sell + alice_number_of_trades * DCA_EXECUTION_FEE;
		let dca_budget_for_bob = bob_number_of_trades * amount_to_sell + bob_number_of_trades * DCA_EXECUTION_FEE;

		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		let schedule2 = schedule_fake_with_sell_order(BOB, dca_budget_for_bob, HDX, DAI, amount_to_sell);
		create_schedule(ALICE, schedule1);
		create_schedule(BOB, schedule2);

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_balance!(BOB.into(), HDX, bob_init_hdx_balance - dca_budget_for_bob);
		assert_balance!(BOB.into(), DAI, BOB_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget);
		assert_reserved_balance!(&BOB.into(), HDX, dca_budget_for_bob);

		//Act
		run_to_block(11, 100);

		//Assert
		let amount_out = 783357567338787;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);

		let amount_out = 925786041562886;

		assert_balance!(BOB.into(), DAI, BOB_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(BOB.into(), HDX, bob_init_hdx_balance - dca_budget_for_bob);
		assert_reserved_balance!(&BOB.into(), HDX, 0);

		let fee = (alice_number_of_trades + bob_number_of_trades) * DCA_EXECUTION_FEE;
		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + fee
		);

		let schedule = hydradx_runtime::DCA::schedules(0);
		assert!(schedule.is_none());

		let schedule = hydradx_runtime::DCA::schedules(1);
		assert!(schedule.is_none());
	});
}

#[test]
fn multiple_full_sell_dca_should_be_executed_then_completed_for_same_user() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		let alice_init_hdx_balance = 50000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		init_omnipool_with_oracle_for_block_10();

		//Trade 1
		let number_of_trades1 = 11;
		let amount_to_sell1 = 100 * UNITS;
		let dca_budget1 = number_of_trades1 * amount_to_sell1 + number_of_trades1 * DCA_EXECUTION_FEE;
		let schedule1 = schedule_fake_with_sell_order(ALICE, dca_budget1, HDX, DAI, amount_to_sell1);
		create_schedule(ALICE, schedule1);

		//Trade 2
		let number_of_trades2 = 15;
		let amount_to_sell2 = 125 * UNITS;
		let dca_budget2 = number_of_trades2 * amount_to_sell2 + number_of_trades2 * DCA_EXECUTION_FEE;
		let schedule2 = schedule_fake_with_sell_order(ALICE, dca_budget2, HDX, DAI, amount_to_sell2);
		create_schedule(ALICE, schedule2);

		//Trade 3
		let number_of_trades3 = 12;
		let amount_to_sell3 = 250 * UNITS;
		let dca_budget3 = number_of_trades3 * amount_to_sell3 + number_of_trades3 * DCA_EXECUTION_FEE;
		let schedule3 = schedule_fake_with_sell_order(ALICE, dca_budget3, HDX, DAI, amount_to_sell3);
		create_schedule(ALICE, schedule3);

		let budget_for_all_trades = dca_budget1 + dca_budget2 + dca_budget3;
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - budget_for_all_trades);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, budget_for_all_trades);

		//Act
		run_to_block(11, 100);

		//Assert
		let amount_out = 4255050233794072;

		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);
		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - budget_for_all_trades);
		assert_reserved_balance!(&ALICE.into(), HDX, 0);

		let fee = (number_of_trades1 + number_of_trades2 + number_of_trades3) * DCA_EXECUTION_FEE;

		assert_balance!(
			&hydradx_runtime::Treasury::account_id(),
			HDX,
			TREASURY_ACCOUNT_INIT_BALANCE + fee
		);

		let schedule = hydradx_runtime::DCA::schedules(0);
		assert!(schedule.is_none());

		let schedule = hydradx_runtime::DCA::schedules(1);
		assert!(schedule.is_none());

		let schedule = hydradx_runtime::DCA::schedules(2);
		assert!(schedule.is_none());
	});
}

#[test]
fn sca_schedules_should_be_executed_and_replanned_through_multiple_blocks_when_all_blocks_are_fully_planned() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		init_omnipool_with_oracle_for_block_10();
		let alice_init_hdx_balance = 500000000000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			alice_init_hdx_balance,
			0,
		));

		let bob_init_hdx_balance = 500000000000 * UNITS;
		assert_ok!(hydradx_runtime::Balances::set_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			BOB.into(),
			bob_init_hdx_balance,
			0,
		));

		let dca_budget = 1100000 * UNITS;
		let amount_to_sell = 100 * UNITS;
		let schedule_for_alice = schedule_fake_with_sell_order(ALICE, dca_budget, HDX, DAI, amount_to_sell);
		let schedule_for_bob = schedule_fake_with_sell_order(BOB, dca_budget, HDX, DAI, amount_to_sell);

		for _ in RangeInclusive::new(1, 60) {
			assert_ok!(hydradx_runtime::DCA::schedule(
				RuntimeOrigin::signed(ALICE.into()),
				schedule_for_alice.clone(),
				Option::Some(11)
			));
		}

		assert_balance!(ALICE.into(), HDX, alice_init_hdx_balance - dca_budget * 60);
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&ALICE.into(), HDX, dca_budget * 60);

		for _ in RangeInclusive::new(61, 120) {
			assert_ok!(hydradx_runtime::DCA::schedule(
				RuntimeOrigin::signed(BOB.into()),
				schedule_for_bob.clone(),
				Option::Some(11)
			));
		}

		assert_balance!(BOB.into(), HDX, alice_init_hdx_balance - dca_budget * 60);
		assert_balance!(BOB.into(), DAI, BOB_INITIAL_DAI_BALANCE);
		assert_reserved_balance!(&BOB.into(), HDX, dca_budget * 60);

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(11);
		assert_eq!(20, actual_schedule_ids.len());

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(12);
		assert_eq!(20, actual_schedule_ids.len());

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(14);
		assert_eq!(20, actual_schedule_ids.len());

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(18);
		assert_eq!(20, actual_schedule_ids.len());

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(26);
		assert_eq!(20, actual_schedule_ids.len());

		let actual_schedule_ids = hydradx_runtime::DCA::schedule_ids_per_block(42);
		assert_eq!(20, actual_schedule_ids.len());

		//Act
		run_to_block(11, 100);

		//Assert
		let amount_out = 84028043372832998;
		assert_balance!(ALICE.into(), DAI, ALICE_INITIAL_DAI_BALANCE + amount_out);

		let amount_out = 42725817689321764;
		assert_balance!(BOB.into(), DAI, BOB_INITIAL_DAI_BALANCE + amount_out);

		//Assert if none of the schedule is terminated
		for schedule_id in RangeInclusive::new(0, 119) {
			assert!(hydradx_runtime::DCA::schedules(schedule_id).is_some());
		}
	});
}

#[test]
fn schedules_should_be_ordered_based_on_random_number_when_executed_in_a_block() {
	TestNet::reset();
	Hydra::execute_with(|| {
		//Arrange
		let native_amount = 100000 * UNITS;
		assert_ok!(Currencies::update_balance(
			hydradx_runtime::RuntimeOrigin::root(),
			ALICE.into(),
			HDX,
			native_amount as i128,
		));

		init_omnipool_with_oracle_for_block_10();

		let schedule1 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);
		let schedule2 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);
		let schedule3 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);
		let schedule4 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);
		let schedule5 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);
		let schedule6 = schedule_fake_with_invalid_min_limit(ALICE, 1000 * UNITS, HDX, DAI, 100 * UNITS);

		create_schedule(ALICE, schedule1);
		create_schedule(ALICE, schedule2);
		create_schedule(ALICE, schedule3);
		create_schedule(ALICE, schedule4);
		create_schedule(ALICE, schedule5);
		create_schedule(ALICE, schedule6);

		//Act
		run_to_block(11, 12);

		//Assert
		//We check the random ordering based on the the emitted events.
		//The orders should fail due to invalid min limit.
		expect_schedule_ids_from_events(vec![2, 5, 0, 4, 3, 1]);
	});
}

fn create_schedule(owner: [u8; 32], schedule1: Schedule<AccountId, AssetId, u32>) {
	assert_ok!(hydradx_runtime::DCA::schedule(
		hydradx_runtime::RuntimeOrigin::signed(owner.into()),
		schedule1,
		None
	));
}

fn schedule_fake_with_buy_order(
	asset_in: AssetId,
	asset_out: AssetId,
	amount: Balance,
	budget: Balance,
) -> Schedule<AccountId, AssetId, u32> {
	Schedule {
		owner: AccountId::from(ALICE),
		period: 2u32,
		total_amount: budget,
		order: Order::Buy {
			asset_in,
			asset_out,
			amount_out: amount,
			max_limit: Balance::MAX,
			slippage: Some(Permill::from_percent(5)),
			route: create_bounded_vec(vec![Trade {
				pool: PoolType::Omnipool,
				asset_in,
				asset_out,
			}]),
		},
	}
}

fn schedule_fake_with_sell_order(
	owner: [u8; 32],
	total_amount: Balance,
	asset_in: AssetId,
	asset_out: AssetId,
	amount: Balance,
) -> Schedule<AccountId, AssetId, u32> {
	Schedule {
		owner: AccountId::from(owner),
		period: 3u32,
		total_amount,
		order: Order::Sell {
			asset_in,
			asset_out,
			amount_in: amount,
			min_limit: Balance::MIN,
			slippage: Some(Permill::from_percent(10)),
			route: create_bounded_vec(vec![Trade {
				pool: PoolType::Omnipool,
				asset_in,
				asset_out,
			}]),
		},
	}
}

fn schedule_fake_with_invalid_min_limit(
	owner: [u8; 32],
	total_amount: Balance,
	asset_in: AssetId,
	asset_out: AssetId,
	amount: Balance,
) -> Schedule<AccountId, AssetId, u32> {
	Schedule {
		owner: AccountId::from(owner),
		period: 3u32,
		total_amount,
		order: Order::Sell {
			asset_in,
			asset_out,
			amount_in: amount,
			min_limit: Balance::MAX,
			slippage: None,
			route: create_bounded_vec(vec![Trade {
				pool: PoolType::Omnipool,
				asset_in,
				asset_out,
			}]),
		},
	}
}

pub fn create_bounded_vec(trades: Vec<Trade<AssetId>>) -> BoundedVec<Trade<AssetId>, ConstU32<5>> {
	let bounded_vec: BoundedVec<Trade<AssetId>, sp_runtime::traits::ConstU32<5>> = trades.try_into().unwrap();
	bounded_vec
}

pub fn init_omnipol() {
	let native_price = FixedU128::from_float(0.5);
	let stable_price = FixedU128::from_float(0.7);
	let acc = hydradx_runtime::Omnipool::protocol_account();

	assert_ok!(hydradx_runtime::Omnipool::set_tvl_cap(RuntimeOrigin::root(), u128::MAX));

	let stable_amount: Balance = 5_000_000_000_000_000_000_000u128;
	let native_amount: Balance = 5_000_000_000_000_000_000_000u128;
	assert_ok!(Tokens::set_balance(
		RawOrigin::Root.into(),
		acc.clone(),
		DAI,
		stable_amount,
		0
	));
	assert_ok!(Currencies::update_balance(
		hydradx_runtime::RuntimeOrigin::root(),
		acc,
		HDX,
		native_amount as i128,
	));

	assert_ok!(hydradx_runtime::Omnipool::initialize_pool(
		hydradx_runtime::RuntimeOrigin::root(),
		stable_price,
		native_price,
		Permill::from_percent(60),
		Permill::from_percent(60)
	));

	assert_ok!(Balances::set_balance(
		RawOrigin::Root.into(),
		hydradx_runtime::Treasury::account_id(),
		TREASURY_ACCOUNT_INIT_BALANCE,
		0,
	));
}

fn init_omnipool_with_oracle_for_block_10() {
	init_omnipol();
	do_trade_to_populate_oracle(DAI, HDX, UNITS);
	set_relaychain_block_number(10);
	do_trade_to_populate_oracle(DAI, HDX, UNITS);
}

fn do_trade_to_populate_oracle(asset_1: AssetId, asset_2: AssetId, amount: Balance) {
	assert_ok!(Tokens::set_balance(
		RawOrigin::Root.into(),
		CHARLIE.into(),
		LRNA,
		1000000000000 * UNITS,
		0,
	));

	assert_ok!(Omnipool::sell(
		hydradx_runtime::RuntimeOrigin::signed(CHARLIE.into()),
		LRNA,
		asset_1,
		amount,
		Balance::MIN
	));

	assert_ok!(Omnipool::sell(
		hydradx_runtime::RuntimeOrigin::signed(CHARLIE.into()),
		LRNA,
		asset_2,
		amount,
		Balance::MIN
	));
}

pub fn run_to_block(from: BlockNumber, to: BlockNumber) {
	for b in from..=to {
		do_trade_to_populate_oracle(DAI, HDX, UNITS);
		set_relaychain_block_number(b);
		do_trade_to_populate_oracle(DAI, HDX, UNITS);
	}
}

pub fn expect_schedule_ids_from_events(e: Vec<u32>) {
	let last_schedule_ids_from_events: Vec<u32> = get_last_schedule_ids_from_trade_failed_events();
	pretty_assertions::assert_eq!(last_schedule_ids_from_events, e);
}

pub fn get_last_schedule_ids_from_trade_failed_events() -> Vec<u32> {
	let last_events: Vec<hydradx_runtime::RuntimeEvent> = last_hydra_events(1000);
	let mut schedule_ids = vec![];

	for event in last_events {
		let e = event.clone();
		if let hydradx_runtime::RuntimeEvent::DCA(pallet_dca::Event::TradeFailed { id, .. }) = e {
			schedule_ids.push(id);
		}
	}

	schedule_ids
}
