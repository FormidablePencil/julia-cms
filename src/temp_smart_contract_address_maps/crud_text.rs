use std::fs::File;
use std::io;
use std::io::{Cursor, Write};
use std::str::from_utf8;

use futures::{StreamExt, TryStreamExt};
use ipfs_api_backend_actix::{IpfsApi, IpfsClient};
use serde::{Deserialize, Serialize};

use crate::temp_smart_contract_address_maps::encryption;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicText {
    pub title: String,
    pub body: String,
}

pub async fn upload(content: &BasicText, secret_encryption_key: Option<String>) -> String {
    let serialized_data = serde_json::to_string(content).unwrap();
    let data = match secret_encryption_key {
        None => {
            Cursor::new(serialized_data.as_bytes().to_owned())
        }
        Some(secret_key) => {
            let encrypted_data = encryption::encrypt(secret_key.as_bytes(), &serialized_data);
            Cursor::new(encrypted_data.as_slice().to_owned())
        }
    };

    let client = IpfsClient::default();
    let address = match client.add(data).await {
        Ok(res) => res.hash,
        Err(e) => panic!("error adding file: {:?}", e) // todo
    };

    // pin content through Storj ipfs
    let pinned = false;

    // save_in_collection_of_all address to private smart contract
    // crud_address_smart_contract::save_in_collection_of_all(address, pinned);

    address
}

async fn get(address: &str) -> Result<Vec<u8>, String> {
    // retrieve data from ipfs from ipns address
    let client = IpfsClient::default();

    match client
        .get(address)
        .map_ok(|chunk| {
            chunk.to_vec()
        })
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            let mut file = File::create("foo.txt").unwrap();
            file.write_all(&res);
            let f = from_utf8(res.as_slice()).unwrap();
            println!("11 - {} - 11", f);


            // out.write_all(&res);

            Ok(res)
        }
        Err(e) => Err(format!("error getting file: {:?}", e))
    }
}

fn update(address: String) {}

fn delete(address: String) {}

#[cfg(test)]
mod smart_contract_address_maps_tests {
    use std::str::from_utf8;
    use crate::temp_smart_contract_address_maps::crud_text::{BasicText, get, upload};

    #[actix_rt::test]
    async fn crud() {

        // let ipfs_content_address = String::from("ipfs_content_address");
        // save_in_collection_of_all(ipfs_content_address);

        let address = upload(
            &BasicText { title: String::from("title"), body: String::from("body") },
            // Some(String::from("my very secret key 123 abcdefghi")),
            None,
        ).await;
        // println!("{address}");

        let res = get(address.as_str()).await.unwrap();
        // let utf = from_utf8(res.as_slice());
        // println!("{}", utf.unwrap());
        //
        // // println!("{:?}", res.unwrap());
        // let f:BasicText = serde_json::from_slice(res.as_slice()).unwrap();
        // println!("{:?}", f);
    }
}