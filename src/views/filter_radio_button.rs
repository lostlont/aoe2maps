use
{
	yew::prelude::*,
};

pub struct FilterRadioButton
{
	name: String,
	is_checked: Box<dyn Fn() -> bool>,
	toggle: Callback<MouseEvent>,
}

impl FilterRadioButton
{
	pub fn new(name: &str, is_checked: Box<dyn Fn() -> bool>, toggle: Box<dyn Fn()>) -> Self
	{
		Self
		{
			name: name.to_string(),
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
				{ &self.name }
			</label>
		}
	}
}
