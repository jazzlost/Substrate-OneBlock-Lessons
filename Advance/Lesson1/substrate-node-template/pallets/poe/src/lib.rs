#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{*, DispatchResultWithPostInfo};
    use frame_system::{pallet_prelude::*, ensure_signed};
    // use sp_runtime::DispatchResultWithInfo;
    use sp_std::prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config
    {
        #[pallet::constant]
        type MaxClaimLength: Get<u32>;
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type Proofs<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxClaimLength>,
        (T::AccountId, T::BlockNumber),
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>
    {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoked(T::AccountId, Vec<u8>),
        ClaimTransfered(T::AccountId, T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T>
    {
        ProofAlreadyExist,
        ClaimTooLong,
        ClaimNotExist,
        NotClaimOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T>
    {
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo
        {
            /* Check Signature */
            let sender = ensure_signed(origin)?;

            /* Check proof length is valid */
            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
            
            /* Make sure not exist */
            ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist);

            /* Insert into storage */
            Proofs::<T>::insert(&bounded_claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));

            /* Post event*/
            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo
        {
            /* Check signature */
            let sender = ensure_signed(origin)?;

            /* Check proof length is valid  */
            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
            
            /* Check proof is on chain */
            let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

            /* Make sure owner */
            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            /* Remove proof */
            Proofs::<T>::remove(&bounded_claim);

            /* Post event */
            Self::deposit_event(Event::ClaimRevoked(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, dest: T::AccountId) -> DispatchResultWithPostInfo
        {
            /* Check signature */
            let sender = ensure_signed(origin)?;

            /* Check proof length is valid  */
            let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;

            /* Check proof is on chain */
            let (owner, _block_number) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;

            /* Make sure owner */
            ensure!(sender == owner, Error::<T>::NotClaimOwner);
            
            /* Replace proof owner */
            Proofs::<T>::insert(&bounded_claim, (dest.clone(), frame_system::Pallet::<T>::block_number()));

            /* Post Event */
            Self::deposit_event(Event::ClaimTransfered(sender, dest, claim));

            Ok(().into())
        }
    }
}