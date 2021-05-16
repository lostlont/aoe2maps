#![recursion_limit = "1024"]

use
{
	wasm_bindgen::prelude::*,
	yew::prelude::*,
	components::page::{ Page, PageProperties },
	data::create_maps,
};

mod agents;
mod components;
mod data;
mod views;

#[wasm_bindgen(start)]
pub fn run_app()
{
	let properties = PageProperties
	{
		maps: create_maps(),
	};

	App::<Page>::new().mount_to_body_with_props(properties);
}
