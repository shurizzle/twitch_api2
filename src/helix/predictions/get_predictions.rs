//! Get information about all predictions or specific predictions for a Twitch channel. Prediction information is available for 90 days.
//! [`get-predictions`](https://dev.twitch.tv/docs/api/reference#get-predictions)
//!
//! ## Request: [GetPredictionsRequest]
//!
//! To use this endpoint, construct a [`GetPredictionsRequest`] with the [`GetPredictionsRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::predictions::get_predictions;
//! let request = get_predictions::GetPredictionsRequest::builder()
//!     .id(vec!["ed961efd-8a3f-4cf5-a9d0-e616c590cd2a".to_string()])
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [Prediction]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, predictions::get_predictions};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = get_predictions::GetPredictionsRequest::builder()
//!     .id(vec!["ed961efd-8a3f-4cf5-a9d0-e616c590cd2a".to_string()])
//!     .broadcaster_id("1234")
//!     .build();
//! let response: Vec<get_predictions::Prediction> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetPredictionsRequest::parse_response(None, &request.get_uri(), response)`](GetPredictionsRequest::parse_response)

use super::*;
use helix::RequestGet;
pub use types::{PredictionOutcome, PredictionOutcomeId, PredictionStatus};

/// Query Parameters for [Get predictions](super::get_predictions)
///
/// [`get-predictions`](https://dev.twitch.tv/docs/api/reference#get-predictions)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct GetPredictionsRequest {
    /// The broadcaster running Predictions. Provided broadcaster_id must match the user_id in the user OAuth token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
    /// ID of a Prediction. Filters results to one or more specific Predictions.
    /// Not providing one or more IDs will return the full list of Predictions for the authenticated channel.
    ///
    /// Maximum: 100
    #[builder(default, setter(into))]
    pub id: Vec<types::PredictionId>,
    /// Cursor for forward pagination
    #[builder(default, setter(into))]
    pub after: Option<helix::Cursor>,
    /// Maximum number of objects to return. Maximum: 20. Default: 20.
    #[builder(default, setter(into))]
    pub first: Option<usize>,
}

/// Return Values for [Get predictions](super::get_predictions)
///
/// [`get-predictions`](https://dev.twitch.tv/docs/api/reference#get-predictions)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Prediction {
    /// ID of the Prediction.
    pub id: types::PredictionId,
    /// ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// Title for the Prediction.
    pub title: String,
    /// ID of the winning outcome. If the status is ACTIVE, this is set to null.
    pub winning_outcome_id: Option<PredictionOutcomeId>,
    /// Array of possible outcomes for the Prediction.
    pub outcomes: Vec<PredictionOutcome>,
    /// Total duration for the Prediction (in seconds).
    pub prediction_window: i64,
    /// Status of the Prediction.
    pub status: PredictionStatus,
    /// UTC timestamp for the Prediction’s start time.
    pub created_at: types::Timestamp,
    /// UTC timestamp for when the Prediction ended. If the status is ACTIVE, this is set to null.
    pub ended_at: Option<types::Timestamp>,
    /// UTC timestamp for when the Prediction was locked. If the status is not LOCKED, this is set to null.
    pub locked_at: Option<types::Timestamp>,
}

impl Request for GetPredictionsRequest {
    type Response = Vec<Prediction>;

    const PATH: &'static str = "predictions";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadPredictions];
}

impl RequestGet for GetPredictionsRequest {}

impl helix::Paginated for GetPredictionsRequest {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) { self.after = cursor; }
}

#[test]
fn test_request() {
    use helix::*;
    let req = GetPredictionsRequest::builder()
        .broadcaster_id("55696719")
        .id(vec!["d6676d5c-c86e-44d2-bfc4-100fb48f0656".to_string()])
        .build();

    // From twitch docs
    let data = br#"
{
    "data": [
        {
        "id": "d6676d5c-c86e-44d2-bfc4-100fb48f0656",
        "broadcaster_id": "55696719",
        "broadcaster_name": "TwitchDev",
        "broadcaster_login": "twitchdev",
        "title": "Will there be any leaks today?",
        "winning_outcome_id": null,
        "outcomes": [
            {
            "id": "021e9234-5893-49b4-982e-cfe9a0aaddd9",
            "title": "Yes",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "BLUE"
            },
            {
            "id": "ded84c26-13cb-4b48-8cb5-5bae3ec3a66e",
            "title": "No",
            "users": 0,
            "channel_points": 0,
            "top_predictors": null,
            "color": "PINK"
            }
        ],
        "prediction_window": 600,
        "status": "ACTIVE",
        "created_at": "2021-04-28T16:03:06.320848689Z",
        "ended_at": null,
        "locked_at": null
        }
    ],
    "pagination": {}
    }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/predictions?broadcaster_id=55696719&id=d6676d5c-c86e-44d2-bfc4-100fb48f0656"
    );

    dbg!(GetPredictionsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
