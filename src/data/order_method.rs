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
pub enum OrderMethod
{
	Name,
	ExpansionPack,
}

impl OrderMethod
{
	fn name(&self) -> &str
	{
		match self
		{
			OrderMethod::Name => "order-method-name",
			OrderMethod::ExpansionPack => "order-method-expansion-pack",
		}
	}
}

impl Default for OrderMethod
{
	fn default() -> Self
	{
		OrderMethod::Name
	}
}

impl Display for OrderMethod
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", self.name())
	}
}

impl EnumValues for OrderMethod
{
	fn values() -> Iter<'static, Self>
	{
		static VALUES: [OrderMethod; 2] =
		[
			OrderMethod::Name,
			OrderMethod::ExpansionPack,
		];
		VALUES.iter()
	}
}

impl Into<Text> for OrderMethod
{
	fn into(self) -> Text
	{
		Text::new_id(self.to_string())
	}
}
