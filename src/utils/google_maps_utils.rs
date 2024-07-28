use async_trait::async_trait;
use crate::model::coordinates::Coordinates;
use crate::errors::google_maps_error::GoogleMapsError;

#[async_trait]
pub trait GoogleMapsUtils: Send + Sync {
    async fn get_coordinates(&self, _address: String) -> Result<Coordinates, GoogleMapsError> {
        Ok(Coordinates { latitude: 0.0, longitude: 0.0 })
    }
}

pub struct GoogleMapsUtilsImpl;

#[async_trait]
impl GoogleMapsUtils for GoogleMapsUtilsImpl {
    async fn get_coordinates(&self, _address: String) -> Result<Coordinates, GoogleMapsError> {
        Ok(Coordinates { latitude: 0.0, longitude: 0.0 })
    }
}