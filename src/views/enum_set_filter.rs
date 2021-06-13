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
		agents::localization::Text,
		components::utils::accordion::Accordion,
		data::enum_values::EnumValues,
		views::filter_checkbox::FilterCheckbox,
	},
};

pub struct EnumSetFilter<TEnum>
where
	TEnum: Clone,
{
	title: Text,
	is_opened: bool,
	checkboxes: Vec<(TEnum, FilterCheckbox)>,
}

impl<TEnum> EnumSetFilter<TEnum>
where
	TEnum: Clone + Copy + Display + EnumValues + Eq + Hash + Into<Text> + 'static,
{
	pub fn new(title: Text, is_opened: bool, contains_value: Rc<dyn Fn(TEnum) -> bool>, toggle_value_contained: Rc<dyn Fn(TEnum)>) -> Self
	{
		let values = TEnum
			::values()
			.copied();
		let checkboxes = TEnum
			::values()
			.map(|v| Self::create_checkbox(*v, contains_value.clone(), toggle_value_contained.clone()));
		let checkboxes = values
			.zip(checkboxes)
			.collect();

		Self
		{
			title,
			is_opened,
			checkboxes,
		}
	}

	fn create_checkbox(value: TEnum, contains_value: Rc<dyn Fn(TEnum) -> bool>, toggle_value_contained: Rc<dyn Fn(TEnum)>) -> FilterCheckbox
	{
		let name = value.into();
		let is_checked = Box::new(move || contains_value(value));
		let toggle = Box::new(move || toggle_value_contained(value));
		FilterCheckbox::new(name, is_checked, toggle)
	}

	pub fn render(&self) -> Html
	{
		html!
		{
			<Accordion
				title=self.title.localize()
				is_opened=self.is_opened
			>
			{
				for self.checkboxes
					.iter()
					.map(|(v, ch)| ch.render())
			}
			</Accordion>
		}
	}
}
