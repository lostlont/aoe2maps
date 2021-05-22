use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	closure::closure,
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
		components::
		{
			menu::
			{
				enum_filter::EnumFilter,
				enum_set_filter::EnumSetFilter,
			},
			utils::hamburger::Hamburger,
		},
		data::
		{
			filter_method::FilterMethod,
			map_attribute::{ ExpansionPack, ResourceAmount, WaterPresence },
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
	filter_method_filter: EnumFilter<FilterMethod>,
	expansion_pack_filter: EnumFilter<ExpansionPack>,
	water_presence_filter: EnumSetFilter<WaterPresence>,
	wood_amount_filter: EnumSetFilter<ResourceAmount>,
	food_amount_filter: EnumSetFilter<ResourceAmount>,
	gold_amount_filter: EnumSetFilter<ResourceAmount>,
	stone_amount_filter: EnumSetFilter<ResourceAmount>,
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
	ChangedFilterMethod(FilterMethod),
	ChangedExpansionPack(ExpansionPack),
	ChangedMapAttribute{ repaint: bool },
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

		let filter = Rc::new(RefCell::new(Filter::new()));
		Self
		{
			link: link.clone(),
			settings: Settings::dispatcher(),
			filter_method_filter: EnumFilter::new(
				"Szűrés módja".to_string(),
				Rc::new(closure!(clone filter, || filter.borrow().get_filter_method())),
				Rc::new(closure!(clone link, |v| link.send_message(Message::ChangedFilterMethod(v))))),
			expansion_pack_filter: EnumFilter::new(
				"Kiegészítő".to_string(),
				Rc::new(closure!(clone filter, || filter.borrow().get_expansion_pack())),
				Rc::new(closure!(clone link, |v| link.send_message(Message::ChangedExpansionPack(v))))),
			water_presence_filter: EnumSetFilter::new(
				"Víz mennyisége".to_string(),
				Rc::new(closure!(clone filter, |a| filter.borrow().is_allowed_water_presence(a))),
				Rc::new(closure!(clone filter, clone link, |a|
					{
						filter.borrow_mut().toggle_allowed_water_presence(a);
						link.send_message(Message::ChangedMapAttribute{ repaint: true });
					})),
			),
			wood_amount_filter: EnumSetFilter::new(
				"Fa mennyisége".to_string(),
				Rc::new(closure!(clone filter, |a| filter.borrow().is_allowed_wood_amount(a))),
				Rc::new(closure!(clone filter, clone link, |a|
					{
						filter.borrow_mut().toggle_allowed_wood_amount(a);
						link.send_message(Message::ChangedMapAttribute{ repaint: true });
					})),
			),
			food_amount_filter: EnumSetFilter::new(
				"Táplálék mennyisége".to_string(),
				Rc::new(closure!(clone filter, |a| filter.borrow().is_allowed_food_amount(a))),
				Rc::new(closure!(clone filter, clone link, |a|
					{
						filter.borrow_mut().toggle_allowed_food_amount(a);
						link.send_message(Message::ChangedMapAttribute{ repaint: true });
					})),
			),
			gold_amount_filter: EnumSetFilter::new(
				"Arany mennyisége".to_string(),
				Rc::new(closure!(clone filter, |a| filter.borrow().is_allowed_gold_amount(a))),
				Rc::new(closure!(clone filter, clone link, |a|
					{
						filter.borrow_mut().toggle_allowed_gold_amount(a);
						link.send_message(Message::ChangedMapAttribute{ repaint: true });
					})),
			),
			stone_amount_filter: EnumSetFilter::new(
				"Kő mennyisége".to_string(),
				Rc::new(closure!(clone filter, |a| filter.borrow().is_allowed_stone_amount(a))),
				Rc::new(closure!(clone filter, clone link, |a|
					{
						filter.borrow_mut().toggle_allowed_stone_amount(a);
						link.send_message(Message::ChangedMapAttribute{ repaint: true });
					})),
			),
			state,
			filter,
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
			Message::ChangedFilterMethod(filter_method) =>
			{
				self.filter.borrow_mut().set_filter_method(filter_method);
				self.settings.send(Request::FilterChanged(self.filter.clone()));
				true
			},
			Message::ChangedExpansionPack(expansion_pack) =>
			{
				self.filter.borrow_mut().set_expansion_pack(expansion_pack);
				self.settings.send(Request::FilterChanged(self.filter.clone()));
				true
			},
			Message::ChangedMapAttribute{repaint} =>
			{
				self.settings.send(Request::FilterChanged(self.filter.clone()));
				repaint
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
				{ self.filter_method_filter.render() }
				{ self.expansion_pack_filter.render() }
				{ self.water_presence_filter.render() }
				{ self.wood_amount_filter.render() }
				{ self.food_amount_filter.render() }
				{ self.gold_amount_filter.render() }
				{ self.stone_amount_filter.render() }
				</div>
			</div>
		}
	}
}
