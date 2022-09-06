#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::{*, DispatchResultWithPostInfo}};
    use frame_system::{pallet_prelude::{*, OriginFor}, ensure_signed};
    use frame_support::traits::{Randomness, Currency, ReservableCurrency};
    use sp_io::hashing::blake2_128;
    use sp_std::{fmt::Debug};
    use sp_runtime::traits::{Bounded, AtLeast32BitUnsigned};

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen,)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::config]
    pub trait Config: frame_system::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        
        type KittyIndex: Parameter + AtLeast32BitUnsigned + Default + Copy + Bounded + MaxEncodedLen;

        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        #[pallet::constant]
        type Reserved: Get<BalanceOf<Self>>;
    }

    #[pallet::type_value]
    pub fn GetDefaultValue<T: Config>() -> T::KittyIndex
    {
        <T::KittyIndex as Default>::default()
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /* Keep track all kitties */
    #[pallet::storage]
    #[pallet::getter(fn kitty_count)]
    pub type KittyCount<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery, GetDefaultValue<T>>;

    /* Storage for all kitties with index */
    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Kitty>;

    /* Storage for all kitties of owner */
    #[pallet::storage]
    #[pallet::getter(fn kitty_owner)]
    pub type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, T::AccountId>;

    /* Kitties sale list, none means not for sale */
    #[pallet::storage]
	#[pallet::getter(fn sale_list)]
	pub type SaleList<T: Config> = StorageMap<_, Blake2_128Concat, T::KittyIndex, Option<BalanceOf<T>>, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config>
    {
        KittyCreated(T::AccountId, T::KittyIndex, Kitty),
        KittyBreed(T::AccountId, T::KittyIndex, Kitty),
        KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
        KittyOnSale(T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
        KittySaled(T::AccountId, T::AccountId, T::KittyIndex, Option<BalanceOf<T>>),
    }

    #[pallet::error]
    pub enum Error<T>
    {
        InvalidKittyId,
        SameKittyId,
        NotOwner,
        NotEnoughBalanceReserved,
        AlreadyOwned,
        NotForSale,
        NotEnoughBalanceBuy,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T>
    {
        #[pallet::weight(10_000)]
        pub fn create(origin: OriginFor<T>) -> DispatchResultWithPostInfo
        {
            /* Check signature */
            let sender = ensure_signed(origin)?;

            /* Generate random value from sender */
            let dna = Self::random_value(&sender);

            /* Mint kitty with reserve */
            let minted = Self::mint(&sender, dna)?;

            Self::deposit_event(Event::<T>::KittyCreated(minted.0, minted.1, minted.2));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResultWithPostInfo
        {
            /* Check signature */
            let sender = ensure_signed(origin)?;

            /* Make sure not same kitty & invalid kitty Id */
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameKittyId);
            let kitty_1 = Self::get_kitty(kitty_id_1).map_err(|_| Error::<T>::InvalidKittyId)?;
            let kitty_2 = Self::get_kitty(kitty_id_2).map_err(|_| Error::<T>::InvalidKittyId)?;

            /* Generate random seed  */
            let selector = Self::random_value(&sender);

            /* Use random seed generating bread dna */
            let mut dna = [0u8; 16];
            for i in 0..kitty_1.0.len(){
                dna[i] = (kitty_1.0[i] & selector[i]) | (kitty_2.0[i] & !selector[i]);
            }

            let minted = Self::mint(&sender, dna)?;

            Self::deposit_event(Event::<T>::KittyBreed(minted.0, minted.1, minted.2));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn transfer(origin: OriginFor<T>, kitty_id: T::KittyIndex, who: T::AccountId) -> DispatchResultWithPostInfo
        {
            /* Check signature */
            let sender = ensure_signed(origin)?;

            /* Check kitty exist */
            Self::get_kitty(kitty_id).map_err(|_| Error::<T>::InvalidKittyId)?;

            /* Only owner can transfer */
            ensure!(Self::kitty_owner(kitty_id) == Some(sender.clone()), Error::<T>::NotOwner);

            /* Reserve on target account */
            T::Currency::reserve(&who, T::Reserved::get()).map_err(|_| Error::<T>::NotEnoughBalanceReserved)?;
            /* Unreserve on source account */
            T::Currency::unreserve(&sender, T::Reserved::get());

            /* Transfer kitty */
            KittyOwner::<T>::insert(kitty_id, who.clone());

            /* Post event */
            Self::deposit_event(Event::KittyTransferred(sender, who, kitty_id));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn sell(origin: OriginFor<T>, kitty_id: T::KittyIndex, price: Option<BalanceOf<T>>) -> DispatchResultWithPostInfo
        {
            /*  */
            let sender = ensure_signed(origin)?;

            ensure!(Self::kitty_owner(kitty_id) == Some(sender.clone()), Error::<T>::NotOwner);

            SaleList::<T>::insert(kitty_id, price);

            Self::deposit_event(Event::<T>::KittyOnSale(sender, kitty_id, price));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn buy(origin: OriginFor<T>, kitty_id: T::KittyIndex) -> DispatchResultWithPostInfo
        {
            let sender = ensure_signed(origin)?;
            let owner = KittyOwner::<T>::get(kitty_id).unwrap();

            ensure!(sender.clone() != owner.clone(), Error::<T>::AlreadyOwned);

            let price = SaleList::<T>::get(kitty_id).ok_or(Error::<T>::NotForSale)?;

            ensure!(T::Currency::free_balance(&sender) > (price + T::Reserved::get()), Error::<T>::NotEnoughBalanceBuy);

            T::Currency::reserve(&sender, T::Reserved::get()).map_err(|_| Error::<T>::NotEnoughBalanceReserved)?;
            T::Currency::unreserve(&owner, T::Reserved::get());

            T::Currency::transfer(&sender, &owner, price, frame_support::traits::ExistenceRequirement::KeepAlive)?;

            KittyOwner::<T>::insert(kitty_id, sender.clone());

            SaleList::<T>::remove(kitty_id);

            Self::deposit_event(Event::<T>::KittySaled(owner, sender, kitty_id, Some(price)));

            Ok(().into())
        }

    }

    impl<T: Config> Pallet<T>
    {
        fn random_value(sender: &T::AccountId) -> [u8; 16]
        {
            let payload = (
                T::Randomness::random_seed(),
                sender.clone(),
                <frame_system::Pallet::<T>>::extrinsic_index(),
            );

            payload.using_encoded(blake2_128)
        }

        fn get_next_id() -> Result<T::KittyIndex, ()>
        {
            let _max_index = T::KittyIndex::max_value();

            match Self::kitty_count()
            {
                a if a ==_max_index => Err(()),
                val => Ok(val),
            }
        }

        fn get_kitty(kitty_id: T::KittyIndex) -> Result<Kitty, ()>
        {
            match Self::kitties(kitty_id)
            {
                Some(kitty) => Ok(kitty),
                None => Err(()),
            }
        }

        fn mint(owner: &T::AccountId, dna: [u8; 16]) -> Result<(T::AccountId, T::KittyIndex, Kitty), Error<T>>
        {
            let kitty_id = Self::get_next_id().map_err(|_| Error::<T>::InvalidKittyId)?;

            let reserved = T::Reserved::get();

            T::Currency::reserve(&owner, reserved).map_err(|_| Error::<T>::NotEnoughBalanceReserved)?;

            /* New kitty from dna */
            let new_kitty = Kitty(dna);
            /* Save kitty to storage */
            Kitties::<T>::insert(kitty_id, new_kitty.clone());
            /* Save kitty with owner */
            KittyOwner::<T>::insert(kitty_id, owner.clone());
            /* Update new kitty Id */
            KittyCount::<T>::set(kitty_id + 1u32.into());
            
            Ok((owner.clone(), kitty_id, new_kitty))
        }
    }
}