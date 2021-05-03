use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	yew::
	{	agent::Dispatcher,
		prelude::*,
	},
	crate::
	{
		agents::filter::{ Filter, FilterView },
		agents::settings::{ Request, Settings },
		data::map_attribute::WaterPresence,
	},
};

#[derive(Properties, Clone)]
pub struct MapAttributeFilterProperties
{
	pub filter: Rc<RefCell<Filter>>,
	pub map_attribute: WaterPresence,
}

pub struct MapAttributeFilter
{
	properties: MapAttributeFilterProperties,
	link: ComponentLink<Self>,
	settings: Dispatcher<Settings>,
}

pub enum Message
{
	ToggleAllowed,
}

impl Component for MapAttributeFilter
{
	type Message = Message;
	type Properties = MapAttributeFilterProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties,
			link,
			settings: Settings::dispatcher(),
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::ToggleAllowed =>
			{
				self.properties.filter.borrow_mut().toggle(self.properties.map_attribute.clone());
				self.settings.send(Request::FilterChanged(self.properties.filter.clone()));

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
			<label>
				<input
					type="checkbox"
					value=self.properties.map_attribute
					checked=self.properties.filter.borrow().allowed_water_presence().contains(&self.properties.map_attribute)
					onclick=self.link.callback(|_| Message::ToggleAllowed) />
				{ &self.properties.map_attribute }
			</label>
		}
	}
}
