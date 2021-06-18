use super::
{
	map_attribute::{ ExpansionPack, MapCategory, ResourceAmount },
	map_data::MapData,
};

pub fn create_maps() -> Vec<MapData>
{
	return vec!
	[
		MapData::new(
			"Arabia",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Scarce,
			ResourceAmount::Scarce,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Archipelago",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::WaterMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Baltic",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::WaterMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Black Forest",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::LandMap,
				MapCategory::ClosedMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Coastal",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Continental",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
		),
		MapData::new(
			"Crater Lake",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::WaterMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
		),
		MapData::new(
			"Fortress",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::LandMap,
				MapCategory::ClosedMap,
			],
			ResourceAmount::Scarce,
			ResourceAmount::Scarce,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Gold Rush",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
		),
		MapData::new(
			"Highland",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Islands",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::WaterMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Mediterranean",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Migration",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::WaterMap,
				MapCategory::MigrationMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Scarce,
			ResourceAmount::Scarce,
		),
		MapData::new(
			"Rivers",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Team Islands",
			ExpansionPack::TheAgeOfKings,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::WaterMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Arena",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::LandMap,
				MapCategory::ClosedMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Ghost Lake",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Mongolia",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::LandMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Nomad",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::MixedMap,
				MapCategory::NomadMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Oasis",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::LandMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Salt Marsh",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Scandinavia",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::FullRandom,
				MapCategory::MixedMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Scarce,
		),
		MapData::new(
			"Yucatan",
			ExpansionPack::TheConquerors,
			vec![
				MapCategory::BlindRandom,
				MapCategory::RandomLandMap,
				MapCategory::FullRandom,
				MapCategory::LandMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Acropolis",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Budapest",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::MixedMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Cenotes",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Scarce,
			ResourceAmount::Average,
			ResourceAmount::Scarce,
			ResourceAmount::Scarce,
		),
		MapData::new(
			"City of Lakes",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::MixedMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Golden Pit",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
		),
		MapData::new(
			"Hamburger",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::MixedMap,
				MapCategory::MigrationMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Hideout",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Hill Fort",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Lombardia",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Steppe",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::NomadMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Plentiful,
			ResourceAmount::Plentiful,
			ResourceAmount::Average,
			ResourceAmount::Average,
		),
		MapData::new(
			"Valley",
			ExpansionPack::TheForgotten,
			vec![
				MapCategory::LandMap,
				MapCategory::OpenMap,
			],
			ResourceAmount::Average,
			ResourceAmount::Average,
			ResourceAmount::Scarce,
			ResourceAmount::Scarce,
		),
	];
}
