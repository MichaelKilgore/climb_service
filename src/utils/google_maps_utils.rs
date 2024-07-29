use std::env;
use async_trait::async_trait;
use google_maps::GoogleMapsClient;
use rust_decimal::prelude::ToPrimitive;
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
    async fn get_coordinates(&self, address: String) -> Result<Coordinates, GoogleMapsError> {
        let google_maps_client = match GoogleMapsClient::try_new(env::var("GOOGLE_MAPS_API_KEY").unwrap()) {
            Ok(client) => client,
            Err(err) => {
                eprintln!("{}", format!("Error occurred while initializing google maps api key: {err}"));
                return Err(GoogleMapsError::ClientInitializationError);
            }
        };

        let location = match google_maps_client.geocoding()
            .with_address(address)
            .execute()
            .await {
            Ok(resp) => resp,
            Err(err) => {
                if err.to_string().eq("Google Maps Geocoding API server: Zero results. This may occur if the geocoder was passed a non-existent address.") {
                    return Err(GoogleMapsError::AddressUnknown);
                }
                eprintln!("{}", format!("Error occurred while retrieving location for address: {err}"));
                return Err(GoogleMapsError::UnknownError);
            }
        };
        
        if location.results.len() == 0 {
            return Err(GoogleMapsError::AddressUnknown);
        }
        
        Ok(Coordinates { latitude: location.results[0].geometry.location.latitude().to_f64().unwrap(), 
            longitude: location.results[0].geometry.location.longitude().to_f64().unwrap() })
    }
}