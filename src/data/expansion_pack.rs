use std::
{
	fmt::
	{
		Display,
		Error,
		Formatter,
	},
	result::Result,
};

#[derive(Clone)]
pub enum ExpansionPack
{
	TheAgeOfKings,
	TheConquerors,
	TheForgotten,
	TheAfricanKingdoms,
	RiseOfTheRajas,
	TheLastKhans,
}

impl ExpansionPack
{
	pub fn name(&self) -> &str
	{
		match self
		{
			ExpansionPack::TheAgeOfKings => "The Age of Kings",
			ExpansionPack::TheConquerors => "The Conquerors",
			ExpansionPack::TheForgotten => "The Forgotten",
			ExpansionPack::TheAfricanKingdoms => "The African Kingdoms",
			ExpansionPack::RiseOfTheRajas => "Rise of the Rajas",
			ExpansionPack::TheLastKhans => "The Last Khans",
		}
	}
}

impl Display for ExpansionPack
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", self.name())
	}
}
