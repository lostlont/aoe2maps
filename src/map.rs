use std::collections::HashSet;
use yew::
{
	prelude::*,
	Properties,
};

use crate::agents::settings::{ Request, Settings };
use crate::data::
{
	map_data::MapData,
	water_presence::WaterPresence,
};

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
	_settings: Box<dyn Bridge<Settings>>,
}

impl Map
{
	fn row_class(&self) -> &str
	{
		if self.properties.active
		{
			"row active-row"
		}
		else
		{
			"row inactive-row"
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

pub enum Message
{
	WaterPresence(HashSet<WaterPresence>),
}

impl Component for Map
{
	type Message = Message;
	type Properties = MapProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let callback = |request| match request
		{
			Request::WaterPresence(allowed_water_presence) => Message::WaterPresence(allowed_water_presence),
		};

		Self
		{
			properties,
			_settings: Settings::bridge(link.callback(callback)),
		}
	}

	fn update(&mut self, message: Self::Message) -> bool
	{
		match message
		{
			Message::WaterPresence(allowed_water_presence) =>
			{
				self.properties.active = allowed_water_presence.contains(&self.properties.map_data.water_presence);
			},
		}

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
			<div class=self.row_class()>
				<div><h3>{ &self.properties.map_data.name }</h3></div>
				<div><img src=&self.properties.map_data.image /></div>
				<div>{ &self.properties.map_data.expansion_pack }</div>
				<div>{ self.render_features() }</div>
			</div>
		}
	}
}
