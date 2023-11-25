/*
 * SpaceTraders API
 *
 * SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.  The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.  ```json http {   \"method\": \"GET\",   \"url\": \"https://api.spacetraders.io/v2\", } ```  Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.  We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.   
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: joel@spacetraders.io
 * Generated by: https://openapi-generator.tech
 */

/// ShipNav : The navigation information of the ship.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipNav {
    /// The system symbol of the ship's current location.
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    /// The waypoint symbol of the ship's current location, or if the ship is in-transit, the waypoint symbol of the ship's destination.
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    #[serde(rename = "route")]
    pub route: Box<crate::models::ShipNavRoute>,
    #[serde(rename = "status")]
    pub status: crate::models::ShipNavStatus,
    #[serde(rename = "flightMode")]
    pub flight_mode: crate::models::ShipNavFlightMode,
}

impl ShipNav {
    /// The navigation information of the ship.
    pub fn new(system_symbol: String, waypoint_symbol: String, route: crate::models::ShipNavRoute, status: crate::models::ShipNavStatus, flight_mode: crate::models::ShipNavFlightMode) -> ShipNav {
        ShipNav {
            system_symbol,
            waypoint_symbol,
            route: Box::new(route),
            status,
            flight_mode,
        }
    }
}


