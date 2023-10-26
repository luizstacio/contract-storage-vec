contract;

use std::*;
use std::hash::*;
use std::storage::storage_vec::*;

storage {
    data: StorageMap<b256, StorageVec<u8>> = StorageMap::<b256, StorageVec<u8>> {}
}

abi MyContract {
    #[storage(read, write)]
    fn test_function(key: b256, args: Vec<u8>) -> Vec<u8>;

    #[storage(read)]
    fn return_vec(key: b256) -> Vec<u8>;
}

impl MyContract for Contract {
    #[storage(read, write)]
    fn test_function(key: b256, args: Vec<u8>) -> Vec<u8> {
        let my_data = storage.data.get(key);
        my_data.store_vec(args);
        return storage.data.get(key).load_vec();
    }

    #[storage(read)]
    fn return_vec(key: b256) -> Vec<u8> {
        return storage.data.get(key).load_vec();
    }
}
