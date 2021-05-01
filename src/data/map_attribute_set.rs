use
{
	std::
	{
		collections::HashSet,
		iter::FromIterator,
	},
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
	pub fn contains(&self, value: &T) -> bool
	{
		self.values.contains(value)
	}

	pub fn toggle(&mut self, value: T) -> bool
	{
		if self.values.contains(&value)
		{
			self.values.remove(&value);
			false
		}
		else
		{
			self.values.insert(value);
			true
		}
	}
}

impl<T> FromIterator<T> for MapAttributeSet<T>
where
	T: MapAttribute,
{
	fn from_iter<TIter>(into_iterator: TIter) -> Self
	where
		TIter: IntoIterator<Item = T>,
	{
		Self
		{
			values: HashSet::from_iter(into_iterator),
		}
	}
}
