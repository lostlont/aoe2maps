use
{
	std::
	{
		fmt::Display,
		hash::Hash,
		rc::Rc,
	},
	yew::prelude::*,
	crate::
	{
		components::utils::accordion::Accordion,
		data::enum_values::EnumValues,
		views::filter_radio_button::FilterRadioButton,
	},
};

pub struct EnumFilter<TEnum>
where
	TEnum: Clone,
{
	title: String,
	buttons: Vec<(TEnum, FilterRadioButton)>,
}

impl<TEnum> EnumFilter<TEnum>
where
	TEnum: Clone + Copy + Display + EnumValues + Eq + Hash + 'static,
{
	pub fn new(title: String, get_value: Rc<dyn Fn() -> TEnum>, set_value: Rc<dyn Fn(TEnum)>) -> Self
	{
		let values = TEnum
			::values()
			.copied();
		let buttons = TEnum
			::values()
			.map(|v| Self::create_button(*v, get_value.clone(), set_value.clone()));
		let buttons = values
			.zip(buttons)
			.collect();

		Self
		{
			title,
			buttons,
		}
	}

	fn create_button(value: TEnum, get_value: Rc<dyn Fn() -> TEnum>, set_value: Rc<dyn Fn(TEnum)>) -> FilterRadioButton
	{
		let name = format!("{}", value);
		let is_checked = Box::new(move || get_value() == value);
		let toggle = Box::new(move || set_value(value));
		FilterRadioButton::new(&name, is_checked, toggle)
	}

	pub fn render(&self) -> Html
	{
		html!
		{
			<Accordion title=&self.title>
			{
				for self.buttons
					.iter()
					.map(|(v, rb)| rb.render())
			}
			</Accordion>
		}
	}
}
