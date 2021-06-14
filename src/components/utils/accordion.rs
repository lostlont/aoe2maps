use
{
	yew::prelude::*,
	web_sys::HtmlElement,
};

#[derive(Properties, Clone, PartialEq)]
pub struct AccordionProperties
{
	pub children: Children,

	pub title: String,

	#[prop_or(true)]
	pub is_opened: bool,
}

pub struct Accordion
{
	properties: AccordionProperties,
	link: ComponentLink<Self>,
	node_ref: NodeRef,
	is_opened: bool,
}

pub enum Message
{
	Toggle,
}

impl Accordion
{
	fn content_class(&self) -> &str
	{
		if self.is_opened
		{
			"opened"
		}
		else
		{
			"closed"
		}
	}

	fn update_content_style(&self)
	{
		if let Some(element) = self.node_ref.cast::<HtmlElement>()
		{
			let max_height = if self.is_opened
			{
				 element.scroll_height()
			}
			else
			{
				0
			};
			let style = format!("max-height: {}px", max_height);
			element.style().set_css_text(&style);
		}
	}
}

impl Component for Accordion
{
	type Message = Message;
	type Properties = AccordionProperties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let is_opened = properties.is_opened;

		Self
		{
			properties,
			link,
			node_ref: NodeRef::default(),
			is_opened,
		}
	}

	fn update(&mut self, message: Self::Message) -> bool
	{
		match message
		{
			Message::Toggle =>
			{
				self.is_opened = !self.is_opened;
				self.update_content_style();
			}
		}
		true
	}

	fn change(&mut self, properties: Self::Properties) -> bool
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
			<div class="accordion">
				<button onclick=self.link.callback(|_| Message::Toggle)>
					<svg class=self.content_class()>
						<polygon points="0,0 5,10 10,0" />
					</svg>
					{ "\u{00A0}" }
					{ &self.properties.title }
				</button>
				<div
					ref=self.node_ref.clone()
					class=self.content_class()
				>
					{ self.properties.children.clone() }
				</div>
			</div>
		}
	}

	fn rendered(&mut self, first_render: bool)
	{
		if first_render
		{
			self.update_content_style();
		}
	}
}
