use
{
	std::
	{
		fmt::{ Display, Error, Formatter },
		result::Result,
	},
	core::slice::Iter,
	super::{ EnumValues, MapAttribute },
};

#[derive(Clone, Eq, Hash, PartialEq)]
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
			ResourceAmount::Scarce => "Kevés",
			ResourceAmount::Average => "Átlagos",
			ResourceAmount::Plentiful => "Bőséges",
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

impl MapAttribute for ResourceAmount
{
}
