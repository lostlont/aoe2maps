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
	pub fn contains(&self, attribute: T) -> bool
	{
		self.values.contains(&attribute)
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
