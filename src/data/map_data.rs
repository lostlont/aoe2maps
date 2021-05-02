use super::map_attribute::{ ExpansionPack, WaterPresence };

#[derive(Clone)]
pub struct MapData
{
	pub name: &'static str,
	pub image: &'static str,
	pub expansion_pack: ExpansionPack,
	pub water_presence: WaterPresence,
}
