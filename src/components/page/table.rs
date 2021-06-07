use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	yew::prelude::*,
	crate::
	{
		agents::
		{
			filter::FilterView,
			settings::{ MenuState, Request, Response, Settings },
		},
		data::
		{
			enum_values::EnumValues,
			map_attribute::ExpansionPack,
			map_data::MapData,
			order_method::OrderMethod,
		},
	},
	super::map::Map,
};

#[derive(Properties, Clone)]
pub struct TableProperties
{
	pub maps: Vec<MapData>,
}

pub struct Table
{
	properties: TableProperties,
	settings: Box<dyn Bridge<Settings>>,
	menu_state: MenuState,
	order_method: OrderMethod,
}

impl Table
{
	fn render_maps(&self) -> Box<dyn Iterator<Item = Html> + '_>
	{
		match self.order_method
		{
			OrderMethod::Name =>
			{
				let result = self.properties.maps
					.iter()
					.map(Self::render_map);

				Box::new(result)
			},
			OrderMethod::ExpansionPack =>
			{
				let result = ExpansionPack
					::values()
					.copied()
					.flat_map(move |e| self.properties.maps
						.iter()
						.filter(move |m| m.expansion_pack() == e)
						.map(Self::render_map));

				Box::new(result)
			},
		}
	}

	fn render_map(map_data: &MapData) -> Html
	{
		html!
		{
			<Map map_data=map_data/>
		}
	}

	fn class(&self) -> String
	{
		let mut classes = vec!["table"];

		match self.menu_state
		{
			MenuState::Open => classes.push("menu-open"),
			MenuState::Collapsed => classes.push("menu-collapsed"),
		};

		classes.join(" ")
	}
}

pub enum Message
{
	SetMenuState(MenuState),
	SetOrderMethod(Rc<RefCell<dyn FilterView>>),
}

impl Component for Table
{
	type Message = Message;
	type Properties = TableProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let callback = |response| match response
		{
			Response::MenuStateChanged(menu_state) => Message::SetMenuState(menu_state),
			Response::FilterChanged(filter) => Message::SetOrderMethod(filter),
		};

		Self
		{
			properties,
			settings: Settings::bridge(link.callback(callback)),
			menu_state: MenuState::Open,
			order_method: OrderMethod::default(),
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::SetMenuState(menu_state) =>
			{
				self.menu_state = menu_state;
				true
			},
			Message::SetOrderMethod(filter) =>
			{
				let order_method = filter.borrow().get_order_method();
				let is_changed = order_method != self.order_method;
				if is_changed
				{
					self.order_method = order_method;
					self.settings.send(Request::FilterChanged(filter));
				}

				is_changed
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
				<div class="row">
					<div class="header"><h2>{ "Név" }</h2></div>
					<div class="header"><h2>{ "Kép" }</h2></div>
					<div class="header"><h2>{ "Kiegészítő" }</h2></div>
					<div class="header"><h2>{ "Jellemzők" }</h2></div>
				</div>
				{ for self.render_maps() }
			</div>
		}
	}
}
