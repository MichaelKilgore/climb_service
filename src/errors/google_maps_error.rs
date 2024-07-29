#[derive(Debug, PartialEq)]
pub enum GoogleMapsError {
    ClientInitializationError,
    AddressUnknown,
    UnknownError
}