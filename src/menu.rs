use yew::prelude::*;

use crate::data::expansion_pack::ExpansionPack;
use crate::data::water_presence::WaterPresence;
use super::accordion::Accordion;

pub struct Menu
{
}

impl Component for Menu
{
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self
	{
		Self
		{
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
					<div>{ WaterPresence::None }</div>
					<div>{ WaterPresence::Some }</div>
					<div>{ WaterPresence::Islands }</div>
				</Accordion>
			</div>
		}
	}
}
