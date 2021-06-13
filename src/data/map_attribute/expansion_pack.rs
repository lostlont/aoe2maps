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

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
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

impl EnumValues for ExpansionPack
{
	fn values() -> Iter<'static, ExpansionPack>
	{
		static VALUES: [ExpansionPack; 6] = [
			ExpansionPack::TheAgeOfKings,
			ExpansionPack::TheConquerors,
			ExpansionPack::TheForgotten,
			ExpansionPack::TheAfricanKingdoms,
			ExpansionPack::RiseOfTheRajas,
			ExpansionPack::TheLastKhans,
		];
		VALUES.iter()
	}
}

impl Display for ExpansionPack
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", self.name())
	}
}

impl Into<Text> for ExpansionPack
{
	fn into(self) -> Text
	{
		Text::new_value(self.to_string())
	}
}

impl MapAttribute for ExpansionPack
{
}
