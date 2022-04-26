// dependency injection?

// Access the models directly without the need of the hierarchy; service, manager, repository.
//  This is because the logic doesn't have to be accessed from the central location of routes. Now the logic can be directly accessed there is no need for a hierarchy anymore.
//  Instead of services, managers, repositories, the logic is going to be organized by composition categories.

// use std::ops::Residual;

use core::fmt;
use std::error::Error;

use codegen::{Block, Function, Scope};
use ipfsapi::IpfsApi;
use julia_cms::{
    compositions::{texts::TextType, CompositionCategory},
    gencode::impl_composition_type_manager,
};

// use julia_cms::compositions::{
//     banners::{banner_basic::BannerCreateReq, BannerManager, BannerType},
//     carousels::carousel_blurred_overlay::get_public,
// };
mod compositions;
// std::io::Read

fn ipfs_test() {
    // let api = IpfsApi::new("127.0.0.1", 5001);

    // let data = DataTest123 {};
    // let res = api.block_put(data);

    // let bytes = api
    //     .cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u")
    //     .unwrap();
    // let data = String::from_utf8(bytes.collect()).unwrap();

    // println!("k");
    // println!("{}", data);
    // println!("k");
}

fn main() {
    // impl_composition_type_manager(CompositionCategory::Banner(BannerType::Basic));
    // impl_composition_type_manager(
    //     CompositionCategory::Carousel(CarouselType::Basic),
    //     "BannerCreateReq",
    // );
    // impl_composition_type_manager(CompositionCategory::Text(TextType::Basic));
    // let banner_basic_arm = CrudOperation::Create.get_arms(
    //     "BannerType".to_string(),
    //     "Basic".to_string(),
    //     "banner_basic",
    // );

    // let mut arms_for_methods = Vec::new();

    // for item in CrudOperation::iter() {
    // arms_for_methods.push(item.get_arms("BannerType", "Basic", "banner_basic"));
    // }

    // trait BannerType2 {
    //     fn get_name(&self) -> String {
    //         format!("{}", &self)
    //     }
    // }

    //     println!("Hello, world!");
    // compositions::carousels::add();
    //     carousel_basic::delete(1);

    //     get_public(1);
}

// use std::io::Cursor;

// #[tokio::main]
// async fn main() {
// let client = IpfsClient::default();
// let data = Cursor::new("Hello World!");

// match client.add(data).await {
//     Ok(res) => println!("{}", res.hash),
//     Err(e) => eprintln!("error adding file: {}", e),
// }
// }
