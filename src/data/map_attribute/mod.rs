use std::
{
	cmp::Eq,
	fmt::Display,
	hash::Hash,
	slice::Iter,
};

mod expansion_pack;
mod resource_amount;
mod water_presence;
pub use
{
	expansion_pack::ExpansionPack,
	resource_amount::ResourceAmount,
	water_presence::WaterPresence,
};

pub trait MapAttribute : Clone + Display + Eq + Hash + 'static
where
	Self: Sized,
{
	fn values() -> Iter<'static, Self>;
}
