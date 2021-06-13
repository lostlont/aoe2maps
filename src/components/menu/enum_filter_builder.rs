use
{
	std::
	{
		cell::RefCell,
		fmt::Display,
		hash::Hash,
		rc::Rc,
	},
	closure::closure,
	yew::ComponentLink,
	super::Message,
	crate::
	{
		agents::
		{
			filter::Filter,
			localization::Text,
		},
		components::menu::Menu,
		data::enum_values::EnumValues,
		views::enum_filter::EnumFilter,
	},
};

pub struct EnumFilterBuilder
{
	filter: Rc<RefCell<Filter>>,
	link: ComponentLink<Menu>,
	title: Text,
}

impl EnumFilterBuilder
{
	pub fn new(filter: &Rc<RefCell<Filter>>, link: &ComponentLink<Menu>) -> Self
	{
		Self
		{
			filter: filter.clone(),
			link: link.clone(),
			title: Text::default(),
		}
	}

	pub fn with_title(&mut self, title: Text) -> &mut Self
	{
		self.title = title;
		self
	}

	pub fn build<T>(
		&self,
		get_value: impl Fn(&Filter) -> T + 'static,
		set_value: impl Fn(&mut Filter, T) + 'static)
		-> EnumFilter<T>
	where
		T: Copy + Display + EnumValues + Eq + Hash + Into<Text> + 'static,
	{
		EnumFilter::new(
			self.title.clone(),
			Rc::new(closure!(clone self.filter, || get_value(&filter.borrow()))),
			Rc::new(closure!(clone self.filter, clone self.link, |v|
				{
					set_value(&mut *filter.borrow_mut(), v);
					link.send_message(Message::ChangedFilter{ repaint: true });
				})))
	}
}
