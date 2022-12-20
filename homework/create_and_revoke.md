完整代码见[pallets/poe/src/lib.rs](../pallets/poe/src/lib.rs)
# 创建凭证
```rust
    #[pallet::weight(0)]
    pub fn create_claim(origin: OriginFor<T>,claim: Vec<u8>) -> DispatchResultWithPostInfo {
        // 校验用户
        let from = ensure_signed(origin)?;
        // 凭证检查
        let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
        // 凭证检查
        ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ClaimExisted);
        // 插入凭证
        Proofs::<T>::insert(&bounded_claim, (from.clone(), frame_system::Pallet::<T>::block_number()));
        // 发出事件
        Self::deposit_event(Event::<T>::ClaimCreated {
            from,
            claim
        });
        Ok(().into())
    }

```
# 移除凭证

```rust
    #[pallet::weight(0)]
    pub fn revoke_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
        // 校验用户
        let from = ensure_signed(origin)?;
        // 校验凭证
        let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
        let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
        // 校验权限
        ensure!(from == owner, Error::<T>::NotClaimOwner);
        // 移除
        Proofs::<T>::remove(&bounded_claim);
        // 发出事件
        Self::deposit_event(Event::<T>::ClaimRevoked {
        from, claim
        });
        Ok(().into())
    }
```
# [操作录屏](https://user-images.githubusercontent.com/83388462/208645139-39611a74-1d1f-4d57-98df-131caff53031.mp4)