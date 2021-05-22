// TODO Move to data?
use
{
	std::iter::FromIterator,
	crate::
	{
		data::
		{
			enum_values::EnumValues,
			filter_method::FilterMethod,
			map_attribute::{ ExpansionPack, MapCategory, ResourceAmount },
			map_attribute_set::MapAttributeSet,
			map_data::MapData,
		},
	},
};

pub struct Filter
{
	filter_method: FilterMethod,
	expansion_pack: ExpansionPack,
	allowed_map_categories: MapAttributeSet<MapCategory>,
	allowed_wood_amount: MapAttributeSet<ResourceAmount>,
	allowed_food_amount: MapAttributeSet<ResourceAmount>,
	allowed_gold_amount: MapAttributeSet<ResourceAmount>,
	allowed_stone_amount: MapAttributeSet<ResourceAmount>,
}

impl Filter
{
	pub fn new() -> Self
	{
		Self
		{
			filter_method: FilterMethod::Mixed,
			expansion_pack: *ExpansionPack::values().last().unwrap(),
			allowed_map_categories: MapAttributeSet::new(),
			allowed_wood_amount: MapAttributeSet::from_iter(ResourceAmount::values().cloned()),
			allowed_food_amount: MapAttributeSet::from_iter(ResourceAmount::values().cloned()),
			allowed_gold_amount: MapAttributeSet::from_iter(ResourceAmount::values().cloned()),
			allowed_stone_amount: MapAttributeSet::from_iter(ResourceAmount::values().cloned()),
		}
	}

	pub fn set_filter_method(&mut self, filter_method: FilterMethod)
	{
		self.filter_method = filter_method;
	}

	pub fn set_expansion_pack(&mut self, expansion_pack: ExpansionPack)
	{
		self.expansion_pack = expansion_pack;
	}

	pub fn is_allowed_map_category(&self, map_category: MapCategory) -> bool
	{
		self.allowed_map_categories.contains(map_category)
	}

	pub fn toggle_allowed_map_category(&mut self, map_category: MapCategory)
	{
		self.allowed_map_categories.toggle(map_category);
	}

	pub fn is_allowed_wood_amount(&self, wood_amount: ResourceAmount) -> bool
	{
		self.allowed_wood_amount.contains(wood_amount)
	}

	pub fn toggle_allowed_wood_amount(&mut self, wood_amount: ResourceAmount)
	{
		self.allowed_wood_amount.toggle(wood_amount);
	}

	pub fn is_allowed_food_amount(&self, food_amount: ResourceAmount) -> bool
	{
		self.allowed_food_amount.contains(food_amount)
	}

	pub fn toggle_allowed_food_amount(&mut self, food_amount: ResourceAmount)
	{
		self.allowed_food_amount.toggle(food_amount);
	}

	pub fn is_allowed_gold_amount(&self, gold_amount: ResourceAmount) -> bool
	{
		self.allowed_gold_amount.contains(gold_amount)
	}

	pub fn toggle_allowed_gold_amount(&mut self, gold_amount: ResourceAmount)
	{
		self.allowed_gold_amount.toggle(gold_amount);
	}

	pub fn is_allowed_stone_amount(&self, stone_amount: ResourceAmount) -> bool
	{
		self.allowed_stone_amount.contains(stone_amount)
	}

	pub fn toggle_allowed_stone_amount(&mut self, stone_amount: ResourceAmount)
	{
		self.allowed_stone_amount.toggle(stone_amount);
	}
}

pub trait FilterView
{
	fn get_filter_method(&self) -> FilterMethod;
	fn get_expansion_pack(&self) -> ExpansionPack;
	fn is_allowed(&self, map_data: &MapData) -> bool;
	fn is_allowed_by_expansion_pack(&self, map_data: &MapData) -> bool;
	fn is_allowed_by_others(&self, map_data: &MapData) -> bool;
}

impl FilterView for Filter
{
	fn get_filter_method(&self) -> FilterMethod
	{
		self.filter_method
	}

	fn get_expansion_pack(&self) -> ExpansionPack
	{
		self.expansion_pack
	}

	fn is_allowed(&self, map_data: &MapData) -> bool
	{
		self.is_allowed_by_expansion_pack(map_data) &&
		self.is_allowed_by_others(map_data)
	}

	fn is_allowed_by_expansion_pack(&self, map_data: &MapData) -> bool
	{
		map_data.expansion_pack() <= self.expansion_pack
	}

	fn is_allowed_by_others(&self, map_data: &MapData) -> bool
	{
		self.allowed_map_categories.matches(map_data.map_categories()) &&
		self.allowed_wood_amount.contains(map_data.wood_amount()) &&
		self.allowed_food_amount.contains(map_data.food_amount()) &&
		self.allowed_gold_amount.contains(map_data.gold_amount()) &&
		self.allowed_stone_amount.contains(map_data.stone_amount())
	}
}
