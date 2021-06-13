use
{
	yew::prelude::*,
	crate::agents::localization::Text,
};

pub struct FilterCheckbox
{
	name: Text,
	is_checked: Box<dyn Fn() -> bool>,
	toggle: Callback<MouseEvent>,
}

impl FilterCheckbox
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

impl Renderable for FilterCheckbox
{
	fn render(&self) -> Html
	{
		html!
		{
			<label>
				<input
					type="checkbox"
					checked=(*self.is_checked)()
					onclick=self.toggle.clone()
				/>
				{ self.name.localize() }
			</label>
		}
	}
}
