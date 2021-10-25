use actix_web::{get, post, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};

use crate::part2::{
    errors::*,
    request::{RawRequest, Request},
    request_handler::HandleRequest,
    state::ServerState,
    status_service::{GetStatus, RequestStatus},
};

// TYPES ---------------------------------------------------------------

#[derive(Deserialize)]
pub struct GetStatusQuery {
    id: String,
}

#[derive(Serialize)]
struct StatusResponse {
    id: String,
    airline: String,
    origin: String,
    destiny: String,
    package: bool,
    status: String,
}

// GET INDEX ------------------------------------------------------------------

#[get("/")]
pub async fn get_index() -> impl Responder {
    HttpResponse::Ok()
}

// GET METRICS ----------------------------------------------------------------

#[get("/metrics")]
pub async fn get_metrics() -> impl Responder {
    HttpResponse::Ok().body("Metrics page")
}

// POST REQUEST ---------------------------------------------------------------

#[post("/request")]
pub async fn post_request(
    raw_request: web::Json<RawRequest>,
    state: web::Data<ServerState>,
) -> impl Responder {
    let request_handler = &state.request_handler;
    let msg = HandleRequest {
        raw_request: raw_request.clone(),
    };

    match request_handler.send(msg).await {
        Ok(Ok(req_id)) => HttpResponse::Created().body(req_id),
        Ok(Err(HandlerError::AirlineNotFound)) => {
            HttpResponse::NotFound().body(format!("Airline {} not found", raw_request.airline))
        }
        Ok(Err(HandlerError::AirlineUnavailable)) => HttpResponse::NotFound().body(format!(
            "Airline {} not available, try later",
            raw_request.airline
        )),
        Ok(Err(HandlerError::HotelUnavailable)) => {
            HttpResponse::NotFound().body("Hotel not available, try later")
        }
        Ok(Err(HandlerError::StatusServiceUnavailable)) => HttpResponse::InternalServerError()
            .body("Internal Server Error: Status Service Unavailable"),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
        }
    }
}

// GET REQUEST ----------------------------------------------------------------

#[get("/request")]
pub async fn get_request(
    query: web::Query<GetStatusQuery>,
    state: web::Data<ServerState>,
) -> impl Responder {
    match state
        .status_service
        .send(GetStatus {
            req_id: (*query.id).to_string(),
        })
        .await
    {
        Ok(Ok(RequestStatus {
            req:
                Request {
                    id,
                    raw_request:
                        RawRequest {
                            origin,
                            destiny,
                            airline,
                            package,
                        },
                },
            pending_airline,
            pending_hotel,
        })) => {
            let status = if pending_airline || pending_hotel {
                String::from("PENDING")
            } else {
                String::from("COMPLETED")
            };

            let response = StatusResponse {
                id,
                airline,
                origin,
                destiny,
                package,
                status,
            };
            HttpResponse::Ok().json(response)
        }
        Ok(Err(StatusServiceError::RequestNotFound)) => {
            HttpResponse::NotFound().body("Request not found")
        }
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Internal Server Error: {}", err))
        }
    }
}
