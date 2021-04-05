use super::expansion_pack::ExpansionPack;
use super::water_presence::WaterPresence;

#[derive(Clone)]
pub struct MapData
{
	pub name: &'static str,
	pub image: &'static str,
	pub expansion_pack: ExpansionPack,
	pub water_presence: WaterPresence,
}
