use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock, RwLock},
};

use anyhow::{Ok, Result, anyhow};

use crate::types::{AuroraProbability, Coordinates};

static STORE: OnceLock<Store> = OnceLock::new();

/// Stores application data in Memory
pub struct Store {
    probability_data: RwLock<BTreeMap<Coordinates, AuroraProbability>>,
}

impl Store {
    /// Returns a static refrence to the in-memory store.
    /// We only want to have a single thread-safe instance of the in-memory store, hence we use OnceLock
    pub fn get_store() -> &'static Self {
        STORE.get_or_init(|| Self {
            probability_data: RwLock::new(BTreeMap::new()),
        })
    }

    /// Add given probability data to the store.
    /// The data is stored with coordinates as keys, thus old data is returned if the same key is stored again.
    pub fn add_probability_data(
        &self,
        data: AuroraProbability,
    ) -> Result<Option<AuroraProbability>> {
        let mut guard = self
            .probability_data
            .write()
            .map_err(|e| anyhow!("Unable to attain write lock: {}", e))?;
        let result = guard.insert(data.coordinates, data);
        Ok(result)
    }

    /// Removes probability data from the store.
    /// Data is removed using its coordinates. The removed data is returned
    pub fn remove_probability_data_by_coordinates(
        &self,
        coordinates: Coordinates,
    ) -> Result<Option<AuroraProbability>> {
        let mut guard = self
            .probability_data
            .write()
            .map_err(|e| anyhow!("Unable to attain write lock: {}", e))?;
        let result = guard.remove(&coordinates);
        Ok(result)
    }

    /// Returns probability data for a given coordinates
    pub fn get_probability_data_by_coordinates(
        &self,
        coordinates: Coordinates,
    ) -> Result<Option<AuroraProbability>> {
        let guard = self
            .probability_data
            .read()
            .map_err(|e| anyhow!("Unable to attain read lock: {}", e))?;
        let result = guard.get(&coordinates);
        Ok(result.cloned())
    }

    /// Returns all probability data
    pub fn get_all_probability_data(&self) -> Result<Arc<[AuroraProbability]>> {
        let guard = self
            .probability_data
            .read()
            .map_err(|e| anyhow!("Unable to attain read lock: {}", e))?;
        let data = guard.values().cloned().collect::<Vec<_>>();
        Ok(data.into())
    }

    /// Remove all probabiliy data
    pub fn remove_all_probability_data(&self) -> Result<()> {
        let mut guard = self
            .probability_data
            .write()
            .map_err(|e| anyhow!("Unable to attain write lock: {}", e))?;
        guard.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Colour;
    use chrono::Utc;

    #[test]
    fn it_creates_the_store_successfully() -> Result<()> {
        let _store = Store::get_store();
        Ok(())
    }

    #[test]
    fn it_can_store_probability_data() -> Result<()> {
        let store = Store::get_store();
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        let inserted = store.add_probability_data(data.clone())?;
        assert!(inserted.is_none());
        Ok(())
    }

    #[test]
    fn it_can_access_stored_probability_data() -> Result<()> {
        let store = Store::get_store();
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        store.add_probability_data(data.clone())?;
        let found = store.get_probability_data_by_coordinates(Coordinates { lat: 80, long: 90 })?;
        assert_eq!(data, found.unwrap());
        Ok(())
    }

    #[test]
    fn it_can_remove_stored_probability_data() -> Result<()> {
        let store = Store::get_store();
        let data = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        store.add_probability_data(data.clone())?;
        let removed =
            store.remove_probability_data_by_coordinates(Coordinates { lat: 80, long: 90 })?;
        assert_eq!(data, removed.unwrap());
        Ok(())
    }

    #[test]
    fn it_returns_all_stored_probability_data_as_a_list() -> Result<()> {
        let store = Store::get_store();
        let data1 = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        let data2 = AuroraProbability {
            coordinates: Coordinates { lat: 82, long: 100 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        store.add_probability_data(data1.clone())?;
        store.add_probability_data(data2.clone())?;
        let list = store.get_all_probability_data()?;
        assert_eq!(vec![data1, data2], list.to_vec());
        Ok(())
    }

    #[test]
    fn it_can_remove_all_stored_probability_data() -> Result<()> {
        let store = Store::get_store();
        let data1 = AuroraProbability {
            coordinates: Coordinates { lat: 80, long: 90 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        let data2 = AuroraProbability {
            coordinates: Coordinates { lat: 82, long: 100 },
            colour: Colour::Green,
            value: 0,
            date: Utc::now(),
            location_name: None,
        };
        store.add_probability_data(data1.clone())?;
        store.add_probability_data(data2.clone())?;
        let list = store.get_all_probability_data()?;
        assert_eq!(vec![data1, data2], list.to_vec());
        store.remove_all_probability_data()?;
        let list = store.get_all_probability_data()?;
        assert_eq!(0, list.len());
        Ok(())
    }
}
