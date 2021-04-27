#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod accordion;
mod agents;
mod data;
mod map;
mod menu;
mod page;
mod table;
mod utils;

use data::create_maps;
use page::Page;
use page::PageProperties;

#[wasm_bindgen(start)]
pub fn run_app()
{
	let properties = PageProperties
	{
		maps: create_maps(),
	};

	App::<Page>::new().mount_to_body_with_props(properties);
}
