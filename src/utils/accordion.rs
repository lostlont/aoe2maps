use
{
	yew::prelude::*,
	web_sys::HtmlElement,
};

#[derive(Properties, Clone)]
pub struct AccordionProperties
{
	pub children: Children,
	pub title: String,
}

pub struct Accordion
{
	properties: AccordionProperties,
	link: ComponentLink<Self>,
	node_ref: NodeRef,
	opened: bool,
}

pub enum Message
{
	Toggle,
}

impl Accordion
{
	fn button_icon(&self) -> &str
	{
		if self.opened
		{
			"⯆"
		}
		else
		{
			"⯈"
		}
	}

	fn content_class(&self) -> &str
	{
		if self.opened
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
			let max_height = if self.opened
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
		Self
		{
			properties,
			link,
			node_ref: NodeRef::default(),
			opened: true,
		}
	}

	fn update(&mut self, message: Self::Message) -> bool
	{
		match message
		{
			Message::Toggle =>
			{
				self.opened = !self.opened;
				self.update_content_style();
			}
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
			<div class="accordion">
				<button
					onclick=self.link.callback(|_| Message::Toggle)>
					{ self.button_icon() }
					{ "\u{00A0}" }
					{ &self.properties.title }
				</button>
				<div
					ref=self.node_ref.clone()
					class=self.content_class()>
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
