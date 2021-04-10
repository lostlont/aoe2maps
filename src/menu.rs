use yew::prelude::*;

pub struct Menu
{
}

impl Component for Menu
{
	type Message = ();
	type Properties = ();

	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self
	{
		Self
		{
		}
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender
	{
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
			<div class="menu">
				{ "Menu" }
			</div>
		}
	}
}
