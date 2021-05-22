use
{
	std::
	{
		cmp::Eq,
		fmt::Display,
		hash::Hash,
	},
	super::enum_values::EnumValues,
};

mod expansion_pack;
mod map_category;
mod resource_amount;
pub use
{
	expansion_pack::ExpansionPack,
	map_category::MapCategory,
	resource_amount::ResourceAmount,
};

pub trait MapAttribute : Clone + Copy + Display + EnumValues + Eq + Hash + 'static
where
	Self: Sized,
{
}
