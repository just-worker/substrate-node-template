完整代码见[pallets/poe/src/lib.rs](../pallets/poe/src/lib.rs)

# 凭证转移
```rust
		#[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, to: T::AccountId) -> DispatchResultWithPostInfo {
			// 校验用户
            let from = ensure_signed(origin)?;
            // 校验凭证
			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ClaimTooLong)?;
			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;
			// 校验权限
            ensure!(from == owner, Error::<T>::NotClaimOwner);
			// 移除旧凭证
            Proofs::<T>::remove(&bounded_claim);
			// 转移新凭证
            Proofs::<T>::insert(&bounded_claim, (to.clone(), frame_system::Pallet::<T>::block_number()));
			// 发出事件
            Self::deposit_event(Event::<T>::ClaimShifted {
				from,
				to,
				claim
			});
			Ok(().into())
		}

```
# 操作录屏
<video src="transfer.mp4"><video>