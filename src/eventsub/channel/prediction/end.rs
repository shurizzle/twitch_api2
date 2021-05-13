#![doc(alias = "channel.hype_train.end")]
//! A Prediction ends on the specified channel.

use super::*;
/// [`channel.hype_train.end`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelpredictionend-beta): a Prediction ends on the specified channel.
#[derive(Clone, Debug, typed_builder::TypedBuilder, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPredictionEndV1 {
    /// The broadcaster user ID of the channel for which “prediction end” notifications will be received.
    #[builder(setter(into))]
    pub broadcaster_user_id: types::UserId,
}

impl EventSubscription for ChannelPredictionEndV1 {
    type Payload = ChannelPredictionEndV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPredictionEnd;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ChannelReadPredictions];
    const VERSION: &'static str = "beta";
}

/// [`channel.hype_train.end`](ChannelPredictionEndV1) response payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPredictionEndV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// The time the Channel Points Prediction ended.
    pub ended_at: types::Timestamp,
    /// Channel Points Prediction ID.
    pub id: types::PredictionId,
    /// The time the Channel Points Prediction locked.
    pub locked_at: types::Timestamp,
    /// An array of outcomes for the Channel Points Prediction. Includes top_predictors.
    pub outcomes: Vec<types::PredictionOutcome>,
    /// The time the Channel Points Prediction started.
    pub started_at: types::Timestamp,
    /// The status of the Channel Points Prediction. Valid values are resolved and canceled.
    pub status: types::PredictionStatus,
    /// Title for the Channel Points Prediction.
    pub title: String,
    /// ID of the winning outcome.
    pub winning_outcome_id: types::PredictionOutcomeId,
}

#[test]
fn parse_payload() {
    // FIXME: example has comments and trailing commas
    // FIXME: example shows user_id as an integer, when it's specified as a string. See https://github.com/twitchdev/issues/issues/390
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.prediction.end",
            "version": "beta",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "id": "1243456",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "title": "Aren’t shoes just really hard socks?",
            "winning_outcome_id": "12345",
            "outcomes": [
                {
                    "id": "12345",
                    "title": "Yeah!", 
                    "color": "blue",
                    "users": 2, 
                    "channel_points": 15000, 
                    "top_predictors": [
                        {
                            "user_name": "Cool_User",
                            "user_login": "cool_user",
                            "user_id": "1234",
                            "channel_points_won": 10000,
                            "channel_points_used": 500
                        },
                        {
                            "user_name": "Coolest_User",
                            "user_login": "coolest_user",
                            "user_id": "1236",
                            "channel_points_won": 5000,
                            "channel_points_used": 100
                        }
                    ] 
                },
                {
                    "id": "22435",
                    "title": "No!", 
                    "users": 2, 
                    "channel_points": 200, 
                    "color": "pink",
                    "top_predictors": [
                        {
                            "user_name": "Cooler_User",
                            "user_login": "cooler_user",
                            "user_id": "12345",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        },
                        {
                            "user_name": "Elite_User",
                            "user_login": "elite_user",
                            "user_id": "1337",
                            "channel_points_won": null,
                            "channel_points_used": 100
                        }
                    ]
                }
            ],
            "status": "resolved",
            "started_at": "2020-07-15T17:16:03.17106713Z",
            "locked_at": "2020-07-15T17:16:11.17106713Z",
            "ended_at": "2020-07-15T17:16:11.17106713Z"
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Payload::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
