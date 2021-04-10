use yew::prelude::*;

use crate::data::map_data::MapData;
use super::map::Map;

#[derive(Properties, Clone)]
pub struct PageProperties
{
	pub maps: Vec<MapData>,
}

pub struct Page
{
	properties: PageProperties,
}

impl Page
{
	fn render_map(map_data: &MapData) -> Html
	{
		html!
		{
			<Map map_data=map_data/>
		}
	}
}

impl Component for Page
{
	type Message = ();
	type Properties = PageProperties;

	fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties,
		}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender
	{
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender
	{
		false
	}

	fn view(&self) -> Html
	{
		html!
		{
			<div class="grid">
				<div class="header"><h2>{ "Név" }</h2></div>
				<div class="header"><h2>{ "Kép" }</h2></div>
				<div class="header"><h2>{ "Kiegészítő" }</h2></div>
				<div class="header"><h2>{ "Jellemzők" }</h2></div>
				{ for self.properties.maps.iter().map(Page::render_map) }
			</div>
		}
	}
}
