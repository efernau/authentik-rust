# \RbacApi

All URIs are relative to *http://localhost/api/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rbac_permissions_assigned_by_roles_assign_create**](RbacApi.md#rbac_permissions_assigned_by_roles_assign_create) | **POST** /rbac/permissions/assigned_by_roles/{uuid}/assign/ | 
[**rbac_permissions_assigned_by_roles_list**](RbacApi.md#rbac_permissions_assigned_by_roles_list) | **GET** /rbac/permissions/assigned_by_roles/ | 
[**rbac_permissions_assigned_by_roles_unassign_partial_update**](RbacApi.md#rbac_permissions_assigned_by_roles_unassign_partial_update) | **PATCH** /rbac/permissions/assigned_by_roles/{uuid}/unassign/ | 
[**rbac_permissions_assigned_by_users_assign_create**](RbacApi.md#rbac_permissions_assigned_by_users_assign_create) | **POST** /rbac/permissions/assigned_by_users/{id}/assign/ | 
[**rbac_permissions_assigned_by_users_list**](RbacApi.md#rbac_permissions_assigned_by_users_list) | **GET** /rbac/permissions/assigned_by_users/ | 
[**rbac_permissions_assigned_by_users_unassign_partial_update**](RbacApi.md#rbac_permissions_assigned_by_users_unassign_partial_update) | **PATCH** /rbac/permissions/assigned_by_users/{id}/unassign/ | 
[**rbac_permissions_list**](RbacApi.md#rbac_permissions_list) | **GET** /rbac/permissions/ | 
[**rbac_permissions_retrieve**](RbacApi.md#rbac_permissions_retrieve) | **GET** /rbac/permissions/{id}/ | 
[**rbac_permissions_roles_list**](RbacApi.md#rbac_permissions_roles_list) | **GET** /rbac/permissions/roles/ | 
[**rbac_permissions_users_list**](RbacApi.md#rbac_permissions_users_list) | **GET** /rbac/permissions/users/ | 
[**rbac_roles_create**](RbacApi.md#rbac_roles_create) | **POST** /rbac/roles/ | 
[**rbac_roles_destroy**](RbacApi.md#rbac_roles_destroy) | **DELETE** /rbac/roles/{uuid}/ | 
[**rbac_roles_list**](RbacApi.md#rbac_roles_list) | **GET** /rbac/roles/ | 
[**rbac_roles_partial_update**](RbacApi.md#rbac_roles_partial_update) | **PATCH** /rbac/roles/{uuid}/ | 
[**rbac_roles_retrieve**](RbacApi.md#rbac_roles_retrieve) | **GET** /rbac/roles/{uuid}/ | 
[**rbac_roles_update**](RbacApi.md#rbac_roles_update) | **PUT** /rbac/roles/{uuid}/ | 
[**rbac_roles_used_by_list**](RbacApi.md#rbac_roles_used_by_list) | **GET** /rbac/roles/{uuid}/used_by/ | 



## rbac_permissions_assigned_by_roles_assign_create

> rbac_permissions_assigned_by_roles_assign_create(uuid, permission_assign_request)


Assign permission(s) to role. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**permission_assign_request** | [**PermissionAssignRequest**](PermissionAssignRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_roles_list

> models::PaginatedRoleAssignedObjectPermissionList rbac_permissions_assigned_by_roles_list(model, object_pk, ordering, page, page_size, search)


Get assigned object permissions for a single object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | **String** | * `authentik_tenants.domain` - Domain * `authentik_crypto.certificatekeypair` - Certificate-Key Pair * `authentik_flows.flow` - Flow * `authentik_flows.flowstagebinding` - Flow Stage Binding * `authentik_outposts.dockerserviceconnection` - Docker Service-Connection * `authentik_outposts.kubernetesserviceconnection` - Kubernetes Service-Connection * `authentik_outposts.outpost` - Outpost * `authentik_policies_dummy.dummypolicy` - Dummy Policy * `authentik_policies_event_matcher.eventmatcherpolicy` - Event Matcher Policy * `authentik_policies_expiry.passwordexpirypolicy` - Password Expiry Policy * `authentik_policies_expression.expressionpolicy` - Expression Policy * `authentik_policies_password.passwordpolicy` - Password Policy * `authentik_policies_reputation.reputationpolicy` - Reputation Policy * `authentik_policies.policybinding` - Policy Binding * `authentik_providers_ldap.ldapprovider` - LDAP Provider * `authentik_providers_oauth2.scopemapping` - Scope Mapping * `authentik_providers_oauth2.oauth2provider` - OAuth2/OpenID Provider * `authentik_providers_proxy.proxyprovider` - Proxy Provider * `authentik_providers_radius.radiusprovider` - Radius Provider * `authentik_providers_saml.samlprovider` - SAML Provider * `authentik_providers_saml.samlpropertymapping` - SAML Property Mapping * `authentik_providers_scim.scimprovider` - SCIM Provider * `authentik_providers_scim.scimmapping` - SCIM Mapping * `authentik_rbac.role` - Role * `authentik_sources_ldap.ldapsource` - LDAP Source * `authentik_sources_ldap.ldappropertymapping` - LDAP Property Mapping * `authentik_sources_oauth.oauthsource` - OAuth Source * `authentik_sources_oauth.useroauthsourceconnection` - User OAuth Source Connection * `authentik_sources_plex.plexsource` - Plex Source * `authentik_sources_plex.plexsourceconnection` - User Plex Source Connection * `authentik_sources_saml.samlsource` - SAML Source * `authentik_sources_saml.usersamlsourceconnection` - User SAML Source Connection * `authentik_stages_authenticator_duo.authenticatorduostage` - Duo Authenticator Setup Stage * `authentik_stages_authenticator_duo.duodevice` - Duo Device * `authentik_stages_authenticator_sms.authenticatorsmsstage` - SMS Authenticator Setup Stage * `authentik_stages_authenticator_sms.smsdevice` - SMS Device * `authentik_stages_authenticator_static.authenticatorstaticstage` - Static Authenticator Setup Stage * `authentik_stages_authenticator_static.staticdevice` - Static Device * `authentik_stages_authenticator_totp.authenticatortotpstage` - TOTP Authenticator Setup Stage * `authentik_stages_authenticator_totp.totpdevice` - TOTP Device * `authentik_stages_authenticator_validate.authenticatorvalidatestage` - Authenticator Validation Stage * `authentik_stages_authenticator_webauthn.authenticatewebauthnstage` - WebAuthn Authenticator Setup Stage * `authentik_stages_authenticator_webauthn.webauthndevice` - WebAuthn Device * `authentik_stages_captcha.captchastage` - Captcha Stage * `authentik_stages_consent.consentstage` - Consent Stage * `authentik_stages_consent.userconsent` - User Consent * `authentik_stages_deny.denystage` - Deny Stage * `authentik_stages_dummy.dummystage` - Dummy Stage * `authentik_stages_email.emailstage` - Email Stage * `authentik_stages_identification.identificationstage` - Identification Stage * `authentik_stages_invitation.invitationstage` - Invitation Stage * `authentik_stages_invitation.invitation` - Invitation * `authentik_stages_password.passwordstage` - Password Stage * `authentik_stages_prompt.prompt` - Prompt * `authentik_stages_prompt.promptstage` - Prompt Stage * `authentik_stages_user_delete.userdeletestage` - User Delete Stage * `authentik_stages_user_login.userloginstage` - User Login Stage * `authentik_stages_user_logout.userlogoutstage` - User Logout Stage * `authentik_stages_user_write.userwritestage` - User Write Stage * `authentik_brands.brand` - Brand * `authentik_blueprints.blueprintinstance` - Blueprint Instance * `authentik_core.group` - Group * `authentik_core.user` - User * `authentik_core.application` - Application * `authentik_core.token` - Token * `authentik_enterprise.license` - License * `authentik_providers_rac.racprovider` - RAC Provider * `authentik_providers_rac.endpoint` - RAC Endpoint * `authentik_providers_rac.racpropertymapping` - RAC Property Mapping * `authentik_events.event` - Event * `authentik_events.notificationtransport` - Notification Transport * `authentik_events.notification` - Notification * `authentik_events.notificationrule` - Notification Rule * `authentik_events.notificationwebhookmapping` - Webhook Mapping | [required] |
**object_pk** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRoleAssignedObjectPermissionList**](PaginatedRoleAssignedObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_roles_unassign_partial_update

> rbac_permissions_assigned_by_roles_unassign_partial_update(uuid, patched_permission_assign_request)


Unassign permission(s) to role. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**patched_permission_assign_request** | Option<[**PatchedPermissionAssignRequest**](PatchedPermissionAssignRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_assign_create

> rbac_permissions_assigned_by_users_assign_create(id, permission_assign_request)


Assign permission(s) to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**permission_assign_request** | [**PermissionAssignRequest**](PermissionAssignRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_list

> models::PaginatedUserAssignedObjectPermissionList rbac_permissions_assigned_by_users_list(model, object_pk, ordering, page, page_size, search)


Get assigned object permissions for a single object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model** | **String** | * `authentik_tenants.domain` - Domain * `authentik_crypto.certificatekeypair` - Certificate-Key Pair * `authentik_flows.flow` - Flow * `authentik_flows.flowstagebinding` - Flow Stage Binding * `authentik_outposts.dockerserviceconnection` - Docker Service-Connection * `authentik_outposts.kubernetesserviceconnection` - Kubernetes Service-Connection * `authentik_outposts.outpost` - Outpost * `authentik_policies_dummy.dummypolicy` - Dummy Policy * `authentik_policies_event_matcher.eventmatcherpolicy` - Event Matcher Policy * `authentik_policies_expiry.passwordexpirypolicy` - Password Expiry Policy * `authentik_policies_expression.expressionpolicy` - Expression Policy * `authentik_policies_password.passwordpolicy` - Password Policy * `authentik_policies_reputation.reputationpolicy` - Reputation Policy * `authentik_policies.policybinding` - Policy Binding * `authentik_providers_ldap.ldapprovider` - LDAP Provider * `authentik_providers_oauth2.scopemapping` - Scope Mapping * `authentik_providers_oauth2.oauth2provider` - OAuth2/OpenID Provider * `authentik_providers_proxy.proxyprovider` - Proxy Provider * `authentik_providers_radius.radiusprovider` - Radius Provider * `authentik_providers_saml.samlprovider` - SAML Provider * `authentik_providers_saml.samlpropertymapping` - SAML Property Mapping * `authentik_providers_scim.scimprovider` - SCIM Provider * `authentik_providers_scim.scimmapping` - SCIM Mapping * `authentik_rbac.role` - Role * `authentik_sources_ldap.ldapsource` - LDAP Source * `authentik_sources_ldap.ldappropertymapping` - LDAP Property Mapping * `authentik_sources_oauth.oauthsource` - OAuth Source * `authentik_sources_oauth.useroauthsourceconnection` - User OAuth Source Connection * `authentik_sources_plex.plexsource` - Plex Source * `authentik_sources_plex.plexsourceconnection` - User Plex Source Connection * `authentik_sources_saml.samlsource` - SAML Source * `authentik_sources_saml.usersamlsourceconnection` - User SAML Source Connection * `authentik_stages_authenticator_duo.authenticatorduostage` - Duo Authenticator Setup Stage * `authentik_stages_authenticator_duo.duodevice` - Duo Device * `authentik_stages_authenticator_sms.authenticatorsmsstage` - SMS Authenticator Setup Stage * `authentik_stages_authenticator_sms.smsdevice` - SMS Device * `authentik_stages_authenticator_static.authenticatorstaticstage` - Static Authenticator Setup Stage * `authentik_stages_authenticator_static.staticdevice` - Static Device * `authentik_stages_authenticator_totp.authenticatortotpstage` - TOTP Authenticator Setup Stage * `authentik_stages_authenticator_totp.totpdevice` - TOTP Device * `authentik_stages_authenticator_validate.authenticatorvalidatestage` - Authenticator Validation Stage * `authentik_stages_authenticator_webauthn.authenticatewebauthnstage` - WebAuthn Authenticator Setup Stage * `authentik_stages_authenticator_webauthn.webauthndevice` - WebAuthn Device * `authentik_stages_captcha.captchastage` - Captcha Stage * `authentik_stages_consent.consentstage` - Consent Stage * `authentik_stages_consent.userconsent` - User Consent * `authentik_stages_deny.denystage` - Deny Stage * `authentik_stages_dummy.dummystage` - Dummy Stage * `authentik_stages_email.emailstage` - Email Stage * `authentik_stages_identification.identificationstage` - Identification Stage * `authentik_stages_invitation.invitationstage` - Invitation Stage * `authentik_stages_invitation.invitation` - Invitation * `authentik_stages_password.passwordstage` - Password Stage * `authentik_stages_prompt.prompt` - Prompt * `authentik_stages_prompt.promptstage` - Prompt Stage * `authentik_stages_user_delete.userdeletestage` - User Delete Stage * `authentik_stages_user_login.userloginstage` - User Login Stage * `authentik_stages_user_logout.userlogoutstage` - User Logout Stage * `authentik_stages_user_write.userwritestage` - User Write Stage * `authentik_brands.brand` - Brand * `authentik_blueprints.blueprintinstance` - Blueprint Instance * `authentik_core.group` - Group * `authentik_core.user` - User * `authentik_core.application` - Application * `authentik_core.token` - Token * `authentik_enterprise.license` - License * `authentik_providers_rac.racprovider` - RAC Provider * `authentik_providers_rac.endpoint` - RAC Endpoint * `authentik_providers_rac.racpropertymapping` - RAC Property Mapping * `authentik_events.event` - Event * `authentik_events.notificationtransport` - Notification Transport * `authentik_events.notification` - Notification * `authentik_events.notificationrule` - Notification Rule * `authentik_events.notificationwebhookmapping` - Webhook Mapping | [required] |
**object_pk** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedUserAssignedObjectPermissionList**](PaginatedUserAssignedObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_assigned_by_users_unassign_partial_update

> rbac_permissions_assigned_by_users_unassign_partial_update(id, patched_permission_assign_request)


Unassign permission(s) to user. When `object_pk` is set, the permissions are only assigned to the specific object, otherwise they are assigned globally.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this User. | [required] |
**patched_permission_assign_request** | Option<[**PatchedPermissionAssignRequest**](PatchedPermissionAssignRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_list

> models::PaginatedPermissionList rbac_permissions_list(codename, content_type__app_label, content_type__model, ordering, page, page_size, role, search, user)


Read-only list of all permissions, filterable by model and app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**codename** | Option<**String**> |  |  |
**content_type__app_label** | Option<**String**> |  |  |
**content_type__model** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**role** | Option<**String**> |  |  |
**search** | Option<**String**> | A search term. |  |
**user** | Option<**i32**> |  |  |

### Return type

[**models::PaginatedPermissionList**](PaginatedPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_retrieve

> models::Permission rbac_permissions_retrieve(id)


Read-only list of all permissions, filterable by model and app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this permission. | [required] |

### Return type

[**models::Permission**](Permission.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_roles_list

> models::PaginatedExtraRoleObjectPermissionList rbac_permissions_roles_list(uuid, ordering, page, page_size, search)


Get a role's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** |  | [required] |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedExtraRoleObjectPermissionList**](PaginatedExtraRoleObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_permissions_users_list

> models::PaginatedExtraUserObjectPermissionList rbac_permissions_users_list(user_id, ordering, page, page_size, search)


Get a users's assigned object permissions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **i32** |  | [required] |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedExtraUserObjectPermissionList**](PaginatedExtraUserObjectPermissionList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_create

> models::Role rbac_roles_create(role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_request** | [**RoleRequest**](RoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_destroy

> rbac_roles_destroy(uuid)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

 (empty response body)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_list

> models::PaginatedRoleList rbac_roles_list(group__name, ordering, page, page_size, search)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group__name** | Option<**String**> |  |  |
**ordering** | Option<**String**> | Which field to use when ordering the results. |  |
**page** | Option<**i32**> | A page number within the paginated result set. |  |
**page_size** | Option<**i32**> | Number of results to return per page. |  |
**search** | Option<**String**> | A search term. |  |

### Return type

[**models::PaginatedRoleList**](PaginatedRoleList.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_partial_update

> models::Role rbac_roles_partial_update(uuid, patched_role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**patched_role_request** | Option<[**PatchedRoleRequest**](PatchedRoleRequest.md)> |  |  |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_retrieve

> models::Role rbac_roles_retrieve(uuid)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_update

> models::Role rbac_roles_update(uuid, role_request)


Role viewset

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |
**role_request** | [**RoleRequest**](RoleRequest.md) |  | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rbac_roles_used_by_list

> Vec<models::UsedBy> rbac_roles_used_by_list(uuid)


Get a list of all objects that use this object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **uuid::Uuid** | A UUID string identifying this Role. | [required] |

### Return type

[**Vec<models::UsedBy>**](UsedBy.md)

### Authorization

[authentik](../README.md#authentik)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

