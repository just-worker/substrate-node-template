# [frame_support](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html)

> 用于标记 `pallet` 模块，具体模板如下

```rust
/// 导出当前定义的 `pallet`
/// 一般情况下，定义的 `mod` 都叫做 `pallet` 
pub use pallet::*;

/// 用于标记定义的 `pallet` 模块，标记之后，后续的宏才能进行使用
#[frame_support::pallet]
pub mod pallet {
    /// 导入基础的公共包
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    /// 定义结构体
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    /// 配置：#[pallet::config]  
    /// 事件：#[pallet::event]   
    /// 错误：#[pallet::error]   
    /// 存储：#[pallet::storage] 
    /// 调用：#[pallet::call]    
}

```

# [pallet::pallet](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#pallet-struct-placeholder-palletpallet-mandatory)
> 定义结构体

```rust
#[pallet::pallet]
#[pallet::generate_store(pub(super) trait Store)]
pub struct Pallet<T>(_);
```

# [pallet::config](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#config-trait-palletconfig-mandatory)
> 定义配置

```rust
/// 用于标记配置申明
#[pallet::config]
pub trait Config: frame_system::Config {
  type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}
```

# [pallet::constant](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#palletconstant)
> 定义常量

```rust
#[pallet::config]
pub trait Config: frame_system::Config {
    #[pallet::constant] 
    type MyGetParam: Get<u32>;
}
```


# [pallet::event](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#event-palletevent-optional)
> 定义事件

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Event emitted when a claim has been created.
    ClaimCreated { who: T::AccountId, claim: T::Hash },
    /// Event emitted when a claim is revoked by the owner.
    ClaimRevoked { who: T::AccountId, claim: T::Hash },
}
```

# [pallet::error](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#error-palleterror-optional)
> 定义错误

```rust
#[pallet::error]
pub enum Error<T> {
    /// The claim already exists.
    AlreadyClaimed,
    /// The claim does not exist, so it cannot be revoked.
    NoSuchClaim,
    /// The claim is owned by another account, so caller can't revoke it.
    NotClaimOwner,
}
```

# [pallet::storage](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#storage-palletstorage-optional)
> 定义存储

```rust
#[pallet::storage]
pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;
```

# [pallet::call](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#call-palletcall-optional) and [weight](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#palletweightexpr)

> 暴露方法调用和权重设定

```rust
// Dispatchable functions allow users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
#[pallet::call]
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
```

# [pallet::hooks](https://paritytech.github.io/substrate/master/frame_support/attr.pallet.html#hooks-pallethooks-optional)
> 定义回调

```rust
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
  // Hooks functions and logic goes here.
}
```