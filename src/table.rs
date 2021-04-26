use yew::prelude::*;

use crate::agents::settings::{ MenuState, Request, Settings };
use crate::data::map_data::MapData;
use super::map::Map;

#[derive(Properties, Clone)]
pub struct TableProperties
{
	pub maps: Vec<MapData>,
}

pub struct Table
{
	properties: TableProperties,
	_settings: Box<dyn Bridge<Settings>>,
	menu_state: MenuState,
}

impl Table
{
	fn render_map(map_data: &MapData) -> Html
	{
		html!
		{
			<Map map_data=map_data/>
		}
	}

	fn class(&self) -> String
	{
		let mut classes = vec!["table"];

		match self.menu_state
		{
			MenuState::Open => classes.push("menu-open"),
			MenuState::Collapsed => classes.push("menu-collapsed"),
		};

		classes.join(" ")
	}
}

pub enum Message
{
	None,
	SetMenuState(MenuState),
}

impl Component for Table
{
	type Message = Message;
	type Properties = TableProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let callback = |request| match request
		{
			Request::SetMenuState(menu_state) => Message::SetMenuState(menu_state),
			_ => Message::None,
		};

		Self
		{
			properties,
			_settings: Settings::bridge(link.callback(callback)),
			menu_state: MenuState::Open,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::None => {},
			Message::SetMenuState(menu_state) => self.menu_state = menu_state,
		};

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
			<div class=self.class()>
				<div class="row">
					<div class="header"><h2>{ "Név" }</h2></div>
					<div class="header"><h2>{ "Kép" }</h2></div>
					<div class="header"><h2>{ "Kiegészítő" }</h2></div>
					<div class="header"><h2>{ "Jellemzők" }</h2></div>
				</div>
				{ for self.properties.maps.iter().map(Self::render_map) }
			</div>
		}
	}
}
