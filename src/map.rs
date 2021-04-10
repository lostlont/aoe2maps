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

impl Map
{
	fn row_class(&self) -> &str
	{
		if self.properties.active
		{
			"active-row"
		}
		else
		{
			"inactive-row"
		}
	}

	fn render_features(&self) -> Html
	{
		html!
		{
			<ul>
				<li>{ &self.properties.map_data.water_presence }</li>
			</ul>
		}
	}
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
		html!
		{
			<>
				<div class=self.row_class()><h3>{ &self.properties.map_data.name }</h3></div>
				<div class=self.row_class()><img src=&self.properties.map_data.image /></div>
				<div class=self.row_class()>{ &self.properties.map_data.expansion_pack }</div>
				<div class=self.row_class()>{ self.render_features() }</div>
			</>
		}
	}
}
