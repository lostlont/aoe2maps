use
{
	std::
	{
		cell::RefCell,
		rc::Rc,
	},
	yew::prelude::*,
	crate::
	{
		data::
		{
			map_attribute::MapAttribute,
			map_attribute_set::MapAttributeSet,
		},
		map_attribute_filter::MapAttributeFilter,
		utils::accordion::Accordion,
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

pub struct MapAttributeSetFilter<TMapAttribute>
where
	TMapAttribute: MapAttribute,
{
	properties: MapAttributeSetFilterProperties<TMapAttribute>,
	link: ComponentLink<Self>,
}

pub enum Message<TMapAttribute>
{
	Toggled(TMapAttribute, bool),
}

impl<TMapAttribute> Component for MapAttributeSetFilter<TMapAttribute>
where
	TMapAttribute: MapAttribute,
{
	type Message = Message<TMapAttribute>;
	type Properties = MapAttributeSetFilterProperties<TMapAttribute>;

	fn create(properties: Self::Properties, link: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties,
			link,
		}
	}

	fn update(&mut self, message: Self::Message) -> ShouldRender
	{
		match message
		{
			Message::Toggled(map_attribute, is_allowed) =>
			{
				let mut setter = self.properties.map_attribute_set.borrow_mut();

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
		html!
		{
			<Accordion title=&self.properties.title>
			{
				for TMapAttribute::values().map(|attribute_value| html!
					{
						<MapAttributeFilter
							name=format!("{}", attribute_value)
							is_allowed=self.properties.map_attribute_set.borrow().contains(&attribute_value)
							toggled=self.link.callback(move |is_allowed| Message::Toggled(attribute_value.clone(), is_allowed)) />
					})
			}
			</Accordion>
		}
	}
}
