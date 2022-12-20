#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// [`frame_support::pallet`] 用于标记新建的 `pallet` 
#[frame_support::pallet]
pub mod pallet {
	
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// the maximum length of claim
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber)
	> ;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated {
			from: T::AccountId,
			claim: Vec<u8>
		},
		ClaimRevoked {
			from: T::AccountId, 
			claim: Vec<u8>
		},
		ClaimShifted {
			from: T::AccountId,
			to: T::AccountId,
			claim: Vec<u8>
		}
		 
	}

	#[pallet::error]
	pub enum Error<T> {
		ClaimExisted,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::call]
	impl <T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ClaimExisted);
			Proofs::<T>::insert(&bounded_claim, (from.clone(), frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::<T>::ClaimCreated {
				from,
				claim
			});
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(from == owner, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&bounded_claim);
			Self::deposit_event(Event::<T>::ClaimRevoked {
				from, claim
			});
			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, to: T::AccountId) -> DispatchResultWithPostInfo {
			let from = ensure_signed(origin)?;
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			ensure!(from == owner, Error::<T>::NotClaimOwner);
			Proofs::<T>::remove(&bounded_claim);
			Proofs::<T>::insert(&bounded_claim, (to.clone(), frame_system::Pallet::<T>::block_number()));
			Self::deposit_event(Event::<T>::ClaimShifted {
				from,
				to,
				claim
			});
			Ok(().into())
		}

	}
}