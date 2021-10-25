pub enum HandlerError {
    AirlineNotFound,
    AirlineUnavailable,
    HotelUnavailable,
    StatusServiceUnavailable,
}

pub enum StatusServiceError {
    RequestNotFound,
}
