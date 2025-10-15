#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::*, dispatch::DispatchResult};
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn verify(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }
    }
}
