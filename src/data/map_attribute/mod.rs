use std::
{
	cmp::Eq,
	hash::Hash,
};

mod expansion_pack;
mod water_presence;
pub use
{
	expansion_pack::ExpansionPack,
	water_presence::WaterPresence,
};

pub trait MapAttribute : Eq + Hash
{
}
