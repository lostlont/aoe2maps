use std::
{
	cmp::Eq,
	hash::Hash,
	slice::Iter,
};

mod expansion_pack;
mod water_presence;
pub use
{
	expansion_pack::ExpansionPack,
	water_presence::WaterPresence,
};

pub trait MapAttribute : Clone + Eq + Hash + 'static
where
	Self: Sized,
{
	fn values() -> Iter<'static, Self>;
}
