use yew::prelude::*;

use crate::data::map_data::MapData;
use crate::map::Map;

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
