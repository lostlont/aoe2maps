use crate::expansion_pack::ExpansionPack;
use crate::map_data::MapData;

pub fn create_maps() -> Vec<MapData>
{
	return vec!
	[
		MapData
		{
			name: String::from("Arabia"),
			expansion_pack: ExpansionPack::TheAgeOfKings,
		},
	];
}
