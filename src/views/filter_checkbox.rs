use
{
	yew::prelude::*,
};

pub struct FilterCheckbox
{
	name: String,
	on_click: Callback<MouseEvent>,
}

impl FilterCheckbox
{
	pub fn new(name: &str, on_click: Callback<()>) -> Self
	{
		Self
		{
			name: name.to_string(),
			on_click: Callback::from(move |_| on_click.emit(())),
		}
	}

	pub fn render(&self, is_checked: bool) -> Html
	{
		html!
		{
			<label>
				<input
					type="checkbox"
					checked=is_checked
					onclick=self.on_click.clone()
				/>
				{ &self.name }
			</label>
		}
	}
}
