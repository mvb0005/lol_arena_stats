#![allow(unused_qualifications)]

use http::HeaderValue;
use validator::Validate;

#[cfg(feature = "server")]
use crate::header;
use crate::{models, types::*};

#[allow(dead_code)]
fn from_validation_error(e: validator::ValidationError) -> validator::ValidationErrors {
    let mut errs = validator::ValidationErrors::new();
    errs.add("na", e);
    errs
}

#[allow(dead_code)]
pub fn check_xss_string(v: &str) -> std::result::Result<(), validator::ValidationError> {
    if ammonia::is_html(v) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_vec_string(v: &[String]) -> std::result::Result<(), validator::ValidationError> {
    if v.iter().any(|i| ammonia::is_html(i)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_string(
    v: &std::collections::HashMap<String, String>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| ammonia::is_html(v)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map_nested<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError>
where
    T: validator::Validate,
{
    if v.keys().any(|k| ammonia::is_html(k)) || v.values().any(|v| v.validate().is_err()) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[allow(dead_code)]
pub fn check_xss_map<T>(
    v: &std::collections::HashMap<String, T>,
) -> std::result::Result<(), validator::ValidationError> {
    if v.keys().any(|k| ammonia::is_html(k)) {
        std::result::Result::Err(validator::ValidationError::new("xss detected"))
    } else {
        std::result::Result::Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetArenaStatsQueryParams {
    #[serde(rename = "playerName")]
    pub player_name: String,
    #[serde(rename = "tagLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_line: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct GetPlayerProfilePathParams {
    pub puuid: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SearchPlayersQueryParams {
    #[serde(rename = "game_name")]
    pub game_name: String,
    #[serde(rename = "tagline")]
    pub tagline: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ActiveArenaSummary {
    #[serde(rename = "in_game")]
    pub in_game: bool,

    #[serde(rename = "queue_id")]
    pub queue_id: i32,

    #[serde(rename = "game_start_time")]
    pub game_start_time: i64,
}

impl ActiveArenaSummary {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(in_game: bool, queue_id: i32, game_start_time: i64) -> ActiveArenaSummary {
        ActiveArenaSummary {
            in_game,
            queue_id,
            game_start_time,
        }
    }
}

/// Converts the ActiveArenaSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ActiveArenaSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("in_game".to_string()),
            Some(self.in_game.to_string()),
            Some("queue_id".to_string()),
            Some(self.queue_id.to_string()),
            Some("game_start_time".to_string()),
            Some(self.game_start_time.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ActiveArenaSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ActiveArenaSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub in_game: Vec<bool>,
            pub queue_id: Vec<i32>,
            pub game_start_time: Vec<i64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ActiveArenaSummary".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "in_game" => intermediate_rep.in_game.push(
                        <bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "queue_id" => intermediate_rep.queue_id.push(
                        <i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "game_start_time" => intermediate_rep.game_start_time.push(
                        <i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ActiveArenaSummary".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ActiveArenaSummary {
            in_game: intermediate_rep
                .in_game
                .into_iter()
                .next()
                .ok_or_else(|| "in_game missing in ActiveArenaSummary".to_string())?,
            queue_id: intermediate_rep
                .queue_id
                .into_iter()
                .next()
                .ok_or_else(|| "queue_id missing in ActiveArenaSummary".to_string())?,
            game_start_time: intermediate_rep
                .game_start_time
                .into_iter()
                .next()
                .ok_or_else(|| "game_start_time missing in ActiveArenaSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ActiveArenaSummary> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ActiveArenaSummary>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ActiveArenaSummary>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ActiveArenaSummary - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ActiveArenaSummary> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ActiveArenaSummary as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ActiveArenaSummary - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ArenaPlayerStats {
    #[serde(rename = "averagePlacement")]
    #[validate(range(min = 1f64, max = 8f64))]
    pub average_placement: f64,

    #[serde(rename = "lastUpdated")]
    #[validate(custom(function = "check_xss_string"))]
    pub last_updated: String,

    #[serde(rename = "matchesPlayed")]
    #[validate(range(min = 0u32))]
    pub matches_played: u32,

    #[serde(rename = "playerName")]
    #[validate(custom(function = "check_xss_string"))]
    pub player_name: String,

    #[serde(rename = "region")]
    #[validate(custom(function = "check_xss_string"))]
    pub region: String,

    #[serde(rename = "tagLine")]
    #[validate(custom(function = "check_xss_string"))]
    pub tag_line: String,

    #[serde(rename = "topFourRate")]
    #[validate(range(min = 0f64, max = 1f64))]
    pub top_four_rate: f64,

    #[serde(rename = "winRate")]
    #[validate(range(min = 0f64, max = 1f64))]
    pub win_rate: f64,
}

impl ArenaPlayerStats {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        average_placement: f64,
        last_updated: String,
        matches_played: u32,
        player_name: String,
        region: String,
        tag_line: String,
        top_four_rate: f64,
        win_rate: f64,
    ) -> ArenaPlayerStats {
        ArenaPlayerStats {
            average_placement,
            last_updated,
            matches_played,
            player_name,
            region,
            tag_line,
            top_four_rate,
            win_rate,
        }
    }
}

/// Converts the ArenaPlayerStats value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ArenaPlayerStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("averagePlacement".to_string()),
            Some(self.average_placement.to_string()),
            Some("lastUpdated".to_string()),
            Some(self.last_updated.to_string()),
            Some("matchesPlayed".to_string()),
            Some(self.matches_played.to_string()),
            Some("playerName".to_string()),
            Some(self.player_name.to_string()),
            Some("region".to_string()),
            Some(self.region.to_string()),
            Some("tagLine".to_string()),
            Some(self.tag_line.to_string()),
            Some("topFourRate".to_string()),
            Some(self.top_four_rate.to_string()),
            Some("winRate".to_string()),
            Some(self.win_rate.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ArenaPlayerStats value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ArenaPlayerStats {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub average_placement: Vec<f64>,
            pub last_updated: Vec<String>,
            pub matches_played: Vec<u32>,
            pub player_name: Vec<String>,
            pub region: Vec<String>,
            pub tag_line: Vec<String>,
            pub top_four_rate: Vec<f64>,
            pub win_rate: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ArenaPlayerStats".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "averagePlacement" => intermediate_rep.average_placement.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "lastUpdated" => intermediate_rep.last_updated.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "matchesPlayed" => intermediate_rep.matches_played.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "playerName" => intermediate_rep.player_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "region" => intermediate_rep.region.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tagLine" => intermediate_rep.tag_line.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "topFourRate" => intermediate_rep.top_four_rate.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "winRate" => intermediate_rep.win_rate.push(
                        <f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ArenaPlayerStats".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ArenaPlayerStats {
            average_placement: intermediate_rep
                .average_placement
                .into_iter()
                .next()
                .ok_or_else(|| "averagePlacement missing in ArenaPlayerStats".to_string())?,
            last_updated: intermediate_rep
                .last_updated
                .into_iter()
                .next()
                .ok_or_else(|| "lastUpdated missing in ArenaPlayerStats".to_string())?,
            matches_played: intermediate_rep
                .matches_played
                .into_iter()
                .next()
                .ok_or_else(|| "matchesPlayed missing in ArenaPlayerStats".to_string())?,
            player_name: intermediate_rep
                .player_name
                .into_iter()
                .next()
                .ok_or_else(|| "playerName missing in ArenaPlayerStats".to_string())?,
            region: intermediate_rep
                .region
                .into_iter()
                .next()
                .ok_or_else(|| "region missing in ArenaPlayerStats".to_string())?,
            tag_line: intermediate_rep
                .tag_line
                .into_iter()
                .next()
                .ok_or_else(|| "tagLine missing in ArenaPlayerStats".to_string())?,
            top_four_rate: intermediate_rep
                .top_four_rate
                .into_iter()
                .next()
                .ok_or_else(|| "topFourRate missing in ArenaPlayerStats".to_string())?,
            win_rate: intermediate_rep
                .win_rate
                .into_iter()
                .next()
                .ok_or_else(|| "winRate missing in ArenaPlayerStats".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ArenaPlayerStats> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ArenaPlayerStats>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ArenaPlayerStats>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ArenaPlayerStats - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ArenaPlayerStats> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ArenaPlayerStats as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ArenaPlayerStats - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ArenaRankSummary {
    #[serde(rename = "tier")]
    #[validate(custom(function = "check_xss_string"))]
    pub tier: String,

    #[serde(rename = "rank")]
    #[validate(custom(function = "check_xss_string"))]
    pub rank: String,

    #[serde(rename = "league_points")]
    #[validate(range(min = 0u32))]
    pub league_points: u32,
}

impl ArenaRankSummary {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(tier: String, rank: String, league_points: u32) -> ArenaRankSummary {
        ArenaRankSummary {
            tier,
            rank,
            league_points,
        }
    }
}

/// Converts the ArenaRankSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ArenaRankSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("tier".to_string()),
            Some(self.tier.to_string()),
            Some("rank".to_string()),
            Some(self.rank.to_string()),
            Some("league_points".to_string()),
            Some(self.league_points.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ArenaRankSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ArenaRankSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub tier: Vec<String>,
            pub rank: Vec<String>,
            pub league_points: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ArenaRankSummary".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "tier" => intermediate_rep.tier.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "rank" => intermediate_rep.rank.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "league_points" => intermediate_rep.league_points.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ArenaRankSummary".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ArenaRankSummary {
            tier: intermediate_rep
                .tier
                .into_iter()
                .next()
                .ok_or_else(|| "tier missing in ArenaRankSummary".to_string())?,
            rank: intermediate_rep
                .rank
                .into_iter()
                .next()
                .ok_or_else(|| "rank missing in ArenaRankSummary".to_string())?,
            league_points: intermediate_rep
                .league_points
                .into_iter()
                .next()
                .ok_or_else(|| "league_points missing in ArenaRankSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ArenaRankSummary> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ArenaRankSummary>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ArenaRankSummary>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ArenaRankSummary - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ArenaRankSummary> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ArenaRankSummary as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ArenaRankSummary - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorResponse {
    #[serde(rename = "message")]
    #[validate(custom(function = "check_xss_string"))]
    pub message: String,
}

impl ErrorResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String) -> ErrorResponse {
        ErrorResponse { message }
    }
}

/// Converts the ErrorResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![Some("message".to_string()), Some(self.message.to_string())];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ErrorResponse".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ErrorResponse".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorResponse {
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in ErrorResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorResponse>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ErrorResponse>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ErrorResponse - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ErrorResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ErrorResponse as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ErrorResponse - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ErrorResponse1 {
    #[serde(rename = "message")]
    #[validate(custom(function = "check_xss_string"))]
    pub message: String,
}

impl ErrorResponse1 {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(message: String) -> ErrorResponse1 {
        ErrorResponse1 { message }
    }
}

/// Converts the ErrorResponse1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ErrorResponse1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> =
            vec![Some("message".to_string()), Some(self.message.to_string())];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ErrorResponse1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ErrorResponse1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing ErrorResponse1".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ErrorResponse1".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ErrorResponse1 {
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in ErrorResponse1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ErrorResponse1> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ErrorResponse1>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ErrorResponse1>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ErrorResponse1 - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ErrorResponse1> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ErrorResponse1 as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ErrorResponse1 - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HealthStatus {
    #[serde(rename = "service")]
    #[validate(custom(function = "check_xss_string"))]
    pub service: String,

    #[serde(rename = "status")]
    #[validate(custom(function = "check_xss_string"))]
    pub status: String,

    #[serde(rename = "version")]
    #[validate(custom(function = "check_xss_string"))]
    pub version: String,
}

impl HealthStatus {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(service: String, status: String, version: String) -> HealthStatus {
        HealthStatus {
            service,
            status,
            version,
        }
    }
}

/// Converts the HealthStatus value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("service".to_string()),
            Some(self.service.to_string()),
            Some("status".to_string()),
            Some(self.status.to_string()),
            Some("version".to_string()),
            Some(self.version.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HealthStatus value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HealthStatus {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub service: Vec<String>,
            pub status: Vec<String>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing HealthStatus".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "service" => intermediate_rep.service.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing HealthStatus".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HealthStatus {
            service: intermediate_rep
                .service
                .into_iter()
                .next()
                .ok_or_else(|| "service missing in HealthStatus".to_string())?,
            status: intermediate_rep
                .status
                .into_iter()
                .next()
                .ok_or_else(|| "status missing in HealthStatus".to_string())?,
            version: intermediate_rep
                .version
                .into_iter()
                .next()
                .ok_or_else(|| "version missing in HealthStatus".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HealthStatus> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<HealthStatus>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<HealthStatus>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for HealthStatus - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<HealthStatus> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <HealthStatus as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into HealthStatus - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlayerProfile {
    #[serde(rename = "puuid")]
    #[validate(custom(function = "check_xss_string"))]
    pub puuid: String,

    #[serde(rename = "game_name")]
    #[validate(custom(function = "check_xss_string"))]
    pub game_name: String,

    #[serde(rename = "tagline")]
    #[validate(custom(function = "check_xss_string"))]
    pub tagline: String,

    #[serde(rename = "summoner_id")]
    #[validate(custom(function = "check_xss_string"))]
    pub summoner_id: String,

    #[serde(rename = "summoner_level")]
    #[validate(range(min = 0u32))]
    pub summoner_level: u32,

    #[serde(rename = "profile_icon_id")]
    #[validate(range(min = 0u32))]
    pub profile_icon_id: u32,

    #[serde(rename = "arena_rank")]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arena_rank: Option<models::ArenaRankSummary>,

    #[serde(rename = "active_arena")]
    #[validate(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_arena: Option<models::ActiveArenaSummary>,
}

impl PlayerProfile {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        puuid: String,
        game_name: String,
        tagline: String,
        summoner_id: String,
        summoner_level: u32,
        profile_icon_id: u32,
    ) -> PlayerProfile {
        PlayerProfile {
            puuid,
            game_name,
            tagline,
            summoner_id,
            summoner_level,
            profile_icon_id,
            arena_rank: None,
            active_arena: None,
        }
    }
}

/// Converts the PlayerProfile value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("puuid".to_string()),
            Some(self.puuid.to_string()),
            Some("game_name".to_string()),
            Some(self.game_name.to_string()),
            Some("tagline".to_string()),
            Some(self.tagline.to_string()),
            Some("summoner_id".to_string()),
            Some(self.summoner_id.to_string()),
            Some("summoner_level".to_string()),
            Some(self.summoner_level.to_string()),
            Some("profile_icon_id".to_string()),
            Some(self.profile_icon_id.to_string()),
            // Skipping arena_rank in query parameter serialization

            // Skipping active_arena in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlayerProfile value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlayerProfile {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub puuid: Vec<String>,
            pub game_name: Vec<String>,
            pub tagline: Vec<String>,
            pub summoner_id: Vec<String>,
            pub summoner_level: Vec<u32>,
            pub profile_icon_id: Vec<u32>,
            pub arena_rank: Vec<models::ArenaRankSummary>,
            pub active_arena: Vec<models::ActiveArenaSummary>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlayerProfile".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "puuid" => intermediate_rep.puuid.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "game_name" => intermediate_rep.game_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tagline" => intermediate_rep.tagline.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "summoner_id" => intermediate_rep.summoner_id.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "summoner_level" => intermediate_rep.summoner_level.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "profile_icon_id" => intermediate_rep.profile_icon_id.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "arena_rank" => intermediate_rep.arena_rank.push(
                        <models::ArenaRankSummary as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "active_arena" => intermediate_rep.active_arena.push(
                        <models::ActiveArenaSummary as std::str::FromStr>::from_str(val)
                            .map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlayerProfile".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlayerProfile {
            puuid: intermediate_rep
                .puuid
                .into_iter()
                .next()
                .ok_or_else(|| "puuid missing in PlayerProfile".to_string())?,
            game_name: intermediate_rep
                .game_name
                .into_iter()
                .next()
                .ok_or_else(|| "game_name missing in PlayerProfile".to_string())?,
            tagline: intermediate_rep
                .tagline
                .into_iter()
                .next()
                .ok_or_else(|| "tagline missing in PlayerProfile".to_string())?,
            summoner_id: intermediate_rep
                .summoner_id
                .into_iter()
                .next()
                .ok_or_else(|| "summoner_id missing in PlayerProfile".to_string())?,
            summoner_level: intermediate_rep
                .summoner_level
                .into_iter()
                .next()
                .ok_or_else(|| "summoner_level missing in PlayerProfile".to_string())?,
            profile_icon_id: intermediate_rep
                .profile_icon_id
                .into_iter()
                .next()
                .ok_or_else(|| "profile_icon_id missing in PlayerProfile".to_string())?,
            arena_rank: intermediate_rep.arena_rank.into_iter().next(),
            active_arena: intermediate_rep.active_arena.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlayerProfile> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PlayerProfile>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlayerProfile>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for PlayerProfile - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PlayerProfile> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlayerProfile as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into PlayerProfile - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlayerProfileResponse {
    #[serde(rename = "player")]
    #[validate(nested)]
    pub player: models::PlayerProfile,

    #[serde(rename = "recent_matches")]
    #[validate(nested)]
    pub recent_matches: Vec<models::RecentArenaMatchSummary>,
}

impl PlayerProfileResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        player: models::PlayerProfile,
        recent_matches: Vec<models::RecentArenaMatchSummary>,
    ) -> PlayerProfileResponse {
        PlayerProfileResponse {
            player,
            recent_matches,
        }
    }
}

/// Converts the PlayerProfileResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerProfileResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping player in query parameter serialization

            // Skipping recent_matches in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlayerProfileResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlayerProfileResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub player: Vec<models::PlayerProfile>,
            pub recent_matches: Vec<Vec<models::RecentArenaMatchSummary>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlayerProfileResponse".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "player" => intermediate_rep.player.push(<models::PlayerProfile as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "recent_matches" => return std::result::Result::Err("Parsing a container in this style is not supported in PlayerProfileResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PlayerProfileResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlayerProfileResponse {
            player: intermediate_rep
                .player
                .into_iter()
                .next()
                .ok_or_else(|| "player missing in PlayerProfileResponse".to_string())?,
            recent_matches: intermediate_rep
                .recent_matches
                .into_iter()
                .next()
                .ok_or_else(|| "recent_matches missing in PlayerProfileResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlayerProfileResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PlayerProfileResponse>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlayerProfileResponse>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for PlayerProfileResponse - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PlayerProfileResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlayerProfileResponse as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into PlayerProfileResponse - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlayerSearchResponse {
    #[serde(rename = "results")]
    #[validate(nested)]
    pub results: Vec<models::PlayerSearchResult>,
}

impl PlayerSearchResponse {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(results: Vec<models::PlayerSearchResult>) -> PlayerSearchResponse {
        PlayerSearchResponse { results }
    }
}

/// Converts the PlayerSearchResponse value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            // Skipping results in query parameter serialization

        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlayerSearchResponse value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlayerSearchResponse {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub results: Vec<Vec<models::PlayerSearchResult>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlayerSearchResponse".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "results" => return std::result::Result::Err("Parsing a container in this style is not supported in PlayerSearchResponse".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing PlayerSearchResponse".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlayerSearchResponse {
            results: intermediate_rep
                .results
                .into_iter()
                .next()
                .ok_or_else(|| "results missing in PlayerSearchResponse".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlayerSearchResponse> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PlayerSearchResponse>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlayerSearchResponse>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for PlayerSearchResponse - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PlayerSearchResponse> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlayerSearchResponse as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into PlayerSearchResponse - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PlayerSearchResult {
    #[serde(rename = "puuid")]
    #[validate(custom(function = "check_xss_string"))]
    pub puuid: String,

    #[serde(rename = "game_name")]
    #[validate(custom(function = "check_xss_string"))]
    pub game_name: String,

    #[serde(rename = "tagline")]
    #[validate(custom(function = "check_xss_string"))]
    pub tagline: String,
}

impl PlayerSearchResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(puuid: String, game_name: String, tagline: String) -> PlayerSearchResult {
        PlayerSearchResult {
            puuid,
            game_name,
            tagline,
        }
    }
}

/// Converts the PlayerSearchResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerSearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("puuid".to_string()),
            Some(self.puuid.to_string()),
            Some("game_name".to_string()),
            Some(self.game_name.to_string()),
            Some("tagline".to_string()),
            Some(self.tagline.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlayerSearchResult value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlayerSearchResult {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub puuid: Vec<String>,
            pub game_name: Vec<String>,
            pub tagline: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing PlayerSearchResult".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "puuid" => intermediate_rep.puuid.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "game_name" => intermediate_rep.game_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tagline" => intermediate_rep.tagline.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing PlayerSearchResult".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlayerSearchResult {
            puuid: intermediate_rep
                .puuid
                .into_iter()
                .next()
                .ok_or_else(|| "puuid missing in PlayerSearchResult".to_string())?,
            game_name: intermediate_rep
                .game_name
                .into_iter()
                .next()
                .ok_or_else(|| "game_name missing in PlayerSearchResult".to_string())?,
            tagline: intermediate_rep
                .tagline
                .into_iter()
                .next()
                .ok_or_else(|| "tagline missing in PlayerSearchResult".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlayerSearchResult> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PlayerSearchResult>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlayerSearchResult>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for PlayerSearchResult - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PlayerSearchResult> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlayerSearchResult as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into PlayerSearchResult - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecentArenaMatchSummary {
    #[serde(rename = "match_id")]
    #[validate(custom(function = "check_xss_string"))]
    pub match_id: String,

    #[serde(rename = "queue_id")]
    pub queue_id: i32,

    #[serde(rename = "game_creation")]
    pub game_creation: i64,

    #[serde(rename = "game_end_timestamp")]
    pub game_end_timestamp: i64,

    #[serde(rename = "participants")]
    #[validate(nested)]
    pub participants: Vec<models::RecentArenaParticipantSummary>,
}

impl RecentArenaMatchSummary {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        match_id: String,
        queue_id: i32,
        game_creation: i64,
        game_end_timestamp: i64,
        participants: Vec<models::RecentArenaParticipantSummary>,
    ) -> RecentArenaMatchSummary {
        RecentArenaMatchSummary {
            match_id,
            queue_id,
            game_creation,
            game_end_timestamp,
            participants,
        }
    }
}

/// Converts the RecentArenaMatchSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RecentArenaMatchSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("match_id".to_string()),
            Some(self.match_id.to_string()),
            Some("queue_id".to_string()),
            Some(self.queue_id.to_string()),
            Some("game_creation".to_string()),
            Some(self.game_creation.to_string()),
            Some("game_end_timestamp".to_string()),
            Some(self.game_end_timestamp.to_string()),
            // Skipping participants in query parameter serialization
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecentArenaMatchSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecentArenaMatchSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub match_id: Vec<String>,
            pub queue_id: Vec<i32>,
            pub game_creation: Vec<i64>,
            pub game_end_timestamp: Vec<i64>,
            pub participants: Vec<Vec<models::RecentArenaParticipantSummary>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecentArenaMatchSummary".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "match_id" => intermediate_rep.match_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "queue_id" => intermediate_rep.queue_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "game_creation" => intermediate_rep.game_creation.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "game_end_timestamp" => intermediate_rep.game_end_timestamp.push(<i64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "participants" => return std::result::Result::Err("Parsing a container in this style is not supported in RecentArenaMatchSummary".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing RecentArenaMatchSummary".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecentArenaMatchSummary {
            match_id: intermediate_rep
                .match_id
                .into_iter()
                .next()
                .ok_or_else(|| "match_id missing in RecentArenaMatchSummary".to_string())?,
            queue_id: intermediate_rep
                .queue_id
                .into_iter()
                .next()
                .ok_or_else(|| "queue_id missing in RecentArenaMatchSummary".to_string())?,
            game_creation: intermediate_rep
                .game_creation
                .into_iter()
                .next()
                .ok_or_else(|| "game_creation missing in RecentArenaMatchSummary".to_string())?,
            game_end_timestamp: intermediate_rep
                .game_end_timestamp
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "game_end_timestamp missing in RecentArenaMatchSummary".to_string()
                })?,
            participants: intermediate_rep
                .participants
                .into_iter()
                .next()
                .ok_or_else(|| "participants missing in RecentArenaMatchSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecentArenaMatchSummary> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RecentArenaMatchSummary>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecentArenaMatchSummary>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for RecentArenaMatchSummary - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RecentArenaMatchSummary> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecentArenaMatchSummary as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into RecentArenaMatchSummary - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RecentArenaParticipantSummary {
    #[serde(rename = "puuid")]
    #[validate(custom(function = "check_xss_string"))]
    pub puuid: String,

    #[serde(rename = "placement")]
    #[validate(range(min = 1u32))]
    pub placement: u32,

    #[serde(rename = "champion_id")]
    #[validate(range(min = 0u32))]
    pub champion_id: u32,

    #[serde(rename = "kills")]
    #[validate(range(min = 0u32))]
    pub kills: u32,

    #[serde(rename = "deaths")]
    #[validate(range(min = 0u32))]
    pub deaths: u32,

    #[serde(rename = "assists")]
    #[validate(range(min = 0u32))]
    pub assists: u32,
}

impl RecentArenaParticipantSummary {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        puuid: String,
        placement: u32,
        champion_id: u32,
        kills: u32,
        deaths: u32,
        assists: u32,
    ) -> RecentArenaParticipantSummary {
        RecentArenaParticipantSummary {
            puuid,
            placement,
            champion_id,
            kills,
            deaths,
            assists,
        }
    }
}

/// Converts the RecentArenaParticipantSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for RecentArenaParticipantSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("puuid".to_string()),
            Some(self.puuid.to_string()),
            Some("placement".to_string()),
            Some(self.placement.to_string()),
            Some("champion_id".to_string()),
            Some(self.champion_id.to_string()),
            Some("kills".to_string()),
            Some(self.kills.to_string()),
            Some("deaths".to_string()),
            Some(self.deaths.to_string()),
            Some("assists".to_string()),
            Some(self.assists.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RecentArenaParticipantSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RecentArenaParticipantSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub puuid: Vec<String>,
            pub placement: Vec<u32>,
            pub champion_id: Vec<u32>,
            pub kills: Vec<u32>,
            pub deaths: Vec<u32>,
            pub assists: Vec<u32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => {
                    return std::result::Result::Err(
                        "Missing value while parsing RecentArenaParticipantSummary".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "puuid" => intermediate_rep.puuid.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "placement" => intermediate_rep.placement.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "champion_id" => intermediate_rep.champion_id.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "kills" => intermediate_rep.kills.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "deaths" => intermediate_rep.deaths.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "assists" => intermediate_rep.assists.push(
                        <u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing RecentArenaParticipantSummary"
                                .to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RecentArenaParticipantSummary {
            puuid: intermediate_rep
                .puuid
                .into_iter()
                .next()
                .ok_or_else(|| "puuid missing in RecentArenaParticipantSummary".to_string())?,
            placement: intermediate_rep
                .placement
                .into_iter()
                .next()
                .ok_or_else(|| "placement missing in RecentArenaParticipantSummary".to_string())?,
            champion_id: intermediate_rep
                .champion_id
                .into_iter()
                .next()
                .ok_or_else(|| {
                    "champion_id missing in RecentArenaParticipantSummary".to_string()
                })?,
            kills: intermediate_rep
                .kills
                .into_iter()
                .next()
                .ok_or_else(|| "kills missing in RecentArenaParticipantSummary".to_string())?,
            deaths: intermediate_rep
                .deaths
                .into_iter()
                .next()
                .ok_or_else(|| "deaths missing in RecentArenaParticipantSummary".to_string())?,
            assists: intermediate_rep
                .assists
                .into_iter()
                .next()
                .ok_or_else(|| "assists missing in RecentArenaParticipantSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RecentArenaParticipantSummary> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<RecentArenaParticipantSummary>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<RecentArenaParticipantSummary>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for RecentArenaParticipantSummary - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<RecentArenaParticipantSummary> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <RecentArenaParticipantSummary as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into RecentArenaParticipantSummary - {err}"#
                    )),
                }
            }
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Unable to convert header: {hdr_value:?} to string: {e}"#
            )),
        }
    }
}
