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
			map_attribute::{ ExpansionPack, MapAttribute, ResourceAmount, WaterPresence },
			map_attribute_set::MapAttributeSet,
			map_data::MapData,
		},
	},
};

type SharedSet<T> = Rc<RefCell<MapAttributeSet<T>>>;

pub struct Filter
{
	allowed_expansion_packs: SharedSet<ExpansionPack>,
	allowed_water_presence: SharedSet<WaterPresence>,
	allowed_wood_amount: SharedSet<ResourceAmount>,
	allowed_food_amount: SharedSet<ResourceAmount>,
	allowed_gold_amount: SharedSet<ResourceAmount>,
	allowed_stone_amount: SharedSet<ResourceAmount>,
}

impl Filter
{
	pub fn new() -> Self
	{
		fn from<T>() -> SharedSet<T>
		where
			T: MapAttribute,
		{
			Rc::new(RefCell::new(MapAttributeSet::from_iter(T::values().cloned())))
		}

		Self
		{
			allowed_expansion_packs: from::<ExpansionPack>(),
			allowed_water_presence: from::<WaterPresence>(),
			allowed_wood_amount: from::<ResourceAmount>(),
			allowed_food_amount: from::<ResourceAmount>(),
			allowed_gold_amount: from::<ResourceAmount>(),
			allowed_stone_amount: from::<ResourceAmount>(),
		}
	}

	pub fn expansion_pack(&self) -> SharedSet<ExpansionPack>
	{
		self.allowed_expansion_packs.clone()
	}

	pub fn water_presence(&self) -> SharedSet<WaterPresence>
	{
		self.allowed_water_presence.clone()
	}

	pub fn wood_amount(&self) -> SharedSet<ResourceAmount>
	{
		self.allowed_wood_amount.clone()
	}

	pub fn food_amount(&self) -> SharedSet<ResourceAmount>
	{
		self.allowed_food_amount.clone()
	}

	pub fn gold_amount(&self) -> SharedSet<ResourceAmount>
	{
		self.allowed_gold_amount.clone()
	}

	pub fn stone_amount(&self) -> SharedSet<ResourceAmount>
	{
		self.allowed_stone_amount.clone()
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
		self.allowed_expansion_packs.borrow().contains(map_data.expansion_pack()) &&
		self.allowed_water_presence.borrow().contains(map_data.water_presence()) &&
		self.allowed_wood_amount.borrow().contains(map_data.wood_amount()) &&
		self.allowed_food_amount.borrow().contains(map_data.food_amount()) &&
		self.allowed_gold_amount.borrow().contains(map_data.gold_amount()) &&
		self.allowed_stone_amount.borrow().contains(map_data.stone_amount())
	}
}
