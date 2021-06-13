use
{
	std::
	{
		fmt::{ Display, Error, Formatter },
		result::Result,
		slice::Iter,
	},
	crate::
	{
		agents::localization::Text,
		data::enum_values::EnumValues,
	},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Language
{
	EnUs,
	HuHu,
}

impl Default for Language
{
	fn default() -> Self
	{
		Language::EnUs
	}
}

impl Display for Language
{
	fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>
	{
		write!(formatter, "{}", match self
		{
			Language::EnUs => "language-en-us",
			Language::HuHu => "language-hu-hu",
		})
	}
}

impl EnumValues for Language
{
	fn values() -> Iter<'static, Self>
	{
		static VALUES: [Language; 2] =
		[
			Language::EnUs,
			Language::HuHu,
		];
		VALUES.iter()
	}
}

impl Into<Text> for Language
{
	fn into(self) -> Text
	{
		Text::new_id(self.to_string())
	}
}
