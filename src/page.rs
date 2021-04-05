use yew::prelude::*;

use crate::map::Map;

pub struct Page
{
}

impl Component for Page
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
		let render_map = |_| html!
		{
			<Map name="test"/>
		};

		html!
		{
			<table>
				<tr>
					<th>{ "Név" }</th>
					<th>{ "Kép" }</th>
					<th>{ "Jellemzők" }</th>
				</tr>
				{ for (0..3).map(render_map) }
			</table>
		}
	}
}
