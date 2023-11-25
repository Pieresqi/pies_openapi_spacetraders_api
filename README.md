# Rust API client for openapi

SpaceTraders is an open-universe game and learning platform that offers a set of HTTP endpoints to control a fleet of ships and explore a multiplayer universe.

The API is documented using [OpenAPI](https://github.com/SpaceTradersAPI/api-docs). You can send your first request right here in your browser to check the status of the game server.

```json http
{
  \"method\": \"GET\",
  \"url\": \"https://api.spacetraders.io/v2\",
}
```

Unlike a traditional game, SpaceTraders does not have a first-party client or app to play the game. Instead, you can use the API to build your own client, write a script to automate your ships, or try an app built by the community.

We have a [Discord channel](https://discord.com/invite/jh6zurdWk5) where you can share your projects, ask questions, and get help from other players.





## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 2.0.0
- Package version: 2.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.spacetraders.io/v2*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AgentsApi* | [**get_agent**](docs/AgentsApi.md#get_agent) | **GET** /agents/{agentSymbol} | Get Public Agent
*AgentsApi* | [**get_agents**](docs/AgentsApi.md#get_agents) | **GET** /agents | List Agents
*AgentsApi* | [**get_my_agent**](docs/AgentsApi.md#get_my_agent) | **GET** /my/agent | Get Agent
*ContractsApi* | [**accept_contract**](docs/ContractsApi.md#accept_contract) | **POST** /my/contracts/{contractId}/accept | Accept Contract
*ContractsApi* | [**deliver_contract**](docs/ContractsApi.md#deliver_contract) | **POST** /my/contracts/{contractId}/deliver | Deliver Cargo to Contract
*ContractsApi* | [**fulfill_contract**](docs/ContractsApi.md#fulfill_contract) | **POST** /my/contracts/{contractId}/fulfill | Fulfill Contract
*ContractsApi* | [**get_contract**](docs/ContractsApi.md#get_contract) | **GET** /my/contracts/{contractId} | Get Contract
*ContractsApi* | [**get_contracts**](docs/ContractsApi.md#get_contracts) | **GET** /my/contracts | List Contracts
*DefaultApi* | [**get_status**](docs/DefaultApi.md#get_status) | **GET** / | Get Status
*DefaultApi* | [**register**](docs/DefaultApi.md#register) | **POST** /register | Register New Agent
*FactionsApi* | [**get_faction**](docs/FactionsApi.md#get_faction) | **GET** /factions/{factionSymbol} | Get Faction
*FactionsApi* | [**get_factions**](docs/FactionsApi.md#get_factions) | **GET** /factions | List Factions
*FleetApi* | [**create_chart**](docs/FleetApi.md#create_chart) | **POST** /my/ships/{shipSymbol}/chart | Create Chart
*FleetApi* | [**create_ship_ship_scan**](docs/FleetApi.md#create_ship_ship_scan) | **POST** /my/ships/{shipSymbol}/scan/ships | Scan Ships
*FleetApi* | [**create_ship_system_scan**](docs/FleetApi.md#create_ship_system_scan) | **POST** /my/ships/{shipSymbol}/scan/systems | Scan Systems
*FleetApi* | [**create_ship_waypoint_scan**](docs/FleetApi.md#create_ship_waypoint_scan) | **POST** /my/ships/{shipSymbol}/scan/waypoints | Scan Waypoints
*FleetApi* | [**create_survey**](docs/FleetApi.md#create_survey) | **POST** /my/ships/{shipSymbol}/survey | Create Survey
*FleetApi* | [**dock_ship**](docs/FleetApi.md#dock_ship) | **POST** /my/ships/{shipSymbol}/dock | Dock Ship
*FleetApi* | [**extract_resources**](docs/FleetApi.md#extract_resources) | **POST** /my/ships/{shipSymbol}/extract | Extract Resources
*FleetApi* | [**extract_resources_with_survey**](docs/FleetApi.md#extract_resources_with_survey) | **POST** /my/ships/{shipSymbol}/extract/survey | Extract Resources with Survey
*FleetApi* | [**get_mounts**](docs/FleetApi.md#get_mounts) | **GET** /my/ships/{shipSymbol}/mounts | Get Mounts
*FleetApi* | [**get_my_ship**](docs/FleetApi.md#get_my_ship) | **GET** /my/ships/{shipSymbol} | Get Ship
*FleetApi* | [**get_my_ship_cargo**](docs/FleetApi.md#get_my_ship_cargo) | **GET** /my/ships/{shipSymbol}/cargo | Get Ship Cargo
*FleetApi* | [**get_my_ships**](docs/FleetApi.md#get_my_ships) | **GET** /my/ships | List Ships
*FleetApi* | [**get_ship_cooldown**](docs/FleetApi.md#get_ship_cooldown) | **GET** /my/ships/{shipSymbol}/cooldown | Get Ship Cooldown
*FleetApi* | [**get_ship_nav**](docs/FleetApi.md#get_ship_nav) | **GET** /my/ships/{shipSymbol}/nav | Get Ship Nav
*FleetApi* | [**install_mount**](docs/FleetApi.md#install_mount) | **POST** /my/ships/{shipSymbol}/mounts/install | Install Mount
*FleetApi* | [**jettison**](docs/FleetApi.md#jettison) | **POST** /my/ships/{shipSymbol}/jettison | Jettison Cargo
*FleetApi* | [**jump_ship**](docs/FleetApi.md#jump_ship) | **POST** /my/ships/{shipSymbol}/jump | Jump Ship
*FleetApi* | [**navigate_ship**](docs/FleetApi.md#navigate_ship) | **POST** /my/ships/{shipSymbol}/navigate | Navigate Ship
*FleetApi* | [**negotiate_contract**](docs/FleetApi.md#negotiate_contract) | **POST** /my/ships/{shipSymbol}/negotiate/contract | Negotiate Contract
*FleetApi* | [**orbit_ship**](docs/FleetApi.md#orbit_ship) | **POST** /my/ships/{shipSymbol}/orbit | Orbit Ship
*FleetApi* | [**patch_ship_nav**](docs/FleetApi.md#patch_ship_nav) | **PATCH** /my/ships/{shipSymbol}/nav | Patch Ship Nav
*FleetApi* | [**purchase_cargo**](docs/FleetApi.md#purchase_cargo) | **POST** /my/ships/{shipSymbol}/purchase | Purchase Cargo
*FleetApi* | [**purchase_ship**](docs/FleetApi.md#purchase_ship) | **POST** /my/ships | Purchase Ship
*FleetApi* | [**refuel_ship**](docs/FleetApi.md#refuel_ship) | **POST** /my/ships/{shipSymbol}/refuel | Refuel Ship
*FleetApi* | [**remove_mount**](docs/FleetApi.md#remove_mount) | **POST** /my/ships/{shipSymbol}/mounts/remove | Remove Mount
*FleetApi* | [**sell_cargo**](docs/FleetApi.md#sell_cargo) | **POST** /my/ships/{shipSymbol}/sell | Sell Cargo
*FleetApi* | [**ship_refine**](docs/FleetApi.md#ship_refine) | **POST** /my/ships/{shipSymbol}/refine | Ship Refine
*FleetApi* | [**siphon_resources**](docs/FleetApi.md#siphon_resources) | **POST** /my/ships/{shipSymbol}/siphon | Siphon Resources
*FleetApi* | [**transfer_cargo**](docs/FleetApi.md#transfer_cargo) | **POST** /my/ships/{shipSymbol}/transfer | Transfer Cargo
*FleetApi* | [**warp_ship**](docs/FleetApi.md#warp_ship) | **POST** /my/ships/{shipSymbol}/warp | Warp Ship
*SystemsApi* | [**get_construction**](docs/SystemsApi.md#get_construction) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/construction | Get Construction Site
*SystemsApi* | [**get_jump_gate**](docs/SystemsApi.md#get_jump_gate) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/jump-gate | Get Jump Gate
*SystemsApi* | [**get_market**](docs/SystemsApi.md#get_market) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/market | Get Market
*SystemsApi* | [**get_shipyard**](docs/SystemsApi.md#get_shipyard) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol}/shipyard | Get Shipyard
*SystemsApi* | [**get_system**](docs/SystemsApi.md#get_system) | **GET** /systems/{systemSymbol} | Get System
*SystemsApi* | [**get_system_waypoints**](docs/SystemsApi.md#get_system_waypoints) | **GET** /systems/{systemSymbol}/waypoints | List Waypoints in System
*SystemsApi* | [**get_systems**](docs/SystemsApi.md#get_systems) | **GET** /systems | List Systems
*SystemsApi* | [**get_waypoint**](docs/SystemsApi.md#get_waypoint) | **GET** /systems/{systemSymbol}/waypoints/{waypointSymbol} | Get Waypoint
*SystemsApi* | [**supply_construction**](docs/SystemsApi.md#supply_construction) | **POST** /systems/{systemSymbol}/waypoints/{waypointSymbol}/construction/supply | Supply Construction Site


## Documentation For Models

 - [AcceptContract200Response](docs/AcceptContract200Response.md)
 - [AcceptContract200ResponseData](docs/AcceptContract200ResponseData.md)
 - [ActivityLevel](docs/ActivityLevel.md)
 - [Agent](docs/Agent.md)
 - [Chart](docs/Chart.md)
 - [Construction](docs/Construction.md)
 - [ConstructionMaterial](docs/ConstructionMaterial.md)
 - [Contract](docs/Contract.md)
 - [ContractDeliverGood](docs/ContractDeliverGood.md)
 - [ContractPayment](docs/ContractPayment.md)
 - [ContractTerms](docs/ContractTerms.md)
 - [Cooldown](docs/Cooldown.md)
 - [CreateChart201Response](docs/CreateChart201Response.md)
 - [CreateChart201ResponseData](docs/CreateChart201ResponseData.md)
 - [CreateShipShipScan201Response](docs/CreateShipShipScan201Response.md)
 - [CreateShipShipScan201ResponseData](docs/CreateShipShipScan201ResponseData.md)
 - [CreateShipSystemScan201Response](docs/CreateShipSystemScan201Response.md)
 - [CreateShipSystemScan201ResponseData](docs/CreateShipSystemScan201ResponseData.md)
 - [CreateShipWaypointScan201Response](docs/CreateShipWaypointScan201Response.md)
 - [CreateShipWaypointScan201ResponseData](docs/CreateShipWaypointScan201ResponseData.md)
 - [CreateSurvey201Response](docs/CreateSurvey201Response.md)
 - [CreateSurvey201ResponseData](docs/CreateSurvey201ResponseData.md)
 - [DeliverContract200Response](docs/DeliverContract200Response.md)
 - [DeliverContract200ResponseData](docs/DeliverContract200ResponseData.md)
 - [DeliverContractRequest](docs/DeliverContractRequest.md)
 - [DockShip200Response](docs/DockShip200Response.md)
 - [ExtractResources201Response](docs/ExtractResources201Response.md)
 - [ExtractResources201ResponseData](docs/ExtractResources201ResponseData.md)
 - [ExtractResourcesRequest](docs/ExtractResourcesRequest.md)
 - [Extraction](docs/Extraction.md)
 - [ExtractionYield](docs/ExtractionYield.md)
 - [Faction](docs/Faction.md)
 - [FactionSymbols](docs/FactionSymbols.md)
 - [FactionTrait](docs/FactionTrait.md)
 - [FulfillContract200Response](docs/FulfillContract200Response.md)
 - [GetAgents200Response](docs/GetAgents200Response.md)
 - [GetConstruction200Response](docs/GetConstruction200Response.md)
 - [GetContract200Response](docs/GetContract200Response.md)
 - [GetContracts200Response](docs/GetContracts200Response.md)
 - [GetFaction200Response](docs/GetFaction200Response.md)
 - [GetFactions200Response](docs/GetFactions200Response.md)
 - [GetJumpGate200Response](docs/GetJumpGate200Response.md)
 - [GetMarket200Response](docs/GetMarket200Response.md)
 - [GetMounts200Response](docs/GetMounts200Response.md)
 - [GetMyAgent200Response](docs/GetMyAgent200Response.md)
 - [GetMyShip200Response](docs/GetMyShip200Response.md)
 - [GetMyShipCargo200Response](docs/GetMyShipCargo200Response.md)
 - [GetMyShips200Response](docs/GetMyShips200Response.md)
 - [GetShipCooldown200Response](docs/GetShipCooldown200Response.md)
 - [GetShipNav200Response](docs/GetShipNav200Response.md)
 - [GetShipyard200Response](docs/GetShipyard200Response.md)
 - [GetStatus200Response](docs/GetStatus200Response.md)
 - [GetStatus200ResponseAnnouncementsInner](docs/GetStatus200ResponseAnnouncementsInner.md)
 - [GetStatus200ResponseLeaderboards](docs/GetStatus200ResponseLeaderboards.md)
 - [GetStatus200ResponseLeaderboardsMostCreditsInner](docs/GetStatus200ResponseLeaderboardsMostCreditsInner.md)
 - [GetStatus200ResponseLeaderboardsMostSubmittedChartsInner](docs/GetStatus200ResponseLeaderboardsMostSubmittedChartsInner.md)
 - [GetStatus200ResponseLinksInner](docs/GetStatus200ResponseLinksInner.md)
 - [GetStatus200ResponseServerResets](docs/GetStatus200ResponseServerResets.md)
 - [GetStatus200ResponseStats](docs/GetStatus200ResponseStats.md)
 - [GetSystem200Response](docs/GetSystem200Response.md)
 - [GetSystemWaypoints200Response](docs/GetSystemWaypoints200Response.md)
 - [GetSystemWaypointsTraitsParameter](docs/GetSystemWaypointsTraitsParameter.md)
 - [GetSystems200Response](docs/GetSystems200Response.md)
 - [GetWaypoint200Response](docs/GetWaypoint200Response.md)
 - [InstallMount201Response](docs/InstallMount201Response.md)
 - [InstallMount201ResponseData](docs/InstallMount201ResponseData.md)
 - [InstallMountRequest](docs/InstallMountRequest.md)
 - [Jettison200Response](docs/Jettison200Response.md)
 - [Jettison200ResponseData](docs/Jettison200ResponseData.md)
 - [JettisonRequest](docs/JettisonRequest.md)
 - [JumpGate](docs/JumpGate.md)
 - [JumpShip200Response](docs/JumpShip200Response.md)
 - [JumpShip200ResponseData](docs/JumpShip200ResponseData.md)
 - [JumpShipRequest](docs/JumpShipRequest.md)
 - [Market](docs/Market.md)
 - [MarketTradeGood](docs/MarketTradeGood.md)
 - [MarketTransaction](docs/MarketTransaction.md)
 - [Meta](docs/Meta.md)
 - [NavigateShip200Response](docs/NavigateShip200Response.md)
 - [NavigateShip200ResponseData](docs/NavigateShip200ResponseData.md)
 - [NavigateShipRequest](docs/NavigateShipRequest.md)
 - [NegotiateContract200Response](docs/NegotiateContract200Response.md)
 - [NegotiateContract200ResponseData](docs/NegotiateContract200ResponseData.md)
 - [OrbitShip200Response](docs/OrbitShip200Response.md)
 - [OrbitShip200ResponseData](docs/OrbitShip200ResponseData.md)
 - [PatchShipNavRequest](docs/PatchShipNavRequest.md)
 - [PurchaseCargo201Response](docs/PurchaseCargo201Response.md)
 - [PurchaseCargoRequest](docs/PurchaseCargoRequest.md)
 - [PurchaseShip201Response](docs/PurchaseShip201Response.md)
 - [PurchaseShip201ResponseData](docs/PurchaseShip201ResponseData.md)
 - [PurchaseShipRequest](docs/PurchaseShipRequest.md)
 - [RefuelShip200Response](docs/RefuelShip200Response.md)
 - [RefuelShip200ResponseData](docs/RefuelShip200ResponseData.md)
 - [RefuelShipRequest](docs/RefuelShipRequest.md)
 - [Register201Response](docs/Register201Response.md)
 - [Register201ResponseData](docs/Register201ResponseData.md)
 - [RegisterRequest](docs/RegisterRequest.md)
 - [RemoveMount201Response](docs/RemoveMount201Response.md)
 - [RemoveMount201ResponseData](docs/RemoveMount201ResponseData.md)
 - [RemoveMountRequest](docs/RemoveMountRequest.md)
 - [ScannedShip](docs/ScannedShip.md)
 - [ScannedShipEngine](docs/ScannedShipEngine.md)
 - [ScannedShipFrame](docs/ScannedShipFrame.md)
 - [ScannedShipMountsInner](docs/ScannedShipMountsInner.md)
 - [ScannedShipReactor](docs/ScannedShipReactor.md)
 - [ScannedSystem](docs/ScannedSystem.md)
 - [ScannedWaypoint](docs/ScannedWaypoint.md)
 - [SellCargo201Response](docs/SellCargo201Response.md)
 - [SellCargo201ResponseData](docs/SellCargo201ResponseData.md)
 - [SellCargoRequest](docs/SellCargoRequest.md)
 - [Ship](docs/Ship.md)
 - [ShipCargo](docs/ShipCargo.md)
 - [ShipCargoItem](docs/ShipCargoItem.md)
 - [ShipCrew](docs/ShipCrew.md)
 - [ShipEngine](docs/ShipEngine.md)
 - [ShipFrame](docs/ShipFrame.md)
 - [ShipFuel](docs/ShipFuel.md)
 - [ShipFuelConsumed](docs/ShipFuelConsumed.md)
 - [ShipModificationTransaction](docs/ShipModificationTransaction.md)
 - [ShipModule](docs/ShipModule.md)
 - [ShipMount](docs/ShipMount.md)
 - [ShipNav](docs/ShipNav.md)
 - [ShipNavFlightMode](docs/ShipNavFlightMode.md)
 - [ShipNavRoute](docs/ShipNavRoute.md)
 - [ShipNavRouteWaypoint](docs/ShipNavRouteWaypoint.md)
 - [ShipNavStatus](docs/ShipNavStatus.md)
 - [ShipReactor](docs/ShipReactor.md)
 - [ShipRefine201Response](docs/ShipRefine201Response.md)
 - [ShipRefine201ResponseData](docs/ShipRefine201ResponseData.md)
 - [ShipRefine201ResponseDataProducedInner](docs/ShipRefine201ResponseDataProducedInner.md)
 - [ShipRefineRequest](docs/ShipRefineRequest.md)
 - [ShipRegistration](docs/ShipRegistration.md)
 - [ShipRequirements](docs/ShipRequirements.md)
 - [ShipRole](docs/ShipRole.md)
 - [ShipType](docs/ShipType.md)
 - [Shipyard](docs/Shipyard.md)
 - [ShipyardShip](docs/ShipyardShip.md)
 - [ShipyardShipCrew](docs/ShipyardShipCrew.md)
 - [ShipyardShipTypesInner](docs/ShipyardShipTypesInner.md)
 - [ShipyardTransaction](docs/ShipyardTransaction.md)
 - [Siphon](docs/Siphon.md)
 - [SiphonResources201Response](docs/SiphonResources201Response.md)
 - [SiphonResources201ResponseData](docs/SiphonResources201ResponseData.md)
 - [SiphonYield](docs/SiphonYield.md)
 - [SupplyConstruction200Response](docs/SupplyConstruction200Response.md)
 - [SupplyConstruction200ResponseData](docs/SupplyConstruction200ResponseData.md)
 - [SupplyConstructionRequest](docs/SupplyConstructionRequest.md)
 - [SupplyLevel](docs/SupplyLevel.md)
 - [Survey](docs/Survey.md)
 - [SurveyDeposit](docs/SurveyDeposit.md)
 - [System](docs/System.md)
 - [SystemFaction](docs/SystemFaction.md)
 - [SystemType](docs/SystemType.md)
 - [SystemWaypoint](docs/SystemWaypoint.md)
 - [TradeGood](docs/TradeGood.md)
 - [TradeSymbol](docs/TradeSymbol.md)
 - [TransferCargo200Response](docs/TransferCargo200Response.md)
 - [TransferCargoRequest](docs/TransferCargoRequest.md)
 - [Waypoint](docs/Waypoint.md)
 - [WaypointFaction](docs/WaypointFaction.md)
 - [WaypointModifier](docs/WaypointModifier.md)
 - [WaypointOrbital](docs/WaypointOrbital.md)
 - [WaypointTrait](docs/WaypointTrait.md)
 - [WaypointType](docs/WaypointType.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

joel@spacetraders.io

