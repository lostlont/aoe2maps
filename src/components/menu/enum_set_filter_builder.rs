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
		views::enum_set_filter::EnumSetFilter,
	},
};

pub struct EnumSetFilterBuilder
{
	filter: Rc<RefCell<Filter>>,
	link: ComponentLink<Menu>,
	title: Text,
	is_opened: bool,
}

impl EnumSetFilterBuilder
{
	pub fn new(filter: &Rc<RefCell<Filter>>, link: &ComponentLink<Menu>) -> Self
	{
		Self
		{
			filter: filter.clone(),
			link: link.clone(),
			title: Text::default(),
			is_opened: true,
		}
	}

	pub fn with_title(&mut self, title: Text) -> &mut Self
	{
		self.title = title;
		self
	}

	pub fn set_opened(&mut self, is_opened: bool) -> &mut Self
	{
		self.is_opened = is_opened;
		self
	}

	pub fn build<T>(
		&mut self,
		get_value: impl Fn(&Filter, T) -> bool + 'static,
		set_value: impl Fn(&mut Filter, T) + 'static)
		-> EnumSetFilter<T>
	where
		T: Copy + Display + EnumValues + Eq + Hash + Into<Text> + 'static,
	{
		EnumSetFilter::new(
			self.title.clone(),
			self.is_opened,
			Rc::new(closure!(clone self.filter, |a| get_value(&filter.borrow(), a))),
			Rc::new(closure!(clone self.filter, clone self.link, |a|
				{
					set_value(&mut *filter.borrow_mut(), a);
					link.send_message(Message::ChangedFilter{ repaint: false });
				})))
	}
}
