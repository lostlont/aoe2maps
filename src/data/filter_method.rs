use
{
	std::
	{
		fmt::{ Error, Formatter, Display },
		slice::Iter,
	},
	crate::agents::localization::Text,
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
			FilterMethod::Hide => "filter-method-hide",
			FilterMethod::Disable => "filter-method-disable",
			FilterMethod::Mixed => "filter-method-mixed",
		}
	}
}

impl Default for FilterMethod
{
	fn default() -> Self
	{
		FilterMethod::Mixed
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

impl Into<Text> for FilterMethod
{
	fn into(self) -> Text
	{
		Text::new_id(self.to_string())
	}
}
