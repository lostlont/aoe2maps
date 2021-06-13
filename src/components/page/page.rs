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
			settings::{ Response, Settings },
		},
		components::menu::Menu,
		data::
		{
			language::Language,
			map_data::MapData,
		},
	},
	super::table::Table,
};

#[derive(Properties, Clone)]
pub struct PageProperties
{
	pub maps: Vec<MapData>,
}

pub struct Page
{
	properties: PageProperties,
	_settings: Box<dyn Bridge<Settings>>,
	language: Language,
}

pub enum Message
{
	None,
	FilterChanged(Rc<RefCell<dyn FilterView>>),
}

impl Component for Page
{
	type Message = Message;
	type Properties = PageProperties;

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
			language: Language::default(),
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::None => false,
			Message::FilterChanged(filter) =>
			{
				let language = filter.borrow().get_language();
				if language != self.language
				{
					self.language = language;
					true
				}
				else
				{
					false
				}
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
			<>
				<Table maps=&self.properties.maps />
				<Menu />
			</>
		}
	}
}
