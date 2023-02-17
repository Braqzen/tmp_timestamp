contract;

use std::block::timestamp;

abi MyContract {
    fn test_function() -> bool;
}

impl MyContract for Contract {
    fn test_function() -> bool {
        log(timestamp());
        true
    }
}
