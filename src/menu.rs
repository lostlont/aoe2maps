use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	yew::
	{
		prelude::*,
		agent::Dispatcher,
	},
	crate::
	{
		agents::
		{
			filter::{ Filter, FilterView },
			settings::{ MenuState, Request, Settings },
		},
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
	filter: Rc<RefCell<Filter>>,
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
			filter: Rc::new(RefCell::new(Filter::new())),
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
				self.filter.borrow_mut().toggle(water_presence);
				self.settings.send(Request::FilterChanged(self.filter.clone()));

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
								checked=self.filter.borrow().allowed_water_presence().contains(&WaterPresence::None)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::None)) />
							{ WaterPresence::None }
						</label>
						<label>
							<input
								type="checkbox"
								value=WaterPresence::Some
								checked=self.filter.borrow().allowed_water_presence().contains(&WaterPresence::Some)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::Some)) />
							{ WaterPresence::Some }
						</label>
						<label>
							<input
								type="checkbox"
								value=WaterPresence::Islands
								checked=self.filter.borrow().allowed_water_presence().contains(&WaterPresence::Islands)
								onclick=self.link.callback(|_| Message::ToggleWaterPresence(WaterPresence::Islands)) />
							{ WaterPresence::Islands }
						</label>
					</Accordion>
				</div>
			</div>
		}
	}
}
