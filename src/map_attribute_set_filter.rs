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
		agents::filter::Filter,
		data::map_attribute::{ MapAttribute, WaterPresence },
		map_attribute_filter::MapAttributeFilter,
		utils::accordion::Accordion,
	},
};

#[derive(Properties, Clone)]
pub struct MapAttributeSetFilterProperties
{
	pub title: String,
	pub filter: Rc<RefCell<Filter>>,
}

pub struct MapAttributeSetFilter
{
	properties: MapAttributeSetFilterProperties,
}

impl Component for MapAttributeSetFilter
{
	type Message = ();
	type Properties = MapAttributeSetFilterProperties;

	fn create(properties: Self::Properties, _: ComponentLink<Self>) -> Self
	{
		Self
		{
			properties,
		}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender
	{
		false
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
				for WaterPresence::values().map(|attribute_value| html!
					{
						<MapAttributeFilter
							filter=self.properties.filter.clone()
							map_attribute=attribute_value />
					})
			}
			</Accordion>
		}
	}
}
