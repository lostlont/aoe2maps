use crate::data::expansion_pack::ExpansionPack;
use crate::data::map_data::MapData;

pub fn create_maps() -> Vec<MapData>
{
	return vec!
	[
		MapData
		{
			name: "Arabia",
			image: "https://static.wikia.nocookie.net/ageofempires/images/4/44/Imgres-0.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
		},
	];
}
