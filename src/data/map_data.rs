use crate::data::expansion_pack::ExpansionPack;

#[derive(Clone)]
pub struct MapData
{
	pub name: &'static str,
	pub image: &'static str,
	pub expansion_pack: ExpansionPack,
}
