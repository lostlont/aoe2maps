use std::collections::HashSet;
use yew::
{
	prelude::*,
	agent::Dispatcher,
};
use crate::
{
	agents::settings::{ MenuState, Request, Settings },
	data::
	{
		expansion_pack::ExpansionPack,
		water_presence::WaterPresence,
	},
	utils::
	{
		hamburger::Hamburger,
		accordion::Accordion,
	},
};

#[derive(PartialEq)]
pub enum State
{
	Open,
	Collapsed,
}

pub struct Menu
{
	link: ComponentLink<Self>,
	settings: Dispatcher<Settings>,
	state: State,
	allowed_water_presence: HashSet<WaterPresence>,
}

impl Menu
{
	fn class(&self) -> String
	{
		let mut classes = vec!["menu"];

		match self.state
		{
			State::Open => classes.push("open"),
			State::Collapsed => classes.push("collapsed"),
		};

		classes.join(" ")
	}
}

pub enum Message
{
	ToggleState,
	ToggleWaterPresence(WaterPresence),
}

impl Component for Menu
{
	type Message = Message;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		Self
		{
			link,
			settings: Settings::dispatcher(),
			state: State::Open,
			allowed_water_presence: vec![
				WaterPresence::None,
				WaterPresence::Some,
				WaterPresence::Islands,
			].into_iter().collect(),
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::ToggleState =>
			{
				self.state = match self.state
				{
					State::Open => State::Collapsed,
					State::Collapsed => State::Open,
				};

				let menu_state = match self.state
				{
					State::Open => MenuState::Open,
					State::Collapsed => MenuState::Collapsed,
				};
				self.settings.send(Request::SetMenuState(menu_state));

				true
			},
			Message::ToggleWaterPresence(water_presence) =>
			{
				self.allowed_water_presence = &self.allowed_water_presence ^ &vec![water_presence].into_iter().collect();
				self.settings.send(Request::WaterPresence(self.allowed_water_presence.clone()));
				false
			},
		}
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
				<Hamburger
					clicked=self.link.callback(|_| Message::ToggleState) />
				<div class="content">
					<Accordion title="Kiegészítő">
						<div>{ ExpansionPack::TheAgeOfKings }</div>
						<div>{ ExpansionPack::TheConquerors }</div>
						<div>{ ExpansionPack::TheForgotten }</div>
						<div>{ ExpansionPack::TheAfricanKingdoms }</div>
						<div>{ ExpansionPack::RiseOfTheRajas }</div>
						<div>{ ExpansionPack::TheLastKhans }</div>
					</Accordion>
					<Accordion title="Víz mennyisége">
						<label>
							<input
								type="checkbox"
								value=WaterPresence::None
								checked=self.allowed_water_presence.contains(&WaterPresence::None)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::None)) />
							{ WaterPresence::None }
						</label>
						<label>
							<input
								type="checkbox"
								value=WaterPresence::Some
								checked=self.allowed_water_presence.contains(&WaterPresence::Some)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::Some)) />
							{ WaterPresence::Some }
						</label>
						<label>
							<input
								type="checkbox"
								value=WaterPresence::Islands
								checked=self.allowed_water_presence.contains(&WaterPresence::Islands)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::Islands)) />
							{ WaterPresence::Islands }
						</label>
					</Accordion>
				</div>
			</div>
		}
	}
}
