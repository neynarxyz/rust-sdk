# OembedRichData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**version** | **String** |  | 
**title** | Option<**String**> | A text title, describing the resource. | [optional]
**author_name** | Option<**String**> | The name of the author/owner of the resource. | [optional]
**author_url** | Option<**String**> | A URL for the author/owner of the resource. | [optional]
**provider_name** | Option<**String**> | The name of the resource provider. | [optional]
**provider_url** | Option<**String**> | The url of the resource provider. | [optional]
**cache_age** | Option<**String**> | The suggested cache lifetime for this resource, in seconds. Consumers may choose to use this value or not. | [optional]
**thumbnail_url** | Option<**String**> | A URL to a thumbnail image representing the resource. The thumbnail must respect any maxwidth and maxheight parameters. If this parameter is present, thumbnail_width and thumbnail_height must also be present. | [optional]
**thumbnail_width** | Option<**f64**> | The width of the optional thumbnail. If this parameter is present, thumbnail_url and thumbnail_height must also be present. | [optional]
**thumbnail_height** | Option<**f64**> | The height of the optional thumbnail. If this parameter is present, thumbnail_url and thumbnail_width must also be present. | [optional]
**html** | Option<**String**> | The HTML required to display the resource. The HTML should have no padding or margins. Consumers may wish to load the HTML in an off-domain iframe to avoid XSS vulnerabilities. The markup should be valid XHTML 1.0 Basic. | 
**width** | Option<**f64**> | The width in pixels required to display the HTML. | 
**height** | Option<**f64**> | The height in pixels required to display the HTML. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


