use yew::prelude::*;
use yew::Properties;

use crate::data::map_data::MapData;

#[derive(Properties, Clone)]
pub struct MapProperties
{
	pub map_data: MapData,

	#[prop_or(true)]
	pub active: bool,
}

pub struct Map
{
	properties: MapProperties,
}

impl Component for Map
{
	type Message = ();
	type Properties = MapProperties;

	fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties
		}
	}

	fn update(&mut self, _: Self::Message) -> bool
	{
		true
	}

	fn change(&mut self, _: Self::Properties) -> bool
	{
		false
	}

	fn view(&self) -> Html
	{
		let row_class = if self.properties.active
		{
			"active-row"
		}
		else
		{
			"inactive-row"
		};

		html!
		{
			<tr class=row_class>
				<td>{ &self.properties.map_data.name }</td>
				<td><img src=&self.properties.map_data.image /></td>
				<td>{ &self.properties.map_data.expansion_pack }</td>
			</tr>
		}
	}
}
