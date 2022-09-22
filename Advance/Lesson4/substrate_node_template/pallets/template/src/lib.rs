#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use serde::{Deserialize, Deserializer};
	use sp_io::offchain_index;
	use sp_runtime::offchain::storage::StorageValueRef;
	use sp_std::vec::Vec;

	/* Indexing data struct */
	#[derive(Debug, Deserialize, Encode, Decode, Default)]
	struct IndexingData(#[serde(deserialize_with = "de_string_to_bytes")] Vec<u8>, u32);

	/* Deserialize method */
	pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: &str = Deserialize::deserialize(de)?;
		Ok(s.as_bytes().to_vec())
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10_000)]
		pub fn extrinsic(origin: OriginFor<T>, number: u32) -> DispatchResult {
			/* Check signature */
			let _who = ensure_signed(origin)?;

			/* Get block number */
			let block_number = frame_system::Pallet::<T>::block_number();
			/* Generate store key */
			let key = Self::derived_key(block_number.clone());

			/* Data to index */
			let data = IndexingData(b"my_offchain_storage".to_vec(), number);
			/* Store data with offchain indexing */
			offchain_index::set(&key, &data.encode());

			log::info!("extrinsic set local storage key: {:?}, data: {:?}, block number: {:?}", key, &data.encode(), block_number);
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	{
		fn offchain_worker(block_number: T::BlockNumber)
		{
			/* Generate store key from block number */
			let key = Self::derived_key(block_number - 1u32.into());

			/* Create persistent storage reference */
			let storage_ref = StorageValueRef::persistent(&key);

			/* Get indexing data */
			if let Ok(Some(data)) = storage_ref.get::<IndexingData>()
			{
				log::info!("Get local storage data: {:?}, block number {:?} ", data.1, block_number);
			}
			else 
			{
				log::info!("Error reading from local storage. Block number: {:?}", block_number);
			}
		}
	}

	impl<T: Config> Pallet<T> {

		#[deny(clippy::clone_double_ref)]
		fn derived_key(block_number: T::BlockNumber) -> Vec<u8> {
			block_number.using_encoded(|encoded_bn| {
				b"node-template::indexing::"
					.iter()
					.chain(encoded_bn)
					.copied()
					.collect::<Vec<u8>>()
			})
		}
	}
}
