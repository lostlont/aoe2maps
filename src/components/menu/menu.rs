use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	maplit::hashmap,
	yew::
	{
		prelude::*,
		agent::Dispatcher,
	},
	super::
	{
		enum_filter_builder::EnumFilterBuilder,
		enum_set_filter_builder::EnumSetFilterBuilder,
	},
	crate::
	{
		agents::
		{
			filter::{ Filter, FilterView },
			localization::Text,
			settings::{ MenuState, Request, Settings },
		},
		components::
		{
			utils::hamburger::Hamburger,
		},
		data::
		{
			filter_method::FilterMethod,
			language::Language,
			map_attribute::{ ExpansionPack, MapCategory, ResourceAmount },
			order_method::OrderMethod,
		},
		views::
		{
			enum_filter::EnumFilter,
			enum_set_filter::EnumSetFilter,
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
	order_method_filter: EnumFilter<OrderMethod>,
	expansion_pack_filter: EnumFilter<ExpansionPack>,
	map_categories_filter: EnumSetFilter<MapCategory>,
	wood_amount_filter: EnumSetFilter<ResourceAmount>,
	food_amount_filter: EnumSetFilter<ResourceAmount>,
	gold_amount_filter: EnumSetFilter<ResourceAmount>,
	stone_amount_filter: EnumSetFilter<ResourceAmount>,
	language_filter: EnumFilter<Language>,
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
		let mut enum_filter_builder = EnumFilterBuilder::new(&filter, &link);
		let mut enum_set_filter_builder = EnumSetFilterBuilder::new(&filter, &link);
		Self
		{
			link: link.clone(),
			settings: Settings::dispatcher(),
			filter_method_filter: enum_filter_builder
				.with_title(Text::new_id("filter-method"))
				.build(Filter::get_filter_method, Filter::set_filter_method),
			order_method_filter: enum_filter_builder
				.with_title(Text::new_id("order-method"))
				.build(Filter::get_order_method, Filter::set_order_method),
			expansion_pack_filter: enum_filter_builder
				.with_title(Text::new_id("attribute-expansion-pack"))
				.build(Filter::get_expansion_pack, Filter::set_expansion_pack),
			map_categories_filter: enum_set_filter_builder
				.with_title(Text::new_id("attribute-categories"))
				.set_opened(true)
				.build(Filter::is_allowed_map_category, Filter::toggle_allowed_map_category),
			wood_amount_filter: enum_set_filter_builder
				.with_title(Text::new_id_args(
					"attribute-resource-amount",
					hashmap!{
						"resource" => Text::new_id("resource-wood"),
					}))
				.set_opened(false)
				.build(Filter::is_allowed_wood_amount, Filter::toggle_allowed_wood_amount),
			food_amount_filter: enum_set_filter_builder
			.with_title(Text::new_id_args(
				"attribute-resource-amount",
				hashmap!{
					"resource" => Text::new_id("resource-food"),
				}))
			.set_opened(false)
				.build(Filter::is_allowed_food_amount, Filter::toggle_allowed_food_amount),
			gold_amount_filter: enum_set_filter_builder
			.with_title(Text::new_id_args(
				"attribute-resource-amount",
				hashmap!{
					"resource" => Text::new_id("resource-gold"),
				}))
			.set_opened(false)
				.build(Filter::is_allowed_gold_amount, Filter::toggle_allowed_gold_amount),
			stone_amount_filter: enum_set_filter_builder
			.with_title(Text::new_id_args(
				"attribute-resource-amount",
				hashmap!{
					"resource" => Text::new_id("resource-stone"),
				}))
			.set_opened(false)
				.build(Filter::is_allowed_stone_amount, Filter::toggle_allowed_stone_amount),
			language_filter: enum_filter_builder
				.with_title(Text::new_id("language"))
				.build(Filter::get_language, Filter::set_language),
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
		true
	}

	fn view(&self) -> Html
	{
		html!
		{
			<div class=self.class()>
				<Hamburger clicked=self.link.callback(|_| Message::ToggleState) />
				<div class="content">
				{ self.filter_method_filter.render() }
				{ self.order_method_filter.render() }
				{ self.expansion_pack_filter.render() }
				{ self.map_categories_filter.render() }
				{ self.wood_amount_filter.render() }
				{ self.food_amount_filter.render() }
				{ self.gold_amount_filter.render() }
				{ self.stone_amount_filter.render() }
				{ self.language_filter.render() }
				</div>
			</div>
		}
	}
}
