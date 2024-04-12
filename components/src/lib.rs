#![allow(non_snake_case)]

#[macro_use]
mod props;

mod use_unique_id;
pub(crate) use use_unique_id::*;

pub mod layout;
pub mod display;
pub mod nav;


const _: &str = manganis::mg!(file("./css-out/index.css"));
