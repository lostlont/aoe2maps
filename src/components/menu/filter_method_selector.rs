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
		agent::Dispatcher,
	},
	crate::
	{
		agents::
		{
			filter::{ Filter, FilterView },
			settings::{ Request, Settings },
		},
		components::utils::accordion::Accordion,
		data::
		{
			enum_values::EnumValues,
			filter_method::FilterMethod,
		},
		views::filter_radio_button::FilterRadioButton,
	},
};

#[derive(Clone, Properties)]
pub struct Properties
{
	pub filter: Rc<RefCell<Filter>>,
}

pub struct FilterMethodSelector
{
	properties: Properties,
	settings: Dispatcher<Settings>,
	buttons: Vec<(FilterMethod, FilterRadioButton)>,
}

pub enum Message
{
	Selected(FilterMethod),
}

impl FilterMethodSelector
{
	fn register_button(value: FilterMethod, link: &ComponentLink<Self>) -> FilterRadioButton
	{
		let name = format!("{}", value);
		FilterRadioButton::new(&name, link.callback(move |_| Message::Selected(value)))
	}
}

impl Component for FilterMethodSelector
{
	type Message = Message;
	type Properties = Properties;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let buttons = FilterMethod
			::values()
			.map(|fm| Self::register_button(*fm, &link));
		let buttons = FilterMethod
			::values()
			.copied()
			.zip(buttons)
			.collect();

		Self
		{
			properties,
			settings: Settings::dispatcher(),
			buttons,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::Selected(filter_method) =>
			{
				self.properties.filter.borrow_mut().set_filter_method(filter_method);
				self.settings.send(Request::FilterChanged(self.properties.filter.clone()));
				true
			},
		}
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender
	{
		false
	}

	fn view(&self) -> Html
	{
		let is_checked = |filter_method: FilterMethod| self.properties.filter.borrow().get_filter_method() == filter_method;

		html!
		{
			<Accordion title="Szűrés módja">
			{
				for self.buttons.iter().map(|(fm, rb)| rb.render(is_checked(*fm)))
			}
			</Accordion>
		}
	}
}
