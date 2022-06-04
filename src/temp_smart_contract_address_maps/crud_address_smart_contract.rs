// unpin address-content
// remove ipns address

// todo - only dealing with addresses. This will belong in a smart contract

fn save(address: String) {
    
}

fn get(address: String) {

}

fn update(address: String) {

}

fn delete(address: String) {
    
}

#[cfg(test)]
mod smart_contract_address_maps_tests {
    use crate::temp_smart_contract_address_maps::crud::save;

    #[test]
    fn crud() {
        let ipfs_content_address = String::from("ipfs_content_address");
        save(ipfs_content_address);
        
        
    }
}