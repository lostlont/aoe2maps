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
		components::
		{
			menu::filter_entry::{ FilterEntry, FilterEntryType },
			utils::accordion::Accordion,
		},
		data::filter_method::FilterMethod,
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
	link: ComponentLink<Self>,
	settings: Dispatcher<Settings>,
}

impl FilterMethodSelector
{
	fn render_entry(&self, filter_method: FilterMethod) -> Html
	{
		html!
		{
			<FilterEntry
				name=format!("{}", filter_method)
				entry_type=FilterEntryType::RadioButton
				is_selected=self.properties.filter.borrow().filter_method() == filter_method
				toggled=self.link.callback(move |_| Message::Selected(filter_method))
			/>
		}
	}
}

pub enum Message
{
	Selected(FilterMethod),
}

impl Component for FilterMethodSelector
{
	type Message = Message;
	type Properties = Properties;

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
		html!
		{
			<Accordion title="Szűrés módja">
			{ self.render_entry(FilterMethod::Hide) }
			{ self.render_entry(FilterMethod::Disable) }
			{ self.render_entry(FilterMethod::Mixed) }
			</Accordion>
		}
	}
}
