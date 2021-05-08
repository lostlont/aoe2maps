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
			filter::Filter,
			settings::{ MenuState, Request, Settings },
		},
		data::map_attribute::{ ExpansionPack, WaterPresence },
		map_attribute_set_filter::MapAttributeSetFilter,
		utils::hamburger::Hamburger,
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
	ChangedMapAttribute,
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
			Message::ChangedMapAttribute =>
			{
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
				<Hamburger clicked=self.link.callback(|_| Message::ToggleState) />
				<div class="content">
					<MapAttributeSetFilter<ExpansionPack>
						title="Kiegészítő"
						map_attribute_set=self.filter.borrow().expansion_pack()
						is_incremental=true
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
					<MapAttributeSetFilter<WaterPresence>
						title="Víz mennyisége"
						map_attribute_set=self.filter.borrow().water_presence()
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
				</div>
			</div>
		}
	}
}
