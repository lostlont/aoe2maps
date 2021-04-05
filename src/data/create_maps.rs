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
		MapData
		{
			name: "Archipelago",
			image: "https://static.wikia.nocookie.net/ageofempires/images/0/04/Imgres-1424918802.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Islands,
		},
		MapData
		{
			name: "Baltic",
			image: "https://static.wikia.nocookie.net/ageofempires/images/c/ca/Unknown.jpeg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Black Forest",
			image: "https://static.wikia.nocookie.net/ageofempires/images/c/ca/Th.jpeg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::None,
		},
		MapData
		{
			name: "Coastal",
			image: "https://static.wikia.nocookie.net/ageofempires/images/d/d1/Imgres-3.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Continental",
			image: "https://static.wikia.nocookie.net/ageofempires/images/8/8a/Continental.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Fortress",
			image: "https://static.wikia.nocookie.net/ageofempires/images/b/bc/Fortress_two-player_match.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Gold Rush",
			image: "https://static.wikia.nocookie.net/ageofempires/images/5/53/Gold_Rush.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::None,
		},
		MapData
		{
			name: "Highland",
			image: "https://static.wikia.nocookie.net/ageofempires/images/9/96/Highland_AoEII.png",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Islands",
			image: "https://static.wikia.nocookie.net/ageofempires/images/2/2f/AoEII_Islands.png",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Islands,
		},
		MapData
		{
			name: "Mediterranean",
			image: "https://static.wikia.nocookie.net/ageofempires/images/a/ab/AoEII_Mediterranean2.png",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Migration",
			image: "https://static.wikia.nocookie.net/ageofempires/images/4/4d/Migration.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Islands,
		},
		MapData
		{
			name: "Rivers",
			image: "https://static.wikia.nocookie.net/ageofempires/images/a/aa/Rivers_minimap.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Some,
		},
		MapData
		{
			name: "Team Islands",
			image: "https://static.wikia.nocookie.net/ageofempires/images/e/ed/Team_Islands.jpg",
			expansion_pack: ExpansionPack::TheAgeOfKings,
			water_presence: WaterPresence::Islands,
		},
	];
}
