#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::fungible::{Inspect, Mutate};
	use frame_system::pallet_prelude::*;

	//----------------------------------------------------------------------------
	// Types and Constants
	//----------------------------------------------------------------------------

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	//----------------------------------------------------------------------------
	// Configuration
	//----------------------------------------------------------------------------

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
	}

	//----------------------------------------------------------------------------
	// Storage
	//----------------------------------------------------------------------------

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub dna: [u8; 32],
		pub owner: T::AccountId,
		pub price: Option<BalanceOf<T>>,
	}

	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 32], Value = Kitty<T>>;

	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 32], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	//----------------------------------------------------------------------------
	// Events
	//----------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred { from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32] },
		PriceSet { caller: T::AccountId, kitty_id: [u8; 32], new_price: Option<BalanceOf<T>> },
		Sold { buyer: T::AccountId, kitty_id: [u8; 32], price: BalanceOf<T> },
	}

	//----------------------------------------------------------------------------
	// Errors
	//----------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
		TransferToSelf,
		NoKitty,
		NotOwner,
		NotForSale,
		MaxPriceTooLow,
	}

	//----------------------------------------------------------------------------
	// Extrinsics
	//----------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			kitty_id: [u8; 32],
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		pub fn set_price(
			origin: OriginFor<T>,
			kitty_id: [u8; 32],
			new_price: Option<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_price(who, kitty_id, new_price)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		pub fn buy_kitty(
			origin: OriginFor<T>,
			kitty_id: [u8; 32],
			max_price: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_buy_kitty(who, kitty_id, max_price)?;
			Ok(())
		}
	}
}
