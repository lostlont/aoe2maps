use
{
	std::
	{
		fmt::{ Display, Error, Formatter },
		result::Result,
		slice::Iter,
	},
	super::MapAttribute,
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum WaterPresence
{
	None,
	Some,
	Islands,
}

impl MapAttribute for WaterPresence
{
	fn values() -> Iter<'static, WaterPresence>
	{
		static VALUES: [WaterPresence; 3] = [
			WaterPresence::None,
			WaterPresence::Some,
			WaterPresence::Islands,
		];
		VALUES.iter()
	}
}

impl Display for WaterPresence
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		let description = match self
		{
			WaterPresence::None => "Nincs víz",
			WaterPresence::Some => "Van valamennyi víz",
			WaterPresence::Islands => "Szigetek",
		};

		write!(formatter, "{}", description)
	}
}
