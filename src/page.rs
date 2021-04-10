use yew::prelude::*;

use crate::data::map_data::MapData;
use super::table::Table;
use super::menu::Menu;

#[derive(Properties, Clone)]
pub struct PageProperties
{
	pub maps: Vec<MapData>,
}

pub struct Page
{
	properties: PageProperties,
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
			<>
				<Table maps=&self.properties.maps />
				<Menu />
			</>
		}
	}
}
