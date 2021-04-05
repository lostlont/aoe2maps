use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod map;
mod page;

use page::Page;

#[wasm_bindgen(start)]
pub fn run_app()
{
	App::<Page>::new().mount_to_body();
}
