use
{
	std::
	{
		fmt::{ Display, Error, Formatter },
		result::Result,
		slice::Iter,
	},
	super::{ EnumValues, MapAttribute },
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum MapCategory
{
	BlindRandom,
	RandomLandMap,
	FullRandom,
	LandMap,
	MixedMap,
	WaterMap,
}

impl EnumValues for MapCategory
{
	fn values() -> Iter<'static, Self>
	{
		static VALUES: [MapCategory; 6] = [
			MapCategory::BlindRandom,
			MapCategory::RandomLandMap,
			MapCategory::FullRandom,
			MapCategory::LandMap,
			MapCategory::MixedMap,
			MapCategory::WaterMap,
		];
		VALUES.iter()
	}
}

impl Display for MapCategory
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		let description = match self
		{
			MapCategory::BlindRandom => "Blind Random (HD)",
			MapCategory::RandomLandMap => "Random Land Map (HD)",
			MapCategory::FullRandom => "Full Random (HD)",
			MapCategory::LandMap => "Land Map (DE)",
			MapCategory::MixedMap => "Mixed Map (DE)",
			MapCategory::WaterMap => "Water Map (DE)",
		};

		write!(formatter, "{}", description)
	}
}

impl MapAttribute for MapCategory
{
}
