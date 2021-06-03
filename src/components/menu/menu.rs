use
{
	std::
	{
		cell::RefCell,
		fmt::Display,
		hash::Hash,
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
			enum_values::EnumValues,
			filter_method::FilterMethod,
			map_attribute::{ ExpansionPack, MapCategory, ResourceAmount },
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
	map_categories_filter: EnumSetFilter<MapCategory>,
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

	fn create_enum_filter<T>(
		filter: &Rc<RefCell<Filter>>,
		link: &ComponentLink<Self>,
		title: &str,
		get_value: impl Fn(&Filter) -> T + 'static,
		set_value: impl Fn(&mut Filter, T) + 'static)
		-> EnumFilter<T>
	where
		T: Copy + Display + EnumValues + Eq + Hash + 'static,
	{
		EnumFilter::new(
			title.to_string(),
			Rc::new(closure!(clone filter, || get_value(&filter.borrow()))),
			Rc::new(closure!(clone filter, clone link, |v|
				{
					set_value(&mut *filter.borrow_mut(), v);
					link.send_message(Message::ChangedFilter{ repaint: true });
				})))
	}

	fn create_enum_set_filter<T>(
		filter: &Rc<RefCell<Filter>>,
		link: &ComponentLink<Self>,
		title: &str,
		is_opened: bool,
		get_value: impl Fn(&Filter, T) -> bool + 'static,
		set_value: impl Fn(&mut Filter, T) + 'static)
		-> EnumSetFilter<T>
	where
		T: Copy + Display + EnumValues + Eq + Hash + 'static,
	{
		EnumSetFilter::new(
			title.to_string(),
			is_opened,
			Rc::new(closure!(clone filter, |a| get_value(&filter.borrow(), a))),
			Rc::new(closure!(clone filter, clone link, |a|
				{
					set_value(&mut *filter.borrow_mut(), a);
					link.send_message(Message::ChangedFilter{ repaint: false });
				})))
	}
}

pub enum Message
{
	ToggleState,
	ChangedFilter{ repaint: bool },
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
			filter_method_filter: Self::create_enum_filter(&filter, &link, "Szűrés módja", Filter::get_filter_method, Filter::set_filter_method),
			expansion_pack_filter: Self::create_enum_filter(&filter, &link, "Kiegészítő", Filter::get_expansion_pack, Filter::set_expansion_pack),
			map_categories_filter: Self::create_enum_set_filter(&filter, &link, "Kategóriák", true, Filter::is_allowed_map_category, Filter::toggle_allowed_map_category),
			wood_amount_filter: Self::create_enum_set_filter(&filter, &link, "Fa mennyisége", false, Filter::is_allowed_wood_amount, Filter::toggle_allowed_wood_amount),
			food_amount_filter: Self::create_enum_set_filter(&filter, &link, "Táplálék mennyisége", false, Filter::is_allowed_food_amount, Filter::toggle_allowed_food_amount),
			gold_amount_filter: Self::create_enum_set_filter(&filter, &link, "Arany mennyisége", false, Filter::is_allowed_gold_amount, Filter::toggle_allowed_gold_amount),
			stone_amount_filter: Self::create_enum_set_filter(&filter, &link, "Kő mennyisége", false, Filter::is_allowed_stone_amount, Filter::toggle_allowed_stone_amount),
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
			Message::ChangedFilter{repaint} =>
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
				{ self.map_categories_filter.render() }
				{ self.wood_amount_filter.render() }
				{ self.food_amount_filter.render() }
				{ self.gold_amount_filter.render() }
				{ self.stone_amount_filter.render() }
				</div>
			</div>
		}
	}
}
