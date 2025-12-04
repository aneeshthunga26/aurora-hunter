use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};
use tokio::sync::RwLock;

use common_macros::hash_map;

use crate::types::{AuroraProbability, Coordinates, Location};

static STORE: OnceLock<Store> = OnceLock::new();

/// Stores application data in Memory
pub struct Store {
    probability_data: RwLock<HashMap<Coordinates, AuroraProbability>>,
    geocoding_data: RwLock<HashMap<Location, Coordinates>>,
}

impl Store {
    /// Returns a static refrence to the in-memory store.
    /// We only want to have a single thread-safe instance of the in-memory store, hence we use OnceLock
    pub async fn get_store() -> &'static Self {
        STORE.get_or_init(|| Self {
            probability_data: RwLock::new(HashMap::new()),
            geocoding_data: RwLock::new(hash_map! {
                Location::from("aberdeen") => Coordinates { lat: 57, long: -2 },
                Location::from("adelaide") => Coordinates { lat: -35, long: 138 },
                Location::from("algiers") => Coordinates { lat: 37, long: 3 },
                Location::from("amsterdam") => Coordinates { lat: 52, long: 5 },
                Location::from("ankara") => Coordinates { lat: 40, long: 33 },
                Location::from("asuncion") => Coordinates { lat: -25, long: -58 },
                Location::from("athens") => Coordinates { lat: 38, long: 24 },
                Location::from("auckland") => Coordinates { lat: -37, long: 175 },
                Location::from("bangkok") => Coordinates { lat: 14, long: 101 },
                Location::from("barcelona") => Coordinates { lat: 41, long: 2 },
                Location::from("beijing") => Coordinates { lat: 40, long: 116 },
                Location::from("belem") => Coordinates { lat: -1, long: -48 },
                Location::from("belfast") => Coordinates { lat: 55, long: -6 },
                Location::from("belgrade") => Coordinates { lat: 45, long: 21 },
                Location::from("berlin") => Coordinates { lat: 53, long: 13 },
                Location::from("birmingham") => Coordinates { lat: 52, long: -2 },
                Location::from("bogota") => Coordinates { lat: 5, long: -74 },
                Location::from("bombay") => Coordinates { lat: 19, long: 73 },
                Location::from("bordeaux") => Coordinates { lat: 45, long: -1 },
                Location::from("bremen") => Coordinates { lat: 53, long: 9 },
                Location::from("brisbane") => Coordinates { lat: -27, long: 153 },
                Location::from("bristol") => Coordinates { lat: 51, long: -3 },
                Location::from("brussels") => Coordinates { lat: 51, long: 4 },
                Location::from("bucharest") => Coordinates { lat: 44, long: 26 },
                Location::from("budapest") => Coordinates { lat: 48, long: 19 },
                Location::from("buenos aires") => Coordinates { lat: -35, long: -58 },
                Location::from("cairo") => Coordinates { lat: 30, long: 31 },
                Location::from("calcutta") => Coordinates { lat: 23, long: 88 },
                Location::from("canton") => Coordinates { lat: 23, long: 113 },
                Location::from("cape town") => Coordinates { lat: -34, long: 18 },
                Location::from("caracas") => Coordinates { lat: 10, long: -67 },
                Location::from("cayenne") => Coordinates { lat: 5, long: -52 },
                Location::from("chihuahua") => Coordinates { lat: 29, long: -106 },
                Location::from("chongqing") => Coordinates { lat: 30, long: 107 },
                Location::from("copenhagen") => Coordinates { lat: 56, long: 13 },
                Location::from("cordoba") => Coordinates { lat: -31, long: -64 },
                Location::from("dakar") => Coordinates { lat: 15, long: -17 },
                Location::from("darwin") => Coordinates { lat: -12, long: 131 },
                Location::from("djibouti") => Coordinates { lat: 12, long: 43 },
                Location::from("dublin") => Coordinates { lat: 53, long: -6 },
                Location::from("durban") => Coordinates { lat: -30, long: 31 },
                Location::from("edinburgh") => Coordinates { lat: 56, long: -3 },
                Location::from("frankfurt") => Coordinates { lat: 50, long: 9 },
                Location::from("georgetown") => Coordinates { lat: 7, long: -58 },
                Location::from("glasgow") => Coordinates { lat: 56, long: -4 },
                Location::from("guatemala city") => Coordinates { lat: 15, long: -91 },
                Location::from("guayaquil") => Coordinates { lat: -2, long: -80 },
                Location::from("hamburg") => Coordinates { lat: 54, long: 10 },
                Location::from("hammerfest") => Coordinates { lat: 71, long: 24 },
                Location::from("havana") => Coordinates { lat: 23, long: -82 },
                Location::from("helsinki") => Coordinates { lat: 60, long: 25 },
                Location::from("hobart") => Coordinates { lat: -43, long: 147 },
                Location::from("hong kong") => Coordinates { lat: 22, long: 114 },
                Location::from("iquique") => Coordinates { lat: -20, long: -70 },
                Location::from("irkutsk") => Coordinates { lat: 53, long: 104 },
                Location::from("jakarta") => Coordinates { lat: -6, long: 107 },
                Location::from("johannesburg") => Coordinates { lat: -26, long: 28 },
                Location::from("kingston") => Coordinates { lat: 18, long: -77 },
                Location::from("kinshasa") => Coordinates { lat: -4, long: 15 },
                Location::from("kuala lumpur") => Coordinates { lat: 3, long: 102 },
                Location::from("la paz") => Coordinates { lat: -16, long: -68 },
                Location::from("leeds") => Coordinates { lat: 54, long: -2 },
                Location::from("lima") => Coordinates { lat: -12, long: -77 },
                Location::from("lisbon") => Coordinates { lat: 39, long: -9 },
                Location::from("liverpool") => Coordinates { lat: 53, long: -3 },
                Location::from("london") => Coordinates { lat: 52, long: 0 },
                Location::from("lyons") => Coordinates { lat: 46, long: 5 },
                Location::from("madrid") => Coordinates { lat: 40, long: -4 },
                Location::from("manchester") => Coordinates { lat: 54, long: -2 },
                Location::from("manila") => Coordinates { lat: 15, long: 121 },
                Location::from("marseilles") => Coordinates { lat: 43, long: 5 },
                Location::from("mazatlan") => Coordinates { lat: 23, long: -106 },
                Location::from("mecca") => Coordinates { lat: 21, long: 40 },
                Location::from("melbourne") => Coordinates { lat: -38, long: 145 },
                Location::from("mexico city") => Coordinates { lat: 19, long: -99 },
                Location::from("milan") => Coordinates { lat: 45, long: 9 },
                Location::from("montevideo") => Coordinates { lat: -35, long: -56 },
                Location::from("moscow") => Coordinates { lat: 56, long: 38 },
                Location::from("munich") => Coordinates { lat: 48, long: 12 },
                Location::from("nagasaki") => Coordinates { lat: 33, long: 130 },
                Location::from("nagoya") => Coordinates { lat: 35, long: 137 },
                Location::from("nairobi") => Coordinates { lat: -1, long: 37 },
                Location::from("nanjing") => Coordinates { lat: 32, long: 119 },
                Location::from("naples") => Coordinates { lat: 41, long: 14 },
                Location::from("new delhi") => Coordinates { lat: 29, long: 77 },
                Location::from("newcastle") => Coordinates { lat: 55, long: -2 },
                Location::from("odessa") => Coordinates { lat: 46, long: 31 },
                Location::from("osaka") => Coordinates { lat: 35, long: 136 },
                Location::from("oslo") => Coordinates { lat: 60, long: 11 },
                Location::from("panama city") => Coordinates { lat: 9, long: -80 },
                Location::from("paramaribo") => Coordinates { lat: 6, long: -55 },
                Location::from("paris") => Coordinates { lat: 49, long: 2 },
                Location::from("perth") => Coordinates { lat: -32, long: 116 },
                Location::from("plymouth") => Coordinates { lat: 50, long: -4 },
                Location::from("port moresby") => Coordinates { lat: -9, long: 147 },
                Location::from("prague") => Coordinates { lat: 50, long: 14 },
                Location::from("rangoon") => Coordinates { lat: 17, long: 96 },
                Location::from("reykjavik") => Coordinates { lat: 64, long: -22 },
                Location::from("rio de janeiro") => Coordinates { lat: -23, long: -43 },
                Location::from("rome") => Coordinates { lat: 42, long: 12 },
                Location::from("salvador") => Coordinates { lat: -13, long: -38 },
                Location::from("santiago") => Coordinates { lat: -33, long: -71 },
                Location::from("st petersburg") => Coordinates { lat: 60, long: 30 },
                Location::from("sao paulo") => Coordinates { lat: -24, long: -47 },
                Location::from("shanghai") => Coordinates { lat: 31, long: 121 },
                Location::from("singapore") => Coordinates { lat: 1, long: 104 },
                Location::from("sofia") => Coordinates { lat: 43, long: 23 },
                Location::from("stockholm") => Coordinates { lat: 59, long: 18 },
                Location::from("sydney") => Coordinates { lat: -34, long: 151 },
                Location::from("tananarive") => Coordinates { lat: -19, long: 48 },
                Location::from("teheran") => Coordinates { lat: 36, long: 52 },
                Location::from("tokyo") => Coordinates { lat: 36, long: 140 },
                Location::from("tripoli") => Coordinates { lat: 33, long: 13 },
                Location::from("venice") => Coordinates { lat: 45, long: 12 },
                Location::from("veracruz") => Coordinates { lat: 19, long: -96 },
                Location::from("vienna") => Coordinates { lat: 48, long: 16 },
                Location::from("vladivostok") => Coordinates { lat: 43, long: 132 },
                Location::from("warsaw") => Coordinates { lat: 52, long: 21 },
                Location::from("wellington") => Coordinates { lat: -41, long: 175 },
                Location::from("zurich") => Coordinates { lat: 47, long: 9 },
            }),
        })
    }

    /// Fetch geo-coding data which we have cached - for now its hardcoded
    pub async fn get_coordinates_by_location(&self, location: &Location) -> Option<Coordinates> {
        let guard = self.geocoding_data.read().await;
        guard.get(location).copied()
    }

    /// Add given coordinates for a location to the store
    /// The data is stored with location as key and old data is returned if already present
    pub async fn add_coordinates_by_location(
        &self,
        location: Location,
        coordinates: Coordinates,
    ) -> Option<Coordinates> {
        let mut guard = self.geocoding_data.write().await;
        guard.insert(location, coordinates)
    }

    /// Remove coordinates for a given location
    /// The removed data is returned
    pub async fn remove_coordinates_by_location(&self, location: &Location) -> Option<Coordinates> {
        let mut guard = self.geocoding_data.write().await;
        guard.remove(location)
    }

    /// Remove all geocoding data
    pub async fn remove_all_geocoding_data(&self) {
        let mut guard = self.geocoding_data.write().await;
        guard.clear();
    }

    /// Add given probability data to the store.
    /// The data is stored with coordinates as key and old data is returned if already present
    pub async fn add_probability_data(&self, data: AuroraProbability) -> Option<AuroraProbability> {
        let mut guard = self.probability_data.write().await;
        guard.insert(data.coordinates, data)
    }

    /// Removes probability data from the store.
    /// Data is removed using its coordinates. The removed data is returned
    pub async fn remove_probability_data_by_coordinates(
        &self,
        coordinates: Coordinates,
    ) -> Option<AuroraProbability> {
        let mut guard = self.probability_data.write().await;
        guard.remove(&coordinates)
    }

    /// Returns probability data for a given coordinates
    pub async fn get_probability_data_by_coordinates(
        &self,
        coordinates: Coordinates,
    ) -> Option<AuroraProbability> {
        let guard = self.probability_data.read().await;
        guard.get(&coordinates).cloned()
    }

    /// Returns all probability data
    pub async fn get_all_probability_data(&self) -> Arc<[AuroraProbability]> {
        let guard = self.probability_data.read().await;
        guard.values().cloned().collect::<Vec<_>>().into()
    }

    /// Remove all probabiliy data
    pub async fn remove_all_probability_data(&self) {
        let mut guard = self.probability_data.write().await;
        guard.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Colour;
    use anyhow::Result;
    use chrono::Utc;

    #[tokio::test]
    async fn it_creates_the_store_successfully() -> Result<()> {
        let _store = Store::get_store().await;
        Ok(())
    }

    #[tokio::test]
    async fn it_can_store_probability_data() -> Result<()> {
        let store = Store::get_store().await;
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        let inserted = store.add_probability_data(data.clone()).await;
        assert!(inserted.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn it_can_access_stored_probability_data() -> Result<()> {
        let store = Store::get_store().await;
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        store.add_probability_data(data.clone()).await;
        let found = store
            .get_probability_data_by_coordinates(Coordinates { lat: 80, long: 90 })
            .await;
        assert_eq!(data, found.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn it_can_remove_stored_probability_data() -> Result<()> {
        let store = Store::get_store().await;
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        store.add_probability_data(data.clone()).await;
        let removed = store
            .remove_probability_data_by_coordinates(Coordinates { lat: 80, long: 90 })
            .await;
        assert_eq!(data, removed.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn it_returns_all_stored_probability_data_as_a_list() -> Result<()> {
        let store = Store::get_store().await;
        let data1 = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        let data2 = AuroraProbability {
            coordinates: Coordinates { lat: 82, long: 100 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        store.add_probability_data(data1.clone()).await;
        store.add_probability_data(data2.clone()).await;
        let list = store.get_all_probability_data().await;
        assert_eq!(vec![data1, data2], list.to_vec());
        Ok(())
    }

    #[tokio::test]
    async fn it_can_remove_all_stored_probability_data() -> Result<()> {
        let store = Store::get_store().await;
        let data1 = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        let data2 = AuroraProbability {
            coordinates: Coordinates { lat: 82, long: 100 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location: None,
        };
        store.add_probability_data(data1.clone()).await;
        store.add_probability_data(data2.clone()).await;
        let list = store.get_all_probability_data().await;
        assert_eq!(vec![data1, data2], list.to_vec());
        store.remove_all_probability_data().await;
        let list = store.get_all_probability_data().await;
        assert_eq!(0, list.len());
        Ok(())
    }
}
