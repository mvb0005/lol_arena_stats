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
pub struct GetPlayerProfileQueryParams {
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SearchPlayerQueryParams {
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
pub struct ApiError {
    #[serde(rename = "code")]
    #[validate(custom(function = "check_xss_string"))]
    pub code: String,

    #[serde(rename = "message")]
    #[validate(custom(function = "check_xss_string"))]
    pub message: String,
}

impl ApiError {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(code: String, message: String) -> ApiError {
        ApiError { code, message }
    }
}

/// Converts the ApiError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("code".to_string()),
            Some(self.code.to_string()),
            Some("message".to_string()),
            Some(self.message.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub code: Vec<String>,
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
                        "Missing value while parsing ApiError".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "code" => intermediate_rep.code.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "message" => intermediate_rep.message.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    _ => {
                        return std::result::Result::Err(
                            "Unexpected key while parsing ApiError".to_string(),
                        );
                    }
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiError {
            code: intermediate_rep
                .code
                .into_iter()
                .next()
                .ok_or_else(|| "code missing in ApiError".to_string())?,
            message: intermediate_rep
                .message
                .into_iter()
                .next()
                .ok_or_else(|| "message missing in ApiError".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiError> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiError>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<ApiError>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for ApiError - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<ApiError> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <ApiError as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into ApiError - {err}"#
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
pub struct PlayerProfileSummary {
    #[serde(rename = "playerName")]
    #[validate(custom(function = "check_xss_string"))]
    pub player_name: String,

    #[serde(rename = "tagLine")]
    #[validate(custom(function = "check_xss_string"))]
    pub tag_line: String,

    #[serde(rename = "region")]
    #[validate(custom(function = "check_xss_string"))]
    pub region: String,

    #[serde(rename = "puuid")]
    #[validate(custom(function = "check_xss_string"))]
    pub puuid: String,

    #[serde(rename = "recentPlacements")]
    #[validate(length(min = 1))]
    pub recent_placements: Vec<u8>,

    #[serde(rename = "totalGames")]
    #[validate(range(min = 0u32))]
    pub total_games: u32,

    #[serde(rename = "winRate")]
    #[validate(range(min = 0f64, max = 1f64))]
    pub win_rate: f64,

    #[serde(rename = "lastUpdated")]
    #[validate(custom(function = "check_xss_string"))]
    pub last_updated: String,
}

impl PlayerProfileSummary {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        player_name: String,
        tag_line: String,
        region: String,
        puuid: String,
        recent_placements: Vec<u8>,
        total_games: u32,
        win_rate: f64,
        last_updated: String,
    ) -> PlayerProfileSummary {
        PlayerProfileSummary {
            player_name,
            tag_line,
            region,
            puuid,
            recent_placements,
            total_games,
            win_rate,
            last_updated,
        }
    }
}

/// Converts the PlayerProfileSummary value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerProfileSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("playerName".to_string()),
            Some(self.player_name.to_string()),
            Some("tagLine".to_string()),
            Some(self.tag_line.to_string()),
            Some("region".to_string()),
            Some(self.region.to_string()),
            Some("puuid".to_string()),
            Some(self.puuid.to_string()),
            Some("recentPlacements".to_string()),
            Some(
                self.recent_placements
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            Some("totalGames".to_string()),
            Some(self.total_games.to_string()),
            Some("winRate".to_string()),
            Some(self.win_rate.to_string()),
            Some("lastUpdated".to_string()),
            Some(self.last_updated.to_string()),
        ];

        write!(
            f,
            "{}",
            params.into_iter().flatten().collect::<Vec<_>>().join(",")
        )
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PlayerProfileSummary value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PlayerProfileSummary {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub player_name: Vec<String>,
            pub tag_line: Vec<String>,
            pub region: Vec<String>,
            pub puuid: Vec<String>,
            pub recent_placements: Vec<Vec<u8>>,
            pub total_games: Vec<u32>,
            pub win_rate: Vec<f64>,
            pub last_updated: Vec<String>,
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
                        "Missing value while parsing PlayerProfileSummary".to_string(),
                    );
                }
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "playerName" => intermediate_rep.player_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "tagLine" => intermediate_rep.tag_line.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "region" => intermediate_rep.region.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "puuid" => intermediate_rep.puuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "recentPlacements" => return std::result::Result::Err("Parsing a container in this style is not supported in PlayerProfileSummary".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "totalGames" => intermediate_rep.total_games.push(<u32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "winRate" => intermediate_rep.win_rate.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "lastUpdated" => intermediate_rep.last_updated.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PlayerProfileSummary".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PlayerProfileSummary {
            player_name: intermediate_rep
                .player_name
                .into_iter()
                .next()
                .ok_or_else(|| "playerName missing in PlayerProfileSummary".to_string())?,
            tag_line: intermediate_rep
                .tag_line
                .into_iter()
                .next()
                .ok_or_else(|| "tagLine missing in PlayerProfileSummary".to_string())?,
            region: intermediate_rep
                .region
                .into_iter()
                .next()
                .ok_or_else(|| "region missing in PlayerProfileSummary".to_string())?,
            puuid: intermediate_rep
                .puuid
                .into_iter()
                .next()
                .ok_or_else(|| "puuid missing in PlayerProfileSummary".to_string())?,
            recent_placements: intermediate_rep
                .recent_placements
                .into_iter()
                .next()
                .ok_or_else(|| "recentPlacements missing in PlayerProfileSummary".to_string())?,
            total_games: intermediate_rep
                .total_games
                .into_iter()
                .next()
                .ok_or_else(|| "totalGames missing in PlayerProfileSummary".to_string())?,
            win_rate: intermediate_rep
                .win_rate
                .into_iter()
                .next()
                .ok_or_else(|| "winRate missing in PlayerProfileSummary".to_string())?,
            last_updated: intermediate_rep
                .last_updated
                .into_iter()
                .next()
                .ok_or_else(|| "lastUpdated missing in PlayerProfileSummary".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PlayerProfileSummary> and HeaderValue

#[cfg(feature = "server")]
impl std::convert::TryFrom<header::IntoHeaderValue<PlayerProfileSummary>> for HeaderValue {
    type Error = String;

    fn try_from(
        hdr_value: header::IntoHeaderValue<PlayerProfileSummary>,
    ) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match HeaderValue::from_str(&hdr_value) {
            std::result::Result::Ok(value) => std::result::Result::Ok(value),
            std::result::Result::Err(e) => std::result::Result::Err(format!(
                r#"Invalid header value for PlayerProfileSummary - value: {hdr_value} is invalid {e}"#
            )),
        }
    }
}

#[cfg(feature = "server")]
impl std::convert::TryFrom<HeaderValue> for header::IntoHeaderValue<PlayerProfileSummary> {
    type Error = String;

    fn try_from(hdr_value: HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
            std::result::Result::Ok(value) => {
                match <PlayerProfileSummary as std::str::FromStr>::from_str(value) {
                    std::result::Result::Ok(value) => {
                        std::result::Result::Ok(header::IntoHeaderValue(value))
                    }
                    std::result::Result::Err(err) => std::result::Result::Err(format!(
                        r#"Unable to convert header value '{value}' into PlayerProfileSummary - {err}"#
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
    #[serde(rename = "playerName")]
    #[validate(custom(function = "check_xss_string"))]
    pub player_name: String,

    #[serde(rename = "tagLine")]
    #[validate(custom(function = "check_xss_string"))]
    pub tag_line: String,

    #[serde(rename = "region")]
    #[validate(custom(function = "check_xss_string"))]
    pub region: String,

    #[serde(rename = "puuid")]
    #[validate(custom(function = "check_xss_string"))]
    pub puuid: String,

    #[serde(rename = "lastUpdated")]
    #[validate(custom(function = "check_xss_string"))]
    pub last_updated: String,
}

impl PlayerSearchResult {
    #[allow(clippy::new_without_default, clippy::too_many_arguments)]
    pub fn new(
        player_name: String,
        tag_line: String,
        region: String,
        puuid: String,
        last_updated: String,
    ) -> PlayerSearchResult {
        PlayerSearchResult {
            player_name,
            tag_line,
            region,
            puuid,
            last_updated,
        }
    }
}

/// Converts the PlayerSearchResult value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::fmt::Display for PlayerSearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<Option<String>> = vec![
            Some("playerName".to_string()),
            Some(self.player_name.to_string()),
            Some("tagLine".to_string()),
            Some(self.tag_line.to_string()),
            Some("region".to_string()),
            Some(self.region.to_string()),
            Some("puuid".to_string()),
            Some(self.puuid.to_string()),
            Some("lastUpdated".to_string()),
            Some(self.last_updated.to_string()),
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
            pub player_name: Vec<String>,
            pub tag_line: Vec<String>,
            pub region: Vec<String>,
            pub puuid: Vec<String>,
            pub last_updated: Vec<String>,
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
                    "playerName" => intermediate_rep.player_name.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "tagLine" => intermediate_rep.tag_line.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "region" => intermediate_rep.region.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "puuid" => intermediate_rep.puuid.push(
                        <String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?,
                    ),
                    #[allow(clippy::redundant_clone)]
                    "lastUpdated" => intermediate_rep.last_updated.push(
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
            player_name: intermediate_rep
                .player_name
                .into_iter()
                .next()
                .ok_or_else(|| "playerName missing in PlayerSearchResult".to_string())?,
            tag_line: intermediate_rep
                .tag_line
                .into_iter()
                .next()
                .ok_or_else(|| "tagLine missing in PlayerSearchResult".to_string())?,
            region: intermediate_rep
                .region
                .into_iter()
                .next()
                .ok_or_else(|| "region missing in PlayerSearchResult".to_string())?,
            puuid: intermediate_rep
                .puuid
                .into_iter()
                .next()
                .ok_or_else(|| "puuid missing in PlayerSearchResult".to_string())?,
            last_updated: intermediate_rep
                .last_updated
                .into_iter()
                .next()
                .ok_or_else(|| "lastUpdated missing in PlayerSearchResult".to_string())?,
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
