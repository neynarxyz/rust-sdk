# WebhookSubscriptionFiltersCast

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**exclude_author_fids** | Option<**Vec<i32>**> | Exclude casts that matches these authors. **Note:** This is applied as an AND operation against rest of the filters. Rest of the filters are bundled as an OR operation.  | [optional]
**author_fids** | Option<**Vec<i32>**> |  | [optional]
**mentioned_fids** | Option<**Vec<i32>**> |  | [optional]
**parent_urls** | Option<**Vec<String>**> |  | [optional]
**root_parent_urls** | Option<**Vec<String>**> |  | [optional]
**parent_hashes** | Option<**Vec<String>**> |  | [optional]
**parent_author_fids** | Option<**Vec<i32>**> |  | [optional]
**text** | Option<**String**> | Regex pattern to match the text key of the cast. **Note:**  1) Regex must be parsed by Go's RE2 engine (Test your expression here: https://www.lddgo.net/en/string/golangregex) 2) Use backslashes to escape special characters. For example: (?i)\\\\$degen should be written as (?i)\\\\\\\\$degen  | [optional]
**embeds** | Option<**String**> | Regex pattern to match the embeded_url (key embeds) of the cast. **Note:**  1) Regex must be parsed by Go's RE2 engine (Test your expression here: https://www.lddgo.net/en/string/golangregex) 2) Use backslashes to escape special characters. For example: \\\\b(farcaster|neynar)\\\\b should be written as \\\\\\\\b(farcaster|neynar)\\\\\\\\b  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


