// TODO Move to data?
use
{
	std::iter::FromIterator,
	crate::
	{
		data::
		{
			map_attribute::WaterPresence,
			map_attribute_set::MapAttributeSet,
			map_data::MapData,
		},
	},
};

pub struct Filter
{
	allowed_water_presence: MapAttributeSet<WaterPresence>,
}

impl Filter
{
	pub fn new() -> Self
	{
		Self
		{
			allowed_water_presence: MapAttributeSet::from_iter(vec![
				WaterPresence::None,
				WaterPresence::Some,
				WaterPresence::Islands,
			]),
		}
	}

	pub fn toggle(&mut self, water_presence: WaterPresence) -> bool
	{
		self.allowed_water_presence.toggle(water_presence)
	}
}

pub trait FilterView
{
	fn allowed_water_presence(&self) -> &MapAttributeSet<WaterPresence>;
	fn is_allowed(&self, map_data: &MapData) -> bool;
}

impl FilterView for Filter
{
	fn allowed_water_presence(&self) -> &MapAttributeSet<WaterPresence>
	{
		&self.allowed_water_presence
	}

	fn is_allowed(&self, map_data: &MapData) -> bool
	{
		self.allowed_water_presence.contains(&map_data.water_presence)
	}
}
