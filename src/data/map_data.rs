use super::map_attribute::{ ExpansionPack, ResourceAmount, WaterPresence };

#[derive(Clone)]
pub struct MapData
{
	name: &'static str,
	image: &'static str,
	expansion_pack: ExpansionPack,
	water_presence: WaterPresence,
	wood_amount: ResourceAmount,
	food_amount: ResourceAmount,
	gold_amount: ResourceAmount,
	stone_amount: ResourceAmount,
}

impl MapData
{
	pub fn new(
		name: &'static str,
		image: &'static str,
		expansion_pack: ExpansionPack,
		water_presence: WaterPresence,
		wood_amount: ResourceAmount,
		food_amount: ResourceAmount,
		gold_amount: ResourceAmount,
		stone_amount: ResourceAmount) -> Self
	{
		Self
		{
			name,
			image,
			expansion_pack,
			water_presence,
			wood_amount,
			food_amount,
			gold_amount,
			stone_amount,
		}
	}

	pub fn name(&self) -> &'static str
	{
		self.name
	}

	pub fn image(&self) -> &'static str
	{
		self.image
	}

	pub fn expansion_pack(&self) -> ExpansionPack
	{
		self.expansion_pack
	}

	pub fn water_presence(&self) -> &WaterPresence
	{
		&self.water_presence
	}

	pub fn wood_amount(&self) -> &ResourceAmount
	{
		&self.wood_amount
	}

	pub fn food_amount(&self) -> &ResourceAmount
	{
		&self.food_amount
	}

	pub fn gold_amount(&self) -> &ResourceAmount
	{
		&self.gold_amount
	}

	pub fn stone_amount(&self) -> &ResourceAmount
	{
		&self.stone_amount
	}
}
