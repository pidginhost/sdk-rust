# DnsRecordCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Record hostname (use '@' or leave empty for zone apex). | 
**ttl** | **i32** | Time to live in seconds. | 
**r#type** | [**models::DnsRecordCreateTypeEnum**](DNSRecordCreateTypeEnum.md) | DNS record type.  * `A` - A * `AAAA` - AAAA * `TYPE257` - TYPE257 * `CNAME` - CNAME * `MX` - MX * `SRV` - SRV * `TXT` - TXT | 
**address** | Option<**String**> | IPv4/IPv6 address (A/AAAA). | [optional]
**cname** | Option<**String**> | Canonical name (CNAME). | [optional]
**exchange** | Option<**String**> | Mail exchange host (MX). | [optional]
**preference** | Option<**i32**> | MX preference / priority. | [optional]
**txtdata** | Option<**String**> | TXT record data. | [optional]
**unencoded** | Option<**String**> | Unencoded TXT value. | [optional]
**target** | Option<**String**> | SRV target host. | [optional]
**priority** | Option<**i32**> | SRV priority. | [optional]
**weight** | Option<**i32**> | SRV weight. | [optional]
**port** | Option<**i32**> | SRV port. | [optional]
**flag** | Option<**i32**> | CAA flag (TYPE257). | [optional]
**tag** | Option<**String**> | CAA tag (TYPE257). | [optional]
**value** | Option<**String**> | CAA value (TYPE257). | [optional]
**line** | Option<**i32**> | Line number of existing record to edit. Omit to add a new record. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


