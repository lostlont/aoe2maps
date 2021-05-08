use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct MapAttributeFilterProperties
{
	pub name: String,
	pub is_allowed: bool,
	pub toggled: Callback<bool>,
}

pub struct MapAttributeFilter
{
	properties: MapAttributeFilterProperties,
	link: ComponentLink<Self>,
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
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::ToggleAllowed =>
			{
				self.properties.is_allowed = !self.properties.is_allowed;
				self.properties.toggled.emit(self.properties.is_allowed);

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
					checked=self.properties.is_allowed
					onclick=self.link.callback(|_| Message::ToggleAllowed) />
				{ &self.properties.name }
			</label>
		}
	}
}
