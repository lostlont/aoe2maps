use
{
	std::collections::HashSet,
	super::map_attribute::MapAttribute,
};

pub struct MapAttributeSet<T>
where
	T: MapAttribute,
{
	values: HashSet<T>,
}

impl<T> MapAttributeSet<T>
where
	T: MapAttribute,
{
	pub fn new() -> Self
	{
		Self
		{
			values: HashSet::new(),
		}
	}

	pub fn contains(&self, attribute: T) -> bool
	{
		self.values.contains(&attribute)
	}

	pub fn matches(&self, attribute: T) -> bool
	{
		self.values.is_empty() ||
		self.values.contains(&attribute)
	}

	pub fn matches_any(&self, attributes: &HashSet<T>) -> bool
	{
		self.values.is_empty() ||
		self.values.iter().any(|v| attributes.contains(v))
	}

	pub fn toggle(&mut self, attribute: T)
	{
		if self.values.contains(&attribute)
		{
			self.values.remove(&attribute);
		}
		else
		{
			self.values.insert(attribute);
		}
	}
}
