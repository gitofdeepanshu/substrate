use super::*;
use codec::Decode;
use frame_support::{
	storage::KeyPrefixIterator,
	traits::{tokens::nonfungibles::InspectPSP34, tokens::nonfungibles::*, Get},
	BoundedSlice,
};
use frame_system::Config as SystemConfig;
use sp_runtime::{DispatchError, DispatchResult};
use sp_std::prelude::*;


const MINT_SELECTOR: [u8; 4] = [0x1, 0x2, 0x3, 0x4];
const BURN_SELECTOR: [u8; 4] = [0x4, 0x3, 0x2, 0x1];
const TRANSFER_SELECTOR: [u8; 4] = [0x5, 0x6, 0x7, 0x8];
const OWNER_OF_SELECTOR: [u8; 4] = [0x56, 0x66, 0x7, 0x8];

impl<T: Config> InspectPSP34<<T as SystemConfig>::AccountId> for Pallet<T> {

	type ItemId = u32;
	type CollectionId = T::AccountId;

	fn owner(
        who: T::AccountId,
		collection: &Self::CollectionId,
		item: &Self::ItemId,    
	) -> Option<<T as SystemConfig>::AccountId> {
        let mut data = vec![];
		data.extend(OWNER_OF_SELECTOR);
		data.extend(item.encode());

		Self::bare_call(
			who.clone(),
			collection.clone(),
			0,
			Weight::from_all(u64::MAX),
			None,
			data,
			false,
			Determinism::Deterministic,
		).result.unwrap().data.decode::<Option<T::AccountId>>();
	}

	fn collection_owner(
        who: T::AccountId,
		_collection: &Self::CollectionId,
	) -> Option<<T as SystemConfig>::AccountId> {
		None
	}

	fn attribute(
		_collection: &Self::CollectionId,
		_item: &Self::ItemId,
		_key: &[u8],
	) -> Option<Vec<u8>> {
		None
	}

	fn collection_attribute(who: T::AccountId, _collection: &Self::CollectionId, _key: &[u8]) -> Option<Vec<u8>> {
		None
	}

	fn can_transfer(who: T::AccountId, _collection: &Self::CollectionId, _item: &Self::ItemId) -> bool {
		true
	}

	fn typed_attribute<K: Encode, V: codec::Decode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
	) -> Option<V> {
		key.using_encoded(|d| Self::attribute(collection, item, d))
			.and_then(|v| V::decode(&mut &v[..]).ok())
	}

	fn typed_collection_attribute<K: Encode, V: Decode>(
		who: T::AccountId,
		collection: &Self::CollectionId,
		key: &K,
	) -> Option<V> {
		key.using_encoded(|d| Self::collection_attribute(who,collection, d))
			.and_then(|v| V::decode(&mut &v[..]).ok())
	}
}

impl<T: Config> MutatePSP34<<T as SystemConfig>::AccountId> for Pallet<T> {
	fn mint_into(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		who: &T::AccountId,
	) -> DispatchResult {
		let mut data = vec![];  
		data.extend(MINT_SELECTOR);
		data.extend(who.encode());
		data.extend(item.encode());

		Self::bare_call(
			who.clone(),
			collection.clone(),
			0,
			Weight::from_all(u64::MAX),
			None,
			data,
			false,
			Determinism::Deterministic,
		);
		Ok(())
	}

	fn burn(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		maybe_check_owner: &T::AccountId,
	) -> DispatchResult {
        let mut data = vec![];
		data.extend(BURN_SELECTOR);
		data.extend(item.encode());

		Self::bare_call(
			maybe_check_owner.clone(),
			collection.clone(),
			0,
			Weight::from_all(u64::MAX),
			None,
			data,
			false,
			Determinism::Deterministic,
		);
		Ok(())
		
	}

	fn set_attribute(
		_collection: &Self::CollectionId,
		_item: &Self::ItemId,
		_key: &[u8],
		_value: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		Err(sp_runtime::TokenError::Unsupported.into())
	}

	fn set_typed_attribute<K: Encode, V: Encode>(
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		key: &K,
		value: &V,
	) -> frame_support::pallet_prelude::DispatchResult {
		key.using_encoded(|k| value.using_encoded(|v| Self::set_attribute(collection, item, k, v)))
	}

	fn set_collection_attribute(
		_collection: &Self::CollectionId,
		_key: &[u8],
		_value: &[u8],
	) -> frame_support::pallet_prelude::DispatchResult {
		Err(sp_runtime::TokenError::Unsupported.into())
	}

	fn set_typed_collection_attribute<K: Encode, V: Encode>(
		collection: &Self::CollectionId,
		key: &K,
		value: &V,
	) -> frame_support::pallet_prelude::DispatchResult {
		key.using_encoded(|k| {
			value.using_encoded(|v| Self::set_collection_attribute(collection, k, v))
		})
	}
}

impl<T: Config> TransferPSP34<T::AccountId> for Pallet<T> {
	fn transfer(
        who: &T::AccountId,
		collection: &Self::CollectionId,
		item: &Self::ItemId,
		destination: &T::AccountId,
	) -> DispatchResult {
        let mut data = vec![];
		data.extend(TRANSFER_SELECTOR);
		data.extend(destination.encode());
		data.extend(item.encode());

		Self::bare_call(
			who.clone(),
			collection.clone(),
			0,
			Weight::from_all(u64::MAX),
			None,
			data,
			false,
			Determinism::Deterministic,
		);
		Ok(())
		// Self::do_transfer(*collection, *item, destination.clone(), |_, _| Ok(()))
	}
}
