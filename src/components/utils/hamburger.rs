use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct HamburgerProperties
{
	pub clicked: Callback<()>,
}

pub struct Hamburger
{
	properties: HamburgerProperties,
	link: ComponentLink<Self>,
}

pub enum Message
{
	Clicked,
}

impl Component for Hamburger
{
	type Message = Message;
	type Properties = HamburgerProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties,
			link,
		}
	}

	fn update(&mut self, message: Self::Message) -> bool
	{
		match message
		{
			Message::Clicked => self.properties.clicked.emit(()),
		}
		false
	}

	fn change(&mut self, _: Self::Properties) -> bool
	{
		false
	}

	fn view(&self) -> Html
	{
		html!
		{
			<button
				class="hamburger"
				onclick=self.link.callback(|_| Message::Clicked)
			>
				<svg>
					<line x1="6" y1="6" x2="26" y2="6" />
					<line x1="6" y1="16" x2="26" y2="16" />
					<line x1="6" y1="26" x2="26" y2="26" />
				</svg>
			</button>
		}
	}
}
