use
{
	std::
	{
		collections::HashSet,
		iter::FromIterator,
	},
	super::map_attribute::{ ExpansionPack, MapCategory, ResourceAmount },
};

#[derive(Clone)]
pub struct MapData
{
	name: &'static str,
	image: String,
	expansion_pack: ExpansionPack,
	map_categories: HashSet<MapCategory>,
	wood_amount: ResourceAmount,
	food_amount: ResourceAmount,
	gold_amount: ResourceAmount,
	stone_amount: ResourceAmount,
}

impl MapData
{
	pub fn new(
		name: &'static str,
		expansion_pack: ExpansionPack,
		map_categories: impl IntoIterator<Item = MapCategory>,
		wood_amount: ResourceAmount,
		food_amount: ResourceAmount,
		gold_amount: ResourceAmount,
		stone_amount: ResourceAmount) -> Self
	{
		Self
		{
			name,
			image: format!("maps/{}.png", name),
			expansion_pack,
			map_categories: HashSet::from_iter(map_categories),
			wood_amount,
			food_amount,
			gold_amount,
			stone_amount,
		}
	}

	pub fn name(&self) -> &str
	{
		self.name
	}

	pub fn image(&self) -> &str
	{
		&self.image
	}

	pub fn expansion_pack(&self) -> ExpansionPack
	{
		self.expansion_pack
	}

	pub fn map_categories(&self) -> &HashSet<MapCategory>
	{
		&self.map_categories
	}

	pub fn wood_amount(&self) -> ResourceAmount
	{
		self.wood_amount
	}

	pub fn food_amount(&self) -> ResourceAmount
	{
		self.food_amount
	}

	pub fn gold_amount(&self) -> ResourceAmount
	{
		self.gold_amount
	}

	pub fn stone_amount(&self) -> ResourceAmount
	{
		self.stone_amount
	}
}
