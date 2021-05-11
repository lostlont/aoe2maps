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
			filter_method::FilterMethod,
			map_attribute::{ ExpansionPack, ResourceAmount, WaterPresence },
		},
		map_attribute_set_filter::MapAttributeSetFilter,
		utils::
		{
			accordion::Accordion,
			hamburger::Hamburger,
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
	ChangedMapAttribute,
	SetFilterMethod(FilterMethod),
}

impl Component for Menu
{
	type Message = Message;
	type Properties = ();

	fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let is_mobile = web_sys::window()
			.and_then(|w| w.document())
			.and_then(|d| d.document_element())
			.map_or(false, |de| de.client_width() <= 992);

		let state = match is_mobile
		{
			true => State::Collapsed,
			false => State::Open,
		};

		Self
		{
			link,
			settings: Settings::dispatcher(),
			state,
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
			Message::SetFilterMethod(filter_method) =>
			{
				self.filter.borrow_mut().set_filter_method(filter_method);

				self.settings.send(Request::FilterChanged(self.filter.clone()));
				true
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
					<Accordion title="Szűrés módja">
						<label>
							<input
								type="radio"
								checked=self.filter.borrow().filter_method() == FilterMethod::Hide
								onclick=self.link.callback(|_| Message::SetFilterMethod(FilterMethod::Hide))
							/>
							{ FilterMethod::Hide }
						</label>
						<label>
							<input
								type="radio"
								checked=self.filter.borrow().filter_method() == FilterMethod::Disable
								onclick=self.link.callback(|_| Message::SetFilterMethod(FilterMethod::Disable))
							/>
							{ FilterMethod::Disable }
						</label>
						<label>
							<input
								type="radio"
								checked=self.filter.borrow().filter_method() == FilterMethod::Mixed
								onclick=self.link.callback(|_| Message::SetFilterMethod(FilterMethod::Mixed))
							/>
							{ FilterMethod::Mixed }
						</label>
					</Accordion>
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
					<MapAttributeSetFilter<ResourceAmount>
						title="Fa mennyisége"
						map_attribute_set=self.filter.borrow().wood_amount()
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
					<MapAttributeSetFilter<ResourceAmount>
						title="Táplálék mennyisége"
						map_attribute_set=self.filter.borrow().food_amount()
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
					<MapAttributeSetFilter<ResourceAmount>
						title="Arany mennyisége"
						map_attribute_set=self.filter.borrow().gold_amount()
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
					<MapAttributeSetFilter<ResourceAmount>
						title="Kő mennyisége"
						map_attribute_set=self.filter.borrow().stone_amount()
						changed=self.link.callback(|_| Message::ChangedMapAttribute)
					/>
				</div>
			</div>
		}
	}
}
