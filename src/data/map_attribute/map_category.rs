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
	NomadMap,
	MigrationMap,
	OpenMap,
	ClosedMap,
}

impl EnumValues for MapCategory
{
	fn values() -> Iter<'static, Self>
	{
		static VALUES: [MapCategory; 10] = [
			MapCategory::BlindRandom,
			MapCategory::RandomLandMap,
			MapCategory::FullRandom,
			MapCategory::LandMap,
			MapCategory::MixedMap,
			MapCategory::WaterMap,
			MapCategory::NomadMap,
			MapCategory::MigrationMap,
			MapCategory::OpenMap,
			MapCategory::ClosedMap,
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
			MapCategory::NomadMap => "Nomad Map (DE)",
			MapCategory::MigrationMap => "Migration Map (DE)",
			MapCategory::OpenMap => "Open Map (DE)",
			MapCategory::ClosedMap => "Closed Map (DE)",
		};

		write!(formatter, "{}", description)
	}
}

impl MapAttribute for MapCategory
{
}
