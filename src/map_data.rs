use yew::prelude::*;

use crate::map::Map;
use crate::expansion_pack::ExpansionPack;

#[derive(Clone)]
pub struct MapData
{
	pub name: String,
	pub expansion_pack: ExpansionPack,
}

impl Renderable for MapData
{
	fn render(&self) -> Html
	{
		html!
		{
			<Map map_data=self/>
		}
	}
}
