use
{
	yew::prelude::*,
};

pub struct FilterCheckbox
{
	name: String,
	is_checked: Box<dyn Fn() -> bool>,
	toggle: Callback<MouseEvent>,
}

impl FilterCheckbox
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
				{ &self.name }
			</label>
		}
	}
}
