/*
Iterates through Vec and pushes non 0 (empty) bytes to an Vec<Vec<u8>>. Where another cluster
begins start pushing bytes to a new Vec<u8>.
 */
use serde::de::DeserializeOwned;

fn parse_ipfs_object(vec: Vec<u8>) -> Vec<Vec<u8>> {
    let mut data_clusters: Vec<Vec<u8>> = vec![];

    let mut previous_byte_empty_or_first_value = true;
    let mut cluster: Vec<u8> = vec![];

    for item in vec {
        if item != 0 {
            cluster.push(item);
            previous_byte_empty_or_first_value = false;
        } else {
            if previous_byte_empty_or_first_value == false {
                data_clusters.push(cluster);
                cluster = vec![];
            }
            previous_byte_empty_or_first_value = true;
        }
    }

    data_clusters
}

/*
Parses data from ipfs object and returns the json data of the provided type of <T>.
 */
pub fn get_data<T>(ipfs_object: Vec<u8>) -> serde_json::Result<T>
    where
        T: DeserializeOwned,
{
    let parsed_object = parse_ipfs_object(ipfs_object);

    serde_json::from_slice::<T>(
        parsed_object[&parsed_object.len() - 1].as_slice()
    )
}

#[cfg(test)]
mod parse_ipfs_object {
    use std::str::from_utf8;

    use futures::TryStreamExt;
    use ipfs_api_backend_actix::{IpfsApi, IpfsClient};

    use crate::helpers::parse_ipfs_object::{get_data, parse_ipfs_object};
    use crate::temp_smart_contract_address_maps::crud_text;
    use crate::temp_smart_contract_address_maps::crud_text::BasicText;

    #[actix_rt::test]
    async fn get_ipfs_content_test() {
        let req = BasicText { title: String::from("title df df df"), body: String::from("body 00 0  df") };
        let address = crud_text::upload(
            &req,
            None,
        ).await;
        let data_bytes = IpfsClient::default()
            .get(address.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await.unwrap();
        let res = get_data::<BasicText>(data_bytes).unwrap();

        assert_eq!(format!("{:?}", req), format!("{:?}", res));
    }

    /*
    Uploaded content to IPFS. Fetched content by address.
     */
    #[actix_rt::test]
    async fn parse_ipfs_object_test() {
        let req = BasicText { title: String::from("title df df df"), body: String::from("body 00 0  df") };
        let address = crud_text::upload(
            &req,
            None,
        ).await;
        let data_bytes = IpfsClient::default()
            .get(address.as_str())
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await.unwrap();

        let data_parsed = parse_ipfs_object(data_bytes);

        // for cluster in data_parsed {
        //     println!("{:?}", from_utf8(cluster.as_slice()).unwrap());
        // };

        assert_eq!(
            format!("{:?}", req),
            format!("{:?}", serde_json::from_slice::<BasicText>(
                data_parsed[data_parsed.len() - 1].as_slice()).unwrap()
            )
        );

        assert_eq!(
            format!("{:?}", address),
            format!("{:?}", from_utf8(data_parsed[0].as_slice()).unwrap())
        );
    }
}