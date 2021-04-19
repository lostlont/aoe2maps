use std::collections::HashSet;
use yew::
{
	prelude::*,
	agent::Dispatcher,
};
use crate::agents::settings::{ Request, Settings };
use crate::data::
{
	expansion_pack::ExpansionPack,
	water_presence::WaterPresence,
};
use super::accordion::Accordion;

pub struct Menu
{
	link: ComponentLink<Self>,
	settings: Dispatcher<Settings>,
	allowed_water_presence: HashSet<WaterPresence>,
}

pub enum Message
{
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
			<div class="menu">
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
		}
	}
}
