use super::expansion_pack::ExpansionPack;
use super::map_data::MapData;
use super::water_presence::WaterPresence;

pub fn create_maps() -> Vec<MapData>
{
	return vec!
	[
		MapData
		{
			name: "Arabia",
			image: "https://static.wikia.nocookie.net/ageofempires/images/4/44/Imgres-0.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::None,
		},
	];
}
