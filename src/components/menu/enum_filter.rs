use
{
	std::
	{
		fmt::Display,
		hash::Hash,
	},
	yew::prelude::*,
	crate::
	{
		components::utils::accordion::Accordion,
		views::filter_radio_button::FilterRadioButton,
	},
};

#[derive(Clone, Properties)]
pub struct Properties<TEnum>
where
	TEnum: Clone,
{
	pub title: String,
	pub values: Vec<TEnum>,
	pub current_value: TEnum,
	pub on_selected: Callback<TEnum>,
}

pub struct EnumFilter<TEnum>
where
	TEnum: Clone,
{
	properties: Properties<TEnum>,
	buttons: Vec<(TEnum, FilterRadioButton)>,
}

impl<TEnum> EnumFilter<TEnum>
where
	TEnum: Clone + Copy + Display + Eq + Hash + 'static,
{
	fn register_button(value: TEnum, link: &ComponentLink<Self>) -> FilterRadioButton
	{
		let name = format!("{}", value);
		FilterRadioButton::new(&name, link.callback(move |_| value))
	}
}

impl<TEnum> Component for EnumFilter<TEnum>
where
	TEnum: Clone + Copy + Display + Eq + Hash + 'static,
{
	type Message = TEnum;
	type Properties = Properties<TEnum>;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let values = properties.values
			.iter()
			.copied();
		let buttons = properties.values
			.iter()
			.map(|v| Self::register_button(*v, &link));
		let buttons = values
			.zip(buttons)
			.collect();

		Self
		{
			properties,
			buttons,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		self.properties.current_value = message;
		self.properties.on_selected.emit(message);
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender
	{
		false
	}

	fn view(&self) -> Html
	{
		html!
		{
			<Accordion title=&self.properties.title>
			{
				for self.buttons.iter().map(|(v, rb)| rb.render(self.properties.current_value == *v))
			}
			</Accordion>
		}
	}
}
