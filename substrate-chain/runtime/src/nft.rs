use parity_codec::{Decode, Encode};
use runtime_primitives::traits::Hash;
use support::{decl_module, decl_storage, dispatch::Result, StorageMap, StorageValue};
use system::ensure_signed;

#[derive(Encode, Decode, Default)]
pub struct NftItem<Hash> {
    id: Hash,
}

pub trait Trait: balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as NftStorage {
        AllItemCount get(all_item_count): u64;
        Item get(item_of): map T::AccountId => NftItem<T::Hash>;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {

        fn create_item(origin) -> Result {
            let sender = ensure_signed(origin)?;
            let all_item_count = Self::all_item_count();
            let new_all_item_count = all_item_count.checked_add(1)
                .ok_or("Overflow adding a new item")?;
            let new_item = NftItem {
                id: <T as system::Trait>::Hashing::hash_of(&0),
            };
            <AllItemCount<T>>::put(new_all_item_count);
            <Item<T>>::insert(&sender, new_item);
            Ok(())
        }
    }
}
