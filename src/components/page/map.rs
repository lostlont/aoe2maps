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
		Properties,
	},
	crate::
	{
		agents::
		{
			filter::FilterView,
			localization::Text,
			settings::{ Response, Settings },
		},
		data::
		{
			filter_method::FilterMethod,
			map_attribute::ResourceAmount,
			map_data::MapData,
		},
	},
};

#[derive(Clone, Copy, PartialEq)]
pub enum State
{
	Default,
	Visible,
	Hidden,
	Disabled,
}

#[derive(Properties, Clone)]
pub struct MapProperties
{
	pub map_data: MapData,

	#[prop_or(State::Default)]
	pub state: State,
}

pub struct Map
{
	properties: MapProperties,
	_settings: Box<dyn Bridge<Settings>>,
	state: State,
}

impl Map
{
	fn row_class(&self) -> String
	{
		let mut classes = vec!["row"];

		match self.state
		{
			State::Hidden => classes.push("hidden"),
			State::Disabled => classes.push("disabled"),
			_ => {},
		};

		classes.join(" ")
	}

	fn render_features(&self) -> Html
	{
		fn localize_feature(resource: Text, amount: ResourceAmount) -> String
		{
			let text = Text::new_id_args("feature", hashmap!
			{
				"key" => resource,
				"value" => amount.into(),
			});
			text.localize()
		}

		html!
		{
			<ul>
				<li>{ localize_feature(Text::new_id("resource-wood"), self.properties.map_data.wood_amount()) }</li>
				<li>{ localize_feature(Text::new_id("resource-food"), self.properties.map_data.food_amount()) }</li>
				<li>{ localize_feature(Text::new_id("resource-gold"), self.properties.map_data.gold_amount()) }</li>
				<li>{ localize_feature(Text::new_id("resource-stone"), self.properties.map_data.stone_amount()) }</li>
			</ul>
		}
	}

	fn update(&mut self, filter: Rc<RefCell<dyn FilterView>>)
	{
		let filter = filter.borrow();
		let is_allowed = filter.is_allowed(&self.properties.map_data);
		if is_allowed && (self.state != State::Default)
		{
			self.state = State::Visible;
		}
		else if !is_allowed
		{
			self.state = match filter.get_filter_method()
			{
				FilterMethod::Hide => State::Hidden,
				FilterMethod::Disable => State::Disabled,
				FilterMethod::Mixed => if filter.is_allowed_by_others(&self.properties.map_data)
				{
					State::Disabled
				}
				else
				{
					State::Hidden
				},
			};
		}
	}
}

pub enum Message
{
	None,
	FilterChanged(Rc<RefCell<dyn FilterView>>),
}

impl Component for Map
{
	type Message = Message;
	type Properties = MapProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let callback = |response| match response
		{
			Response::FilterChanged(filter) => Message::FilterChanged(filter),
			_ => Message::None,
		};

		let state = properties.state;
		Self
		{
			properties,
			_settings: Settings::bridge(link.callback(callback)),
			state,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::None => {},
			Message::FilterChanged(filter) => self.update(filter),
		}

		true
	}

	fn change(&mut self, properties: Self::Properties) -> ShouldRender
	{
		self.properties = properties;
		true
	}

	fn view(&self) -> Html
	{
		html!
		{
			<div class=self.row_class()>
				<div><h3>{ self.properties.map_data.name() }</h3></div>
				<div><img src=self.properties.map_data.image() /></div>
				<div>{ self.properties.map_data.expansion_pack() }</div>
				<div>{ self.render_features() }</div>
			</div>
		}
	}
}
