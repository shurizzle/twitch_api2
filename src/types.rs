//! Twitch types
//!

use beef::Cow;

use serde::{Deserialize, Serialize};

/// A user ID.
pub type UserId<'a> = Cow<'a,str>;

/// A reward ID.
pub type RewardId<'a> = Cow<'a,str>;

/// A reward redemption ID.
pub type RedemptionId<'a> = Cow<'a,str>;

/// A username, also specified as login. Should not be capitalized.
pub type UserName<'a> = Nickname<'a>;

/// A users display name
pub type DisplayName<'a> = Cow<'a,str>;

/// A nickname, not capitalized.
pub type Nickname<'a> = Cow<'a,str>;

/// RFC3339 timestamp
pub type Timestamp<'a> = Cow<'a,str>;

/// A game or category ID
pub type CategoryId<'a> = Cow<'a,str>;

/// A tag ID
pub type TagId<'a> = Cow<'a,str>;

/// A video ID
pub type VideoId<'a> = Cow<'a,str>;

/// An EventSub Subscription ID
pub type EventSubId<'a> = Cow<'a,str>;

/// A Team ID
pub type TeamId<'a> = Cow<'a,str>;

/// A Stream ID
pub type StreamId<'a> = Cow<'a,str>;

/// A game or category as defined by Twitch
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TwitchCategory<'a> {
    ///Template URL for the game’s box art.
    pub box_art_url: Cow<'a, str>,
    /// Game or category ID.
    pub id: CategoryId<'a>,
    ///Game name.
    pub name: Cow<'a,str>,
}

/// Subscription tiers
#[derive(PartialEq, Eq, Deserialize, Clone, Debug)]
#[serde(field_identifier)]
pub enum SubscriptionTier {
    /// Tier 1. $4.99
    #[serde(rename = "1000")]
    Tier1,
    /// Tier 1. $9.99
    #[serde(rename = "2000")]
    Tier2,
    /// Tier 1. $24.99
    #[serde(rename = "3000")]
    Tier3,
    /// Prime subscription
    Prime,
    /// Other
    Other(String),
}

impl Serialize for SubscriptionTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            SubscriptionTier::Tier1 => "1000",
            SubscriptionTier::Tier2 => "2000",
            SubscriptionTier::Tier3 => "3000",
            SubscriptionTier::Prime => "Prime",
            SubscriptionTier::Other(o) => o,
        })
    }
}

/// Broadcaster types: "partner", "affiliated", or "".
#[derive(PartialEq, Deserialize, Clone, Debug)]
pub enum BroadcasterType {
    /// Partner
    #[serde(rename = "partner")]
    Partner,
    /// Affiliated
    #[serde(rename = "affiliated")]
    Affiliated,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for BroadcasterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            BroadcasterType::Partner => "partner",
            BroadcasterType::Affiliated => "affiliated",
            BroadcasterType::None => "",
        })
    }
}

/// User types: "staff", "admin", "global_mod", or "".
#[derive(PartialEq, Deserialize, Clone, Debug)]
pub enum UserType {
    /// Staff
    #[serde(rename = "staff")]
    Staff,
    /// Admin
    #[serde(rename = "admin")]
    Admin,
    /// Global Moderator
    #[serde(rename = "global_mod")]
    GlobalMod,
    /// None
    #[serde(other)]
    None,
}

impl Serialize for UserType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(match self {
            UserType::Staff => "staff",
            UserType::Admin => "admin",
            UserType::GlobalMod => "global_mod",
            UserType::None => "",
        })
    }
}

/// Period during which the video was created
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPeriod {
    /// Filter by all. Effectively a no-op
    All,
    /// Filter by from this day only
    Day,
    /// Filter by this week
    Week,
    /// Filter by this month
    Month,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoType {
    /// A live video
    Live,
    // FIXME: What is this?
    /// A playlist video
    Playlist,
    /// A uploaded video
    Upload,
    /// An archived video
    Archive,
    /// A highlight
    Highlight,
    /// A premiere
    Premiere,
    /// A rerun
    Rerun,
    /// A watch party
    WatchParty,
    /// A watchparty premiere,
    WatchPartyPremiere,
    /// A watchparty rerun
    WatchPartyRerun,
}

/// Type of video
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoPrivacy {
    /// Video is public
    Public,
    /// Video is private
    Private,
}

/// Length of the commercial in seconds
#[derive(
    displaydoc::Display,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[repr(u64)]
#[non_exhaustive]
pub enum CommercialLength {
    /// 30s
    Length30 = 30,
    /// 60s
    Length60 = 60,
    /// 90s
    Length90 = 90,
    /// 120s
    Length120 = 120,
    /// 150s
    Length150 = 150,
    /// 180s
    Length180 = 180,
}

impl std::convert::TryFrom<u64> for CommercialLength {
    type Error = CommercialLengthParseError;

    fn try_from(l: u64) -> Result<Self, Self::Error> {
        match l {
            30 => Ok(CommercialLength::Length30),
            60 => Ok(CommercialLength::Length60),
            90 => Ok(CommercialLength::Length90),
            120 => Ok(CommercialLength::Length120),
            150 => Ok(CommercialLength::Length150),
            180 => Ok(CommercialLength::Length180),
            other => Err(CommercialLengthParseError::InvalidLength(other)),
        }
    }
}

/// Error for the `TryFrom` on [`CommercialLength`]
#[derive(thiserror::Error, Debug, displaydoc::Display)]
pub enum CommercialLengthParseError {
    /// invalid length of {0}
    InvalidLength(u64),
}

/// A user according to many endpoints
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User<'a> {
    /// ID of the user
    #[serde(alias = "user_id")]
    pub id: UserId<'a>,
    /// Login name of the user, not capitalized
    #[serde(alias = "user_login")]
    pub login: UserName<'a>,
    /// Display name of user
    #[serde(alias = "user_display_name", alias = "user_name")]
    pub display_name: DisplayName<'a>,
}

/// Links to the same image of different sizes
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct Image {
    /// URL to png of size 28x28
    pub url_1x: String,
    /// URL to png of size 56x56
    pub url_2x: String,
    /// URL to png of size 112x112
    pub url_4x: String,
}

/// Information about global cooldown
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct GlobalCooldown {
    /// Cooldown enabled
    pub is_enabled: bool,
    /// Cooldown amount
    #[serde(alias = "seconds")]
    pub global_cooldown_seconds: u32,
}

/// Reward redemption max
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[serde(untagged)]
#[non_exhaustive]
pub enum Max {
    /// Max per stream
    MaxPerStream {
        /// Max per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per stream
        #[serde(alias = "value")]
        max_per_stream: u32,
    },
    /// Max per user per stream
    MaxPerUserPerStream {
        /// Max per user per stream is enabled
        is_enabled: bool,
        /// Max amount of redemptions per user per stream
        #[serde(alias = "value")]
        max_per_user_per_stream: u32,
    },
}
