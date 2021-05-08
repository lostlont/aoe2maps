// TODO Move to data?
use
{
	std::
	{
		cell::RefCell,
		iter::FromIterator,
		rc::Rc,
	},
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
	allowed_water_presence: Rc<RefCell<MapAttributeSet<WaterPresence>>>,
}

impl Filter
{
	pub fn new() -> Self
	{
		Self
		{
			allowed_water_presence: Rc::new(RefCell::new(MapAttributeSet::from_iter(vec![
				WaterPresence::None,
				WaterPresence::Some,
				WaterPresence::Islands,
			]))),
		}
	}

	pub fn water_presence(&self) -> Rc<RefCell<MapAttributeSet<WaterPresence>>>
	{
		self.allowed_water_presence.clone()
	}
}

pub trait FilterView
{
	fn is_allowed(&self, map_data: &MapData) -> bool;
}

impl FilterView for Filter
{
	fn is_allowed(&self, map_data: &MapData) -> bool
	{
		self.allowed_water_presence.borrow().contains(&map_data.water_presence)
	}
}
