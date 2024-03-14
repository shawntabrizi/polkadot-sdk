// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Test the methods generated by the Builder derive macro.
//! Tests directly on the actual Xcm struct and Instruction enum.

use xcm::latest::prelude::*;

#[test]
fn builder_pattern_works() {
	let asset: Asset = (Here, 100u128).into();
	let beneficiary: Location = AccountId32 { id: [0u8; 32], network: None }.into();
	let message: Xcm<()> = Xcm::builder_unsafe()
		.receive_teleported_asset((Here, 100u128))
		.buy_execution(asset.clone(), Unlimited)
		.deposit_asset(asset.clone(), beneficiary.clone())
		.build();
	assert_eq!(
		message,
		Xcm(vec![
			ReceiveTeleportedAsset(asset.clone().into()),
			BuyExecution { fees: asset.clone(), weight_limit: Unlimited },
			DepositAsset { assets: asset.into(), beneficiary },
		])
	);
}

#[test]
fn default_builder_requires_buy_execution() {
	let asset: Asset = (Here, 100u128).into();
	let beneficiary: Location = AccountId32 { id: [0u8; 32], network: None }.into();
	// This is invalid, since it doesn't pay for fees.
	// This is enforced by the runtime, because the build() method doesn't exist
	// on the resulting type.
	// let message: Xcm<()> = Xcm::builder()
	//     .withdraw_asset(asset.clone().into())
	//     .deposit_asset(asset.into(), beneficiary)
	//     .build();

	// To be able to do that, we need to use the explicitly unpaid variant
	let message: Xcm<()> = Xcm::builder_unpaid()
		.unpaid_execution(Unlimited, None)
		.withdraw_asset(asset.clone())
		.deposit_asset(asset.clone(), beneficiary.clone())
		.build(); // This works
	assert_eq!(
		message,
		Xcm(vec![
			UnpaidExecution { weight_limit: Unlimited, check_origin: None },
			WithdrawAsset(asset.clone().into()),
			DepositAsset { assets: asset.clone().into(), beneficiary: beneficiary.clone() },
		])
	);

	// The other option doesn't have any limits whatsoever, so it should
	// only be used when you really know what you're doing.
	let message: Xcm<()> = Xcm::builder_unsafe()
		.withdraw_asset(asset.clone())
		.deposit_asset(asset.clone(), beneficiary.clone())
		.build();
	assert_eq!(
		message,
		Xcm(vec![
			WithdrawAsset(asset.clone().into()),
			DepositAsset { assets: asset.clone().into(), beneficiary },
		])
	);
}
