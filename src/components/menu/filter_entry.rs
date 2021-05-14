use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode
{
	CheckBox,
	RadioButton,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Properties
{
	pub name: String,

	#[prop_or(Mode::CheckBox)]
	pub mode: Mode,

	pub is_selected: bool,

	pub toggled: Callback<bool>,
}

pub struct FilterEntry
{
	properties: Properties,
	link: ComponentLink<Self>,
}

impl FilterEntry
{
	fn mode(&self) -> &str
	{
		if self.properties.mode == Mode::CheckBox
		{
			"checkbox"
		}
		else
		{
			"radio"
		}
	}
}

pub enum Message
{
	ToggleSelected,
}

impl Component for FilterEntry
{
	type Message = Message;
	type Properties = Properties;

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
			Message::ToggleSelected =>
			{
				self.properties.is_selected = !self.properties.is_selected;
				self.properties.toggled.emit(self.properties.is_selected);

				false
			},
		}
	}

	fn change(&mut self, properties: Self::Properties) -> ShouldRender
	{
		let is_changed = properties != self.properties;
		if is_changed
		{
			self.properties = properties;
		}

		is_changed
	}

	fn view(&self) -> Html
	{
		html!
		{
			<label>
				<input
					type=self.mode()
					checked=self.properties.is_selected
					onclick=self.link.callback(|_| Message::ToggleSelected)
				/>
				{ &self.properties.name }
			</label>
		}
	}
}
