#![recursion_limit = "1024"]

use
{
	wasm_bindgen::prelude::*,
	yew::prelude::*,
	data::create_maps,
	page::
	{
		Page,
		PageProperties,
	},
};

mod agents;
mod data;
mod map;
mod map_attribute_filter;
mod map_attribute_set_filter;
mod menu;
mod page;
mod table;
mod utils;

#[wasm_bindgen(start)]
pub fn run_app()
{
	let properties = PageProperties
	{
		maps: create_maps(),
	};

	App::<Page>::new().mount_to_body_with_props(properties);
}
