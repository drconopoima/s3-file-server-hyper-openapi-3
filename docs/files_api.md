# files_api

All URIs are relative to *http://localhost:8000/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**createFile**](files_api.md#createFile) | **POST** /files | Create a file object metadata
**createShareURL**](files_api.md#createShareURL) | **GET** /files/{fileId}/share | Generate a shareable URL for a file object
**deleteFileById**](files_api.md#deleteFileById) | **DELETE** /files/{fileId} | Delete a file object content and metadata properties
**downloadFile**](files_api.md#downloadFile) | **GET** /files/{fileId}/download | Download a file
**getFileById**](files_api.md#getFileById) | **GET** /files/{fileId} | Retrieve specific file object metadata properties
**listFiles**](files_api.md#listFiles) | **GET** /files | List all files
**recoverFile**](files_api.md#recoverFile) | **POST** /files/{fileId}/recover | Recovers a deleted file object content and metadata properties
**uploadFile**](files_api.md#uploadFile) | **POST** /files/{fileId}/upload | Upload a file


# **createFile**
> models::File createFile(file)
Create a file object metadata

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file** | [**File**](File.md)| File Object Information | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createShareURL**
> models::File createShareURL(file_id, optional)
Generate a shareable URL for a file object

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file to retrieve | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_id** | [****](.md)| The id of the file to retrieve | 
 **expires** | **chrono::DateTime::<chrono::Utc>**|  | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteFileById**
> models::File deleteFileById(file_id)
Delete a file object content and metadata properties

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file object to delete | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **downloadFile**
> swagger::ByteArray downloadFile(file_id, optional)
Download a file

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file to download | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_id** | [****](.md)| The id of the file to download | 
 **token** | **String**|  | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getFileById**
> models::File getFileById(file_id)
Retrieve specific file object metadata properties

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file object to retrieve | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listFiles**
> Vec<models::File> listFiles(optional)
List all files

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| How many items to return at one time (max 100) | 

### Return type

[**Vec<models::File>**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **recoverFile**
> models::File recoverFile(file_id)
Recovers a deleted file object content and metadata properties

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file object to recover | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **uploadFile**
> models::File uploadFile(file_id)
Upload a file

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **file_id** | [****](.md)| The id of the file to upload | 

### Return type

[**models::File**](File.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

