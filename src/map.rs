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
		Properties,
	},
	crate::
	{
		agents::
		{
			filter::FilterView,
			settings::{ Response, Settings },
		},
		data::map_data::MapData,
	},
};

#[derive(Clone, PartialEq)]
pub enum State
{
	Default,
	Visible,
	Hidden,
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
}

impl Map
{
	fn row_class(&self) -> String
	{
		let mut classes = vec!["row"];

		match self.properties.state
		{
			State::Hidden => classes.push("hidden"),
			_ => {},
		};

		classes.join(" ")
	}

	fn render_features(&self) -> Html
	{
		html!
		{
			<ul>
				<li>{ &self.properties.map_data.water_presence }</li>
			</ul>
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

		Self
		{
			properties,
			_settings: Settings::bridge(link.callback(callback)),
		}
	}

	fn update(&mut self, message: Self::Message) -> bool
	{
		match message
		{
			Message::None => {},
			Message::FilterChanged(filter) =>
			{
				let visible = filter.borrow().is_allowed(&self.properties.map_data);
				if visible && (self.properties.state != State::Default)
				{
					self.properties.state = State::Visible;
				}
				else if !visible
				{
					self.properties.state = State::Hidden;
				}
			},
		}

		true
	}

	fn change(&mut self, _: Self::Properties) -> bool
	{
		false
	}

	fn view(&self) -> Html
	{
		html!
		{
			<div class=self.row_class()>
				<div><h3>{ &self.properties.map_data.name }</h3></div>
				<div><img src=&self.properties.map_data.image /></div>
				<div>{ &self.properties.map_data.expansion_pack }</div>
				<div>{ self.render_features() }</div>
			</div>
		}
	}
}
