use
{
	yew::prelude::*,
	crate::agents::localization::Text,
};

pub struct FilterRadioButton
{
	name: Text,
	is_checked: Box<dyn Fn() -> bool>,
	toggle: Callback<MouseEvent>,
}

impl FilterRadioButton
{
	pub fn new(name: Text, is_checked: Box<dyn Fn() -> bool>, toggle: Box<dyn Fn()>) -> Self
	{
		Self
		{
			name,
			is_checked,
			toggle: Callback::from(move |_| (*toggle)()),
		}
	}
}

impl Renderable for FilterRadioButton
{
	fn render(&self) -> Html
	{
		html!
		{
			<label>
				<input
					type="radio"
					checked=(*self.is_checked)()
					onclick=self.toggle.clone()
				/>
				{ self.name.localize() }
			</label>
		}
	}
}
