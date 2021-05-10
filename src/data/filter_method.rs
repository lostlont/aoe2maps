use std::fmt::{ Error, Formatter, Display };

#[derive(Clone, Copy)]
pub enum FilterMethod
{
	Hide,
	Disable,
	Mixed,
}

impl FilterMethod
{
	pub fn name(&self) -> &str
	{
		match self
		{
			FilterMethod::Hide => "Elrejtés",
			FilterMethod::Disable => "Szürkítés",
			FilterMethod::Mixed => "Vegyes",
		}
	}
}

impl Display for FilterMethod
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", self.name())
	}
}
