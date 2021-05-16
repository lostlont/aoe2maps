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
		components::
		{
			menu::
			{
				enum_filter::EnumFilter,
				filter_method_selector::FilterMethodSelector,
			},
			utils::hamburger::Hamburger,
		},
		data::
		{
			enum_values::EnumValues,
			map_attribute::{ ExpansionPack, ResourceAmount, WaterPresence },
		},
	},
	super::map_attribute_set_filter::MapAttributeSetFilter,
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
	ChangedExpansionPack(ExpansionPack),
	ChangedMapAttribute,
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
			Message::ChangedExpansionPack(expansion_pack) =>
			{
				self.filter.borrow_mut().set_expansion_pack(expansion_pack);
				self.settings.send(Request::FilterChanged(self.filter.clone()));
				false
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
					<FilterMethodSelector
						filter=self.filter.clone()
					/>
					<EnumFilter<ExpansionPack>
						title="Szűrés módja"
						values=ExpansionPack::values().copied().collect::<Vec<_>>()
						current_value=ExpansionPack::values().skip(1).next().unwrap()
						on_selected=self.link.callback(Message::ChangedExpansionPack)
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
