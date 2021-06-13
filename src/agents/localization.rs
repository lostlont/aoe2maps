use
{
	std::
	{
		collections::HashMap,
		sync::Mutex,
	},
	fluent_templates::{
		fluent_bundle::FluentValue,
		LanguageIdentifier,
		Loader,
		static_loader
	},
	i18n_embed::WebLanguageRequester,
	lazy_static::lazy_static,
	unic_langid::langid,
	crate::data::language::Language,
};

static_loader!
{
	static LOCALES =
	{
		locales: "./static/locales",
		fallback_language: "en-US",
	};
}

static EN_US: LanguageIdentifier = langid!("en-US");
static HU_HU: LanguageIdentifier = langid!("hu-HU");

lazy_static!
{
	static ref LANGUAGE: Mutex<LanguageIdentifier> = Mutex::new(detect_language());
}

// TODO Split?
#[derive(Clone)]
pub struct Text
{
	value: String,
	args: Option<HashMap<&'static str, Text>>,
	localize: bool,
}

impl Text
{
	pub fn new_id(id: impl ToString) -> Self
	{
		Self
		{
			value: id.to_string(),
			args: None,
			localize: true,
		}
	}

	pub fn new_id_args(id: impl ToString, args: HashMap<&'static str, Text>) -> Self
	{
		Self
		{
			value: id.to_string(),
			args: Some(args),
			localize: true,
		}
	}

	pub fn new_value(value: String) -> Self
	{
		Self
		{
			value,
			args: None,
			localize: false,
		}
	}

	pub fn localize(&self) -> String
	{
		if self.localize
		{
			if let Some(args) = &self.args
			{
				let args = args
					.iter()
					.map(|(k, v)| (*k, v.localize().into()))
					.collect();
				localize_with(&self.value, &args)
			}
			else
			{
				localize(&self.value)
			}
		}
		else
		{
			self.value.clone()
		}
	}
}

impl Default for Text
{
	fn default() -> Self
	{
		Text::new_value("".to_string())
	}
}

fn detect_language() -> LanguageIdentifier
{
	let supported_languages = vec![ &EN_US, &HU_HU ];
	let requested_languages = WebLanguageRequester::requested_languages();
	requested_languages
		.iter()
		.filter(|l| supported_languages.contains(l))
		.cloned()
		.next()
		.unwrap_or_else(|| EN_US.clone())
}

pub fn set_language(language: Language)
{
	*LANGUAGE.lock().unwrap() = match language
	{
		Language::EnUs => EN_US.clone(),
		Language::HuHu => HU_HU.clone(),
	};
}

pub fn get_language() -> Language
{
	let language = &*LANGUAGE.lock().unwrap();
	if language == &EN_US
	{
		Language::EnUs
	}
	else if language == &HU_HU
	{
		Language::HuHu
	}
	else
	{
		Language::EnUs
	}
}

pub fn localize(id: &str) -> String
{
	LOCALES.lookup(&*LANGUAGE.lock().unwrap(), id)
}

pub fn localize_with(id: &str, args: &std::collections::HashMap<impl AsRef<str>, FluentValue>) -> String
{
	LOCALES.lookup_with_args(&*LANGUAGE.lock().unwrap(), id, args)
}
