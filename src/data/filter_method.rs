use
{
	std::
	{
		fmt::{ Error, Formatter, Display },
		slice::Iter,
	},
	super::enum_values::EnumValues,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FilterMethod
{
	Hide,
	Disable,
	Mixed,
}

impl FilterMethod
{
	fn name(&self) -> &str
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

impl EnumValues for FilterMethod
{
	fn values() -> Iter<'static, Self>
	{
		static VALUES: [FilterMethod; 3] =
		[
			FilterMethod::Hide,
			FilterMethod::Disable,
			FilterMethod::Mixed,
		];
		VALUES.iter()
	}
}
