use
{
	std::
	{
		fmt::{ Display, Error, Formatter },
		result::Result,
		slice::Iter,
	},
	crate::agents::localization::Text,
	super::{ EnumValues, MapAttribute },
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ResourceAmount
{
	Scarce,
	Average,
	Plentiful,
}

impl ResourceAmount
{
	pub fn name(&self) -> &str
	{
		match self
		{
			ResourceAmount::Scarce => "resource-amount-scarce",
			ResourceAmount::Average => "resource-amount-average",
			ResourceAmount::Plentiful => "resource-amount-plentiful",
		}
	}
}

impl EnumValues for ResourceAmount
{
	fn values() -> Iter<'static, ResourceAmount>
	{
		static VALUES: [ResourceAmount; 3] = [
			ResourceAmount::Scarce,
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
		];
		VALUES.iter()
	}
}

impl Display for ResourceAmount
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", self.name())
	}
}

impl Into<Text> for ResourceAmount
{
	fn into(self) -> Text
	{
		Text::new_id(self.to_string())
	}
}

impl MapAttribute for ResourceAmount
{
}
