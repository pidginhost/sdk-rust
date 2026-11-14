# ServerRoute

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**server** | **i32** |  | 
**hostname** | **String** |  | 
**power** | [**models::RoutePowerEnum**](RoutePowerEnum.md) |  | 
**config** | [**models::RouteConfigEnum**](RouteConfigEnum.md) | The spec 6.2 rule over current config.  * `ok` - ok * `missing` - missing * `duplicate` - duplicate * `obsolete` - obsolete * `unknown` - unknown | 
**pending** | Option<[**models::RouteConfigEnum**](RouteConfigEnum.md)> | The rule over the config a reboot applies; null without one.  * `ok` - ok * `missing` - missing * `duplicate` - duplicate * `obsolete` - obsolete * `unknown` - unknown | 
**routes_here** | **bool** | The rule gives this network's interface the default route. | 
**acknowledged** | **bool** | The last route apply's config and boot still hold. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


