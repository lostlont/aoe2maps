use
{
	yew::prelude::*,
	crate::
	{
		components::menu::Menu,
		data::map_data::MapData,
	},
	super::table::Table,
};

#[derive(Properties, Clone)]
pub struct PageProperties
{
	pub maps: Vec<MapData>,
}

pub struct Page
{
	properties: PageProperties,
}

impl Component for Page
{
	type Message = ();
	type Properties = PageProperties;

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
			<>
				<Table maps=&self.properties.maps />
				<Menu />
			</>
		}
	}
}
