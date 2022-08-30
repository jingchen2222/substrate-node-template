#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]  // <-- Step 2. code block will replace this.
  /// Configure the pallet by specifying the parameters and types on which it depends
pub trait Config: frame_system::Config {
  /// Because this pallet emits events, it depends on the runtime's definition of an event.
  type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
}
  #[pallet::event]   // <-- Step 3. code block will replace this.
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  ///
  /// As described in Design the application, the proof-of-existence pallet emits an event under the following conditions:
  /// When a new claim is added to the blockchain.
  /// When a claim is revoked.
  /// Each event also displays an AccountId to identify who triggered the event and the proof-of-existence claim (as Hash) that is being stored or removed.
  pub enum Event<T: Config> {
    /// Event emitted when a claim has been created.
    ClaimCreated { who: T::AccountId, claim: T::Hash },
    /// Event emitted when a claim is revoked by the owner.
    ClaimRevoked { who: T::AccountId, claim: T::Hash },
  }

  /// An attempt to make a claim that has already exists.
  /// An attempt to revoke a claim that does not exist.
  /// An attempt to revoke a claim that is owned by another account.
  #[pallet::error]   // <-- Step 4. code block will replace this.
  pub enum Error<T> {
    /// The claim already exists.
    AlreadyClaimed,
    /// The claim does not exist, so it cannot be revoked.
    NoSuchClaim,
    /// The claim is owned by another account, so caller can't revoke it.
    NotClaimOwner,
  }
  #[pallet::storage] // <-- Step 5. code block will replace this.
  pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;
  // Dispatchable functions allow users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
  #[pallet::call]    // <-- Step 6. code block will replace this.
impl<T: Config> Pallet<T> {
  #[pallet::weight(0)]
  pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    // This function will return an error if the extrinsic is not signed.
    let sender = ensure_signed(origin)?;

    // Verify that the specified claim has not already been stored.
    ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

    // Get the block number from the FRAME System pallet.
    let current_block = <frame_system::Pallet<T>>::block_number();

    // Store the claim with the sender and block number.
    Claims::<T>::insert(&claim, (&sender, current_block));

    // Emit an event that the claim was created.
    Self::deposit_event(Event::ClaimCreated { who: sender, claim });

    Ok(())
  }

  #[pallet::weight(0)]
  pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    // This function will return an error if the extrinsic is not signed.
    let sender = ensure_signed(origin)?;

    // Get owner of the claim, if none return an error.
    let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

    // Verify that sender of the current call is the claim owner.
    ensure!(sender == owner, Error::<T>::NotClaimOwner);

    // Remove claim from storage.
    Claims::<T>::remove(&claim);

    // Emit an event that the claim was erased.
    Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
    Ok(())
  }
}
}



