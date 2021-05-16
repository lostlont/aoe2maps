use
{
	std::
	{
		cell::RefCell,
		collections::HashMap,
		rc::Rc,
	},
	yew::prelude::*,
	crate::
	{
		components::utils::accordion::Accordion,
		data::
		{
			map_attribute::MapAttribute,
			map_attribute_set::MapAttributeSet,
		},
		views::filter_checkbox::FilterCheckbox,
	},
};

#[derive(Properties, Clone)]
pub struct MapAttributeSetFilterProperties<TMapAttribute>
where
	TMapAttribute: MapAttribute,
{
	pub title: String,

	pub map_attribute_set: Rc<RefCell<MapAttributeSet<TMapAttribute>>>,

	#[prop_or(false)]
	pub is_incremental: bool,

	pub changed: Callback<()>,
}

// TODO Replace MapAttributeSetFilter<T> with EnumFilter<T>!
pub struct MapAttributeSetFilter<TMapAttribute>
where
	TMapAttribute: MapAttribute,
{
	properties: MapAttributeSetFilterProperties<TMapAttribute>,
	checkboxes: HashMap<TMapAttribute, FilterCheckbox>,
}

pub enum Message<TMapAttribute>
{
	Toggled(TMapAttribute),
}

impl<TMapAttribute> Component for MapAttributeSetFilter<TMapAttribute>
where
	TMapAttribute: MapAttribute,
{
	type Message = Message<TMapAttribute>;
	type Properties = MapAttributeSetFilterProperties<TMapAttribute>;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		let checkboxes = TMapAttribute
			::values()
			.map(|a| (a.clone(), FilterCheckbox::new(&format!("{}", &a), link.callback(move |_| Message::Toggled(a.clone())))))
			.collect();

		Self
		{
			properties,
			checkboxes,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::Toggled(map_attribute) =>
			{
				let mut setter = self.properties.map_attribute_set.borrow_mut();
				let is_allowed = !setter.contains(&map_attribute);

				if self.properties.is_incremental
				{
					let mut is_in_sequence = true;
					for attribute in TMapAttribute::values()
					{
						if is_in_sequence && *attribute != map_attribute
						{
							setter.set(attribute.clone(), true);
						}
						else if is_in_sequence
						{
							setter.set(attribute.clone(), is_allowed);

							is_in_sequence = false;
						}
						else
						{
							setter.set(attribute.clone(), false);
						}
					}
				}
				else
				{
					setter.set(map_attribute, is_allowed);
				}

				self.properties.changed.emit(());
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
		let is_checked = |attribute_value|
		{
			self.properties.map_attribute_set.borrow().contains(attribute_value)
		};

		html!
		{
			<Accordion title=&self.properties.title>
			{
				for self.checkboxes.iter().map(|(a, ch)| ch.render(is_checked(a)))
			}
			</Accordion>
		}
	}
}
