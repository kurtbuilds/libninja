use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemGetResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///Information about the last successful and failed transactions update for the Item.
    pub status: ItemStatusNullable,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/auth/get` results.
    pub options: Option<AuthGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetResponse {
    ///The `accounts` for which numbers are being retrieved.
    pub accounts: Vec<AccountBase>,
    ///An object containing identifying numbers used for making electronic transfers to and from the `accounts`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by any `accounts` for which data has been requested, the array for that type will be empty.
    pub numbers: AuthGetNumbers,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGetNumbers {
    ///An array of ACH numbers identifying accounts.
    pub ach: Vec<NumbersACH>,
    ///An array of EFT numbers identifying accounts.
    pub eft: Vec<NumbersEFT>,
    ///An array of IBAN numbers identifying accounts.
    pub international: Vec<NumbersInternational>,
    ///An array of BACS numbers identifying accounts.
    pub bacs: Vec<NumbersBACS>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///An optional object to be used with the request. If specified, `options` must not be `null`.
    pub options: Option<TransactionsGetRequestOptions>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The earliest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    ///The latest date for which data should be returned. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
    ///The number of transactions to fetch.
    pub count: Option<i64>,
    ///The number of transactions to skip. The default value is 0.
    pub offset: Option<i64>,
    ///Include the raw unparsed transaction description from the financial institution. This field is disabled by default. If you need this information in addition to the parsed data provided, contact your Plaid Account Manager.
    pub include_original_description: Option<bool>,
    ///Include the `personal_finance_category` object in the response. This feature is currently in beta – to request access, contact transactions-feedback@plaid.com.
    pub include_personal_finance_category_beta: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsGetResponse {
    ///An array containing the `accounts` associated with the Item for which transactions are being returned. Each transaction can be mapped to its corresponding account via the `account_id` field.
    pub accounts: Vec<AccountBase>,
    ///An array containing transactions from the account. Transactions are returned in reverse chronological order, with the most recent at the beginning of the array. The maximum number of transactions returned is determined by the `count` parameter.
    pub transactions: Vec<Transaction>,
    ///The total number of transactions available within the date range specified. If `total_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.
    pub total_transactions: i64,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**A list of `account_ids` to retrieve for the Item

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRecurringGetResponse {
    ///An array of depository transaction streams.
    pub inflow_streams: Vec<TransactionStream>,
    ///An array of expense transaction streams.
    pub outflow_streams: Vec<TransactionStream>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**The cursor value represents the last update requested. Providing it will cause the response to only return changes after this update.
If omitted, the entire history of updates will be returned, starting with the first-added transactions on the item.
Note: The upper-bound length of this cursor is 256 characters of base64.*/
    pub cursor: Option<String>,
    ///The number of transaction updates to fetch.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsSyncResponse {
    ///Transactions that have been added to the item since `cursor` ordered by ascending last modified time.
    pub added: Vec<Transaction>,
    ///Transactions that have been modified on the item since `cursor` ordered by ascending last modified time.
    pub modified: Vec<Transaction>,
    ///Transactions that have been removed from the item since `cursor` ordered by ascending last modified time.
    pub removed: Vec<RemovedTransaction>,
    ///Cursor used for fetching any future updates after the latest update provided in this response.
    pub next_cursor: String,
    ///Represents if more than requested count of transaction updates exist. If true, the additional updates can be fetched by making an additional request with `cursor` set to `next_cursor`.
    pub has_more: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The total number of Institutions to return.
    pub count: i64,
    ///The number of Institutions to skip.
    pub offset: i64,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. 

In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///An optional object to filter `/institutions/get` results.
    pub options: Option<InstitutionsGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetRequestOptions {
    ///Filter the Institutions based on which products they support. 
    pub products: Option<Vec<Products>>,
    ///Specify an array of routing numbers to filter institutions. The response will only return institutions that match all of the routing numbers in the array. Routing number records used for this matching are not comprehensive; failure to match a given routing number to an institution does not mean that the institution is unsupported by Plaid.
    pub routing_numbers: Option<Vec<String>>,
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    /**When `true`, return the institution's homepage URL, logo and primary brand color.

Note that Plaid does not own any of the logos shared by the API, and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetResponse {
    ///A list of Plaid institutions
    pub institutions: Vec<Institution>,
    ///The total number of institutions available via this endpoint
    pub total: i64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The search query. Institutions with names matching the query are returned
    pub query: String,
    ///Filter the Institutions based on whether they support all products listed in `products`. Provide `null` to get institutions regardless of supported products. Note that when `auth` is specified as a product, if you are enabled for Instant Match or Automated Micro-deposits, institutions that support those products will be returned even if `auth` is not present in their product array.
    pub products: Option<Vec<Products>>,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///An optional object to filter `/institutions/search` results.
    pub options: Option<InstitutionsSearchRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchRequestOptions {
    ///Limit results to institutions with or without OAuth login flows.
    pub oauth: Option<bool>,
    ///When true, return the institution's homepage URL, logo and primary brand color.
    pub include_optional_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
    ///Additional options that will be used to filter institutions by various Payment Initiation configurations.
    pub payment_initiation: InstitutionsSearchPaymentInitiationOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchPaymentInitiationOptions {
    ///A unique ID identifying the payment
    pub payment_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchResponse {
    ///An array of institutions matching the search criteria
    pub institutions: Vec<Institution>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the institution to get details about
    pub institution_id: String,
    /**Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. In API versions 2019-05-29 and earlier, the `country_codes` parameter is an optional parameter within the `options` object and will default to `[US]` if it is not supplied.
*/
    pub country_codes: Vec<CountryCode>,
    ///Specifies optional parameters for `/institutions/get_by_id`. If provided, must not be `null`.
    pub options: Option<InstitutionsGetByIdRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdRequestOptions {
    /**When `true`, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is `false`.

Note that Plaid does not own any of the logos shared by the API and that by accessing or using these logos, you agree that you are doing so at your own risk and will, if necessary, obtain all required permissions from the appropriate rights holders and adhere to any applicable usage guidelines. Plaid disclaims all express or implied warranties with respect to the logos.*/
    pub include_optional_metadata: Option<bool>,
    ///If `true`, the response will include status information about the institution. Default value is `false`.
    pub include_status: Option<bool>,
    ///When `true`, returns metadata related to the Auth product indicating which auth methods are supported.
    pub include_auth_metadata: Option<bool>,
    ///When `true`, returns metadata related to the Payment Initiation product indicating which payment configurations are supported.
    pub include_payment_initiation_metadata: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsGetByIdResponse {
    ///Details relating to a specific financial institution
    pub institution: Institution,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRemoveResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/accounts/get` results.
    pub options: Option<AccountsGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Account.
    pub account_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsGetResponse {
    /**An array of financial institution accounts associated with the Item.
If `/accounts/balance/get` was called, each account will include real-time balance information.*/
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetRequest {}
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesGetResponse {
    ///An array of all of the transaction categories used by Plaid.
    pub categories: Vec<Category>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverridePassword(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOverrideUsername(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: Option<SandboxProcessorTokenCreateRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateRequestOptions {
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: SandboxOverrideUsername,
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: SandboxOverridePassword,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxProcessorTokenCreateResponse {
    ///A processor token that can be used to call the `/processor/` endpoints.
    pub processor_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the institution the Item will be associated with
    pub institution_id: String,
    ///The products to initially pull for the Item. May be any products that the specified `institution_id`  supports. This array may not be empty.
    pub initial_products: Vec<Products>,
    ///An optional set of options to be used when configuring the Item. If specified, must not be `null`.
    pub options: Option<SandboxPublicTokenCreateRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptions {
    ///Specify a webhook to associate with the new Item.
    pub webhook: Option<String>,
    ///Test username to use for the creation of the Sandbox Item. Default value is `user_good`.
    pub override_username: SandboxOverrideUsername,
    ///Test password to use for the creation of the Sandbox Item. Default value is `pass_good`.
    pub override_password: SandboxOverridePassword,
    ///An optional set of parameters corresponding to transactions options.
    pub transactions: Option<SandboxPublicTokenCreateRequestOptionsTransactions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateRequestOptionsTransactions {
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: Option<String>,
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxPublicTokenCreateResponse {
    ///A public token that can be exchanged for an access token using `/item/public_token/exchange`
    pub public_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    /**The following values for `webhook_code` are supported:

* `DEFAULT_UPDATE`
* `NEW_ACCOUNTS_AVAILABLE`*/
    pub webhook_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemFireWebhookResponse {
    ///Value is `true`  if the test` webhook_code`  was successfully fired.
    pub webhook_fired: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequest {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///An optional object to filter `/accounts/balance/get` results.
    pub options: Option<AccountsBalanceGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountsBalanceGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item. The default value is `null`.

Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: Option<MinLastUpdatedDatetime>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MinLastUpdatedDatetime(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/identity/get` results.
    pub options: Option<IdentityGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetRequestOptions {
    /**A list of `account_ids` to retrieve for the Item.
Note: An error will be returned if a provided `account_id` is not associated with the Item.*/
    pub account_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityGetResponse {
    ///The accounts for which Identity data has been requested
    pub accounts: Vec<AccountIdentity>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorAuthGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
    ///An object containing identifying numbers used for making electronic transfers to and from the `account`. The identifying number type (ACH, EFT, IBAN, or BACS) used will depend on the country of the account. An account may have more than one number type. If a particular identifying number type is not used by the `account` for which auth data has been requested, a null value will be returned.
    pub numbers: ProcessorNumber,
    ///A single account at a financial institution.
    pub account: AccountBase,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: BankTransferIdempotencyKey,
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: Option<ACHClass>,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: BankTransferMetadata,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBankTransferCreateResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorNumber {
    ///Identifying information for transferring money to or from a US account via ACH or wire transfer.
    pub ach: NumbersACHNullable,
    ///Identifying information for transferring money to or from a Canadian bank account via EFT.
    pub eft: NumbersEFTNullable,
    ///Identifying information for transferring money to or from an international bank account via wire transfer.
    pub international: NumbersInternationalNullable,
    ///Identifying information for transferring money to or from a UK bank account via BACS.
    pub bacs: NumbersBACSNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorIdentityGetResponse {
    ///Identity information about an account
    pub account: AccountIdentity,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The processor token obtained from the Plaid integration partner. Processor tokens are in the format: `processor-<environment>-<identifier>`
    pub processor_token: ProcessorToken,
    ///An optional object to filter `/processor/balance/get` results.
    pub options: Option<ProcessorBalanceGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetRequestOptions {
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the oldest acceptable balance when making a request to `/accounts/balance/get`.

If the balance that is pulled for `ins_128026` (Capital One) is older than the given timestamp, an `INVALID_REQUEST` error with the code of `LAST_UPDATED_DATETIME_OUT_OF_RANGE` will be returned with the most recent timestamp for the requested account contained in the response.

This field is only used when the institution is `ins_128026` (Capital One), in which case a value must be provided or an `INVALID_REQUEST` error with the code of `INVALID_FIELD` will be returned. For all other institutions, this field is ignored.*/
    pub min_last_updated_datetime: Option<MinLastUpdatedDatetime>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorBalanceGetResponse {
    ///A single account at a financial institution.
    pub account: AccountBase,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The new webhook URL to associate with the Item.
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemWebhookUpdateResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemAccessTokenInvalidateResponse {
    ///The access token associated with the Item data is being requested for.
    pub new_access_token: AccessToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The key ID ( `kid` ) from the JWT header.
    pub key_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookVerificationKeyGetResponse {
    ///A JSON Web Key (JWK) that can be used in conjunction with [JWT libraries](https://jwt.io/#libraries-io) to verify Plaid webhooks
    pub key: JWKPublicKey,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JWKPublicKey {
    ///The alg member identifies the cryptographic algorithm family used with the key.
    pub alg: String,
    ///The crv member identifies the cryptographic curve used with the key.
    pub crv: String,
    ///The kid (Key ID) member can be used to match a specific key. This can be used, for instance, to choose among a set of keys within the JWK during key rollover.
    pub kid: String,
    ///The kty (key type) parameter identifies the cryptographic algorithm family used with the key, such as RSA or EC.
    pub kty: String,
    #[serde(rename = "use")]
    ///The use (public key use) parameter identifies the intended use of the public key.
    pub use_: String,
    ///The x member contains the x coordinate for the elliptic curve point.
    pub x: String,
    ///The y member contains the y coordinate for the elliptic curve point.
    pub y: String,
    ///The timestamp when the key was created, in Unix time.
    pub created_at: i64,
    ///The timestamp when the key expired, in Unix time.
    pub expired_at: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/liabilities/get` results. If provided, `options` cannot be null.
    pub options: Option<LiabilitiesGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetRequestOptions {
    /**A list of accounts to retrieve for the Item.

An error will be returned if a provided `account_id` is not associated with the Item*/
    pub account_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesGetResponse {
    ///An array of accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    ///Metadata about the Item.
    pub item: Item,
    ///An object containing liability accounts
    pub liabilities: LiabilitiesObject,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The name of the recipient
    pub name: String,
    ///The International Bank Account Number (IBAN) for the recipient. If BACS data is not provided, an IBAN is required.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: RecipientBACSNullable,
    ///The optional address of the payment recipient. This object is not currently required to make payments from UK institutions and should not be populated, though may be necessary for future European expansion.
    pub address: PaymentInitiationAddress,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientCreateResponse {
    ///A unique ID identifying the recipient
    pub recipient_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseResponse {
    ///A unique ID identifying the refund
    pub refund_id: String,
    /**The status of the refund.

`PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.

`INITIATED`: The refund has been successfully initiated.

`EXECUTED`: Indicates that the refund has been successfully executed.

`FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.*/
    pub status: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the recipient
    pub recipient_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipient {
    ///The ID of the recipient.
    pub recipient_id: String,
    ///The name of the recipient.
    pub name: String,
    ///The optional address of the payment recipient. This object is not currently required to make payments from UK institutions and should not be populated, though may be necessary for future European expansion.
    pub address: PaymentInitiationAddress,
    ///The International Bank Account Number (IBAN) for the recipient.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: RecipientBACSNullable,
    ///The EMI (E-Money Institution) recipient that this recipient is associated with, if any. This EMI recipient is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub emi_recipient_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRecipientListResponse {
    ///An array of payment recipients created for Payment Initiation
    pub recipients: Vec<PaymentInitiationRecipient>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the recipient the payment is for.
    pub recipient_id: String,
    ///A reference for the payment. This must be an alphanumeric string with at most 18 characters and must not contain any special characters (since not all institutions support them).
    pub reference: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: Option<ExternalPaymentScheduleRequest>,
    ///Additional payment options
    pub options: ExternalPaymentOptions,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentReverseRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the payment to reverse
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentCreateResponse {
    ///A unique ID identifying the payment
    pub payment_id: String,
    /**For a payment returned by this endpoint, there is only one possible value:

`PAYMENT_STATUS_INPUT_NEEDED`: The initial phase of the payment*/
    pub status: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemResetLoginResponse {
    ///`true` if the call succeeded
    pub reset_login: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    ///The verification status to set the account to.
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxItemSetVerificationStatusResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Your `public_token`, obtained from the Link `onSuccess` callback or `/sandbox/item/public_token/create`.
    pub public_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenExchangeResponse {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `item_id` value of the Item associated with the returned `access_token`
    pub item_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPublicTokenCreateResponse {
    ///A `public_token` for the particular Item corresponding to the specified `access_token`
    pub public_token: String,
    ///
    pub expiration: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentGetResponse(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPayment {
    ///The ID of the payment. Like all Plaid identifiers, the `payment_id` is case sensitive.
    pub payment_id: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub status: PaymentInitiationPaymentStatus,
    ///The ID of the recipient
    pub recipient_id: String,
    ///A reference for the payment.
    pub reference: String,
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: String,
    ///The schedule that the payment will be executed on. If a schedule is provided, the payment is automatically set up as a standing order. If no schedule is specified, the payment will be executed only once.
    pub schedule: ExternalPaymentScheduleGet,
    ///Details about external payment refund
    pub refund_details: ExternalPaymentRefundDetails,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: SenderBACSNullable,
    ///The International Bank Account Number (IBAN) for the sender, if specified in the `/payment_initiation/payment/create` call.
    pub iban: Option<String>,
    ///Initiated refunds associated with the payment.
    pub initiated_refunds: Option<Vec<PaymentInitiationRefund>>,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: PaymentScheme,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub adjusted_scheme: PaymentScheme,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationRefund {
    ///The ID of the refund. Like all Plaid identifiers, the `refund_id` is case sensitive.
    pub refund_id: String,
    ///The amount and currency of a payment
    pub amount: PaymentAmount,
    /**The status of the refund.

`PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.

`INITIATED`: The refund has been successfully initiated.

`EXECUTED`: Indicates that the refund has been successfully executed.

`FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved.*/
    pub status: String,
    ///The date and time of the last time the `status` was updated, in IS0 8601 format
    pub last_status_update: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The `payment_id` returned from `/payment_initiation/payment/create`.
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentTokenCreateResponse {
    ///A `payment_token` that can be provided to Link initialization to enter the payment initiation flow
    pub payment_token: String,
    ///The date and time at which the token will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `payment_token` expires after 15 minutes.
    pub payment_token_expiration_time: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The maximum number of payments to return. If `count` is not specified, a maximum of 10 payments will be returned, beginning with the most recent payment before the cursor (if specified).
    pub count: Option<i64>,
    ///A string in RFC 3339 format (i.e. "2019-12-06T22:35:49Z"). Only payments created before the cursor will be returned.
    pub cursor: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationPaymentListResponse {
    ///An array of payments that have been created, associated with the given `client_id`.
    pub payments: Vec<PaymentInitiationPayment>,
    ///The value that, when used as the optional `cursor` parameter to `/payment_initiation/payment/list`, will return the next unreturned payment as its first payment.
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///An array of access tokens corresponding to the Items that will be included in the report. The `assets` product must have been initialized for the Items during link; the Assets product cannot be added after initialization.
    pub access_tokens: Vec<AccessToken>,
    ///The maximum integer number of days of history to include in the Asset Report. If using Fannie Mae Day 1 Certainty, `days_requested` must be at least 61 for new originations or at least 31 for refinancings.
    pub days_requested: i64,
    ///An optional object to filter `/asset_report/create` results. If provided, must be non-`null`. The optional `user` object is required for the report to be eligible for Fannie Mae's Day 1 Certainty program.
    pub options: Option<AssetReportCreateRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateRequestOptions {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: Option<AssetReportUser>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportCreateResponse {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The `asset_report_token` returned by the original call to `/asset_report/create`
    pub asset_report_token: AssetReportRefreshAssetReportToken,
    ///The maximum number of days of history to include in the Asset Report. Must be an integer. If not specified, the value from the original call to `/asset_report/create` will be used.
    pub days_requested: Option<i64>,
    ///An optional object to filter `/asset_report/refresh` results. If provided, cannot be `null`. If not specified, the `options` from the original call to `/asset_report/create` will be used.
    pub options: Option<AssetReportRefreshRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshRequestOptions {
    ///Client-generated identifier, which can be used by lenders to track loan applications.
    pub client_report_id: Option<String>,
    ///URL to which Plaid will send Assets webhooks, for example when the requested Asset Report is ready.
    pub webhook: Option<String>,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: Option<AssetReportUser>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshResponse {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRemoveResponse {
    ///`true` if the Asset Report was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///The accounts to exclude from the Asset Report, identified by `account_id`.
    pub account_ids_to_exclude: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportFilterResponse {
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///`true` if you would like to retrieve the Asset Report with Insights, `false` otherwise. This field defaults to `false` if omitted.
    pub include_insights: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportGetResponse {
    ///An object representing an Asset Report
    pub report: AssetReport,
    ///If the Asset Report generation was successful but identity information cannot be returned, this array will contain information about the errors causing identity information to be missing
    pub warnings: Vec<Warning>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPDFGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportPDFGetResponse(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A token that can be provided to endpoints such as `/asset_report/get` or `/asset_report/pdf/get` to fetch or update an Asset Report.
    pub asset_report_token: AssetReportToken,
    ///The `auditor_id` of the third party with whom you would like to share the Asset Report.
    pub auditor_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyCreateResponse {
    ///A token that can be shared with a third party auditor to allow them to obtain access to the Asset Report. This token should be stored securely.
    pub audit_copy_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The `audit_copy_token` granting access to the Audit Copy you would like to revoke.
    pub audit_copy_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyRemoveResponse {
    ///`true` if the Audit Copy was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///An optional object to filter `/investments/holdings/get` results. If provided, must not be `null`.
    pub options: Option<InvestmentHoldingsGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentHoldingsGetRequestOptions {
    ///An array of `account_id`s to retrieve for the Item. An error will be returned if a provided `account_id` is not associated with the Item.
    pub account_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsHoldingsGetResponse {
    ///The accounts associated with the Item
    pub accounts: Vec<AccountBase>,
    ///The holdings belonging to investment accounts associated with the Item. Details of the securities in the holdings are provided in the `securities` field. 
    pub holdings: Vec<Holding>,
    ///Objects describing the securities held in the accounts associated with the Item. 
    pub securities: Vec<Security>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The earliest date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub start_date: String,
    ///The most recent date for which to fetch transaction history. Dates should be formatted as YYYY-MM-DD.
    pub end_date: String,
    ///An optional object to filter `/investments/transactions/get` results. If provided, must be non-`null`.
    pub options: Option<InvestmentsTransactionsGetRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetRequestOptions {
    ///An array of `account_ids` to retrieve for the Item.
    pub account_ids: Option<Vec<String>>,
    /**The number of transactions to fetch.
*/
    pub count: Option<i64>,
    ///The number of transactions to skip when fetching transaction history
    pub offset: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsTransactionsGetResponse {
    ///Metadata about the Item.
    pub item: Item,
    ///The accounts for which transaction history is being fetched.
    pub accounts: Vec<AccountBase>,
    ///All securities for which there is a corresponding transaction being fetched.
    pub securities: Vec<Security>,
    ///The transactions being fetched
    pub investment_transactions: Vec<InvestmentTransaction>,
    ///The total number of transactions available within the date range specified. If `total_investment_transactions` is larger than the size of the `transactions` array, more transactions are available and can be fetched via manipulating the `offset` parameter.'
    pub total_investment_transactions: i64,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
    ///The processor you are integrating with.
    pub processor: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorTokenCreateResponse {
    ///The `processor_token` that can then be used by the Plaid partner to make API requests
    pub processor_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorStripeBankAccountTokenCreateResponse {
    ///A token that can be sent to Stripe for use in making API calls to Plaid
    pub stripe_bank_account_token: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorApexProcessorTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` value obtained from the `onSuccess` callback in Link
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Access token for the target Item, typically provided in the Import Item response. 
    pub target_access_token: String,
    ///Plaid Account ID that specifies the target bank account. This account will become the recipient for a user's direct deposit.
    pub target_account_id: String,
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: Option<DepositSwitchCreateRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateRequestOptions {
    /**The URL registered to receive webhooks when the status of a deposit switch request has changed.
*/
    pub webhook: Option<String>,
    ///An array of access tokens corresponding to transaction items to use when attempting to match the user to their Payroll Provider. These tokens must be created by the same client id as the one creating the switch, and have access to the transactions product.
    pub transaction_item_access_tokens: Option<Vec<AccessToken>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchCreateResponse {
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Array of product strings
    pub products: Vec<Products>,
    ///Object of user ID and auth token pair, permitting Plaid to aggregate a user’s accounts
    pub user_auth: ItemImportRequestUserAuth,
    ///An optional object to configure `/item/import` request.
    pub options: Option<ItemImportRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestOptions {
    /**Specifies a webhook URL to associate with an Item. Plaid fires a webhook if credentials fail.
*/
    pub webhook: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportRequestUserAuth {
    ///Opaque user identifier
    pub user_id: String,
    ///Authorization token Plaid will use to aggregate this user’s accounts
    pub auth_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemImportResponse {
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTokenCreateResponse {
    ///Deposit switch token, used to initialize Link for the Deposit Switch product
    pub deposit_switch_token: String,
    ///Expiration time of the token, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub deposit_switch_token_expiration_time: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///A `link_token` from a previous invocation of `/link/token/create`
    pub link_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The name of your application, as it should be displayed in Link. Maximum length of 30 characters. If a value longer than 30 characters is provided, Link will display "This Application" instead.
    pub client_name: String,
    /**The language that Link should be displayed in.

Supported languages are:
- English (`'en'`)
- French (`'fr'`)
- Spanish (`'es'`)
- Dutch (`'nl'`)
- German(`'de'`)

When using a Link customization, the language configured here must match the setting in the customization, or the customization will not be applied.*/
    pub language: String,
    /**Specify an array of Plaid-supported country codes using the ISO-3166-1 alpha-2 country code standard. Institutions from all listed countries will be shown.  Supported country codes are: `US`, `CA`, `DE`, `ES`, `FR`, `GB`, `IE`, `NL`. For a complete mapping of supported products by country, see https://plaid.com/global/.

If Link is launched with multiple country codes, only products that you are enabled for in all countries will be used by Link. Note that while all countries are enabled by default in Sandbox and Development, in Production only US and Canada are enabled by default. To gain access to European institutions in the Production environment, [file a product access Support ticket](https://dashboard.plaid.com/support/new/product-and-development/product-troubleshooting/request-product-access) via the Plaid dashboard. If you initialize with a European country code, your users will see the European consent panel during the Link flow.

If using a Link customization, make sure the country codes in the customization match those specified in `country_codes`. If both `country_codes` and a Link customization are used, the value in `country_codes` may override the value in the customization.

If using the Auth features Instant Match, Same-day Micro-deposits, or Automated Micro-deposits, `country_codes` must be set to `['US']`.*/
    pub country_codes: Vec<CountryCode>,
    ///An object specifying information about the end user who will be linking their account.
    pub user: LinkTokenCreateRequestUser,
    /**List of Plaid product(s) you wish to use. If launching Link in update mode, should be omitted; required otherwise.

`balance` is *not* a valid value, the Balance product does not require explicit initialization and will automatically be initialized when any other product is initialized.

Only institutions that support *all* requested products will be shown in Link; to maximize the number of institutions listed, it is recommended to initialize Link with the minimal product set required for your use case. Additional products can be added after Link initialization by calling the relevant endpoints. For details and exceptions, see [Choosing when to initialize products](https://plaid.com/docs/link/best-practices/#choosing-when-to-initialize-products).

Note that, unless you have opted to disable Instant Match support, institutions that support Instant Match will also be shown in Link if `auth` is specified as a product, even though these institutions do not contain `auth` in their product array.

In Production, you will be billed for each product that you specify when initializing Link. Note that a product cannot be removed from an Item once the Item has been initialized with that product. To stop billing on an Item for subscription-based products, such as Liabilities, Investments, and Transactions, remove the Item via `/item/remove`.*/
    pub products: Option<Vec<Products>>,
    ///The destination URL to which any webhooks should be sent.
    pub webhook: Option<String>,
    ///The `access_token` associated with the Item to update, used when updating or modifying an existing `access_token`. Used when launching Link in update mode, when completing the Same-day (manual) Micro-deposit flow, or (optionally) when initializing Link as part of the Payment Initiation (UK and Europe) flow.
    pub access_token: Option<String>,
    ///The name of the Link customization from the Plaid Dashboard to be applied to Link. If not specified, the `default` customization will be used. When using a Link customization, the language in the customization must match the language selected via the `language` parameter, and the countries in the customization should match the country codes selected via `country_codes`.
    pub link_customization_name: Option<String>,
    ///A URI indicating the destination where a user should be forwarded after completing the Link flow; used to support OAuth authentication flows when launching Link in the browser or via a webview. The `redirect_uri` should not contain any query parameters. When used in Production or Development, must be an https URI. To specify any subdomain, use `*` as a wildcard character, e.g. `https://*.example.com/oauth.html`. If `android_package_name` is specified, this field should be left blank.  Note that any redirect URI must also be added to the Allowed redirect URIs list in the [developer dashboard](https://dashboard.plaid.com/team/api).
    pub redirect_uri: Option<String>,
    ///The name of your app's Android package. Required if using the `link_token` to initialize Link on Android. When creating a `link_token` for initializing Link on other platforms, this field must be left blank. Any package name specified here must also be added to the Allowed Android package names setting on the [developer dashboard](https://dashboard.plaid.com/team/api). 
    pub android_package_name: Option<String>,
    /**By default, Link will provide limited account filtering: it will only display Institutions that are compatible with all products supplied in the `products` parameter of `/link/token/create`, and, if `auth` is specified in the `products` array, will also filter out accounts other than `checking` and `savings` accounts on the Account Select pane. You can further limit the accounts shown in Link by using `account_filters` to specify the account subtypes to be shown in Link. Only the specified subtypes will be shown. This filtering applies to both the Account Select view (if enabled) and the Institution Select view. Institutions that do not support the selected subtypes will be omitted from Link. To indicate that all subtypes should be shown, use the value `"all"`. If the `account_filters` filter is used, any account type for which a filter is not specified will be entirely omitted from Link. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema).

For institutions using OAuth, the filter will not affect the list of accounts shown by the bank in the OAuth window.
*/
    pub account_filters: Option<LinkTokenAccountFilters>,
    ///Configuration parameters for EU flows
    pub eu_config: Option<LinkTokenEUConfig>,
    ///Used for certain Europe-only configurations, as well as certain legacy use cases in other regions.
    pub institution_id: Option<String>,
    ///Specifies options for initializing Link for use with the Payment Initiation (Europe) product. This field is required if `payment_initiation` is included in the `products` array.
    pub payment_initiation: Option<LinkTokenCreateRequestPaymentInitiation>,
    ///Specifies options for initializing Link for use with the Deposit Switch (beta) product. This field is required if `deposit_switch` is included in the `products` array.
    pub deposit_switch: Option<LinkTokenCreateRequestDepositSwitch>,
    ///Specifies options for initializing Link for use with the Income (beta) product. This field is required if `income_verification` is included in the `products` array.
    pub income_verification: Option<LinkTokenCreateRequestIncomeVerification>,
    ///Specifies options for initializing Link for use with the Auth product. This field is currently only required if using the Flexible Auth product (currently in closed beta).
    pub auth: Option<LinkTokenCreateRequestAuth>,
    ///Specifies options for initializing Link for use with the Transfer product.
    pub transfer: Option<LinkTokenCreateRequestTransfer>,
    ///Specifies options for initializing Link for [update mode](https://plaid.com/docs/link/update-mode).
    pub update: Option<LinkTokenCreateRequestUpdate>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenAccountFilters {
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<DepositoryFilter>,
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<CreditFilter>,
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LoanFilter>,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type acconunts for API versions 2018-05-22 and earlier).
    pub investment: Option<InvestmentFilter>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenEUConfig {
    ///If `true`, open Link without an initial UI. Defaults to `false`.
    pub headless: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestPaymentInitiation {
    ///The `payment_id` provided by the `/payment_initiation/payment/create` endpoint.
    pub payment_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestDepositSwitch {
    ///The `deposit_switch_id` provided by the `/deposit_switch/create` endpoint.
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestTransfer {
    ///The `id` returned by the `/transfer/intent/create` endpoint.
    pub intent_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAuth {
    ///The optional Auth flow to use. Currently only used to enable Flexible Auth.
    pub flow_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUser {
    ///A unique ID representing the end user. Typically this will be a user ID number from your application. Personally identifiable information, such as an email address or phone number, should not be used in the `client_user_id`. It is currently used as a means of searching logs for the given user in the Plaid Dashboard.
    pub client_user_id: String,
    ///The user's full legal name. This is an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user) to associate Items to the user.
    pub legal_name: Option<String>,
    ///The user's phone number in [E.164](https://en.wikipedia.org/wiki/E.164) format. This field is optional, but required to enable the [returning user experience](https://plaid.com/docs/link/returning-user).
    pub phone_number: Option<String>,
    /**The date and time the phone number was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This field is optional, but required to enable any [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for a phone number that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`
*/
    pub phone_number_verified_time: Option<String>,
    ///The user's email address. This field is optional, but required to enable the [pre-authenticated returning user flow](https://plaid.com/docs/link/returning-user/#enabling-the-returning-user-experience).
    pub email_address: Option<String>,
    /**The date and time the email address was verified in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDThh:mm:ssZ`). This is an optional field used in the [returning user experience](https://plaid.com/docs/link/returning-user).

 Only pass a verification time for an email address that you have verified. If you have performed verification but don’t have the time, you may supply a signal value of the start of the UNIX epoch.

 Example: `2020-01-01T00:00:00Z`*/
    pub email_address_verified_time: Option<String>,
    ///To be provided in the format "ddd-dd-dddd". This field is optional and will support not-yet-implemented functionality for new products.
    pub ssn: Option<String>,
    ///To be provided in the format "yyyy-mm-dd". This field is optional and will support not-yet-implemented functionality for new products.
    pub date_of_birth: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestUpdate {
    ///If `true`, enables [update mode with Account Select](https://plaid.com/docs/link/update-mode/#using-update-mode-to-request-new-accounts).
    pub account_selection_enabled: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestAccountSubtypes {
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<LinkTokenCreateDepositoryFilter>,
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<LinkTokenCreateCreditFilter>,
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LinkTokenCreateLoanFilter>,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier).
    pub investment: Option<LinkTokenCreateInvestmentFilter>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateDepositoryFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: Option<DepositoryAccountSubtypes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateCreditFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: Option<CreditAccountSubtypes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateLoanFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: Option<LoanAccountSubtypes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateInvestmentFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: Option<InvestmentAccountSubtypes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetResponse {
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///The creation timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: Option<String>,
    ///The expiration timestamp for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub expiration: Option<String>,
    ///An object specifying the arguments originally provided to the `/link/token/create` call.
    pub metadata: LinkTokenGetMetadataResponse,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenGetMetadataResponse {
    ///The `products` specified in the `/link/token/create` call.
    pub initial_products: Vec<Products>,
    ///The `webhook` specified in the `/link/token/create` call.
    pub webhook: Option<String>,
    ///The `country_codes` specified in the `/link/token/create` call.
    pub country_codes: Vec<CountryCode>,
    ///The `language` specified in the `/link/token/create` call.
    pub language: Option<String>,
    /**The `account_filters` specified in the original call to `/link/token/create`.
*/
    pub account_filters: Option<AccountFiltersResponse>,
    ///The `redirect_uri` specified in the `/link/token/create` call.
    pub redirect_uri: Option<String>,
    ///The `client_name` specified in the `/link/token/create` call.
    pub client_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateResponse {
    ///A `link_token`, which can be supplied to Link in order to initialize it and receive a `public_token`, which can be exchanged for an `access_token`.
    pub link_token: String,
    ///The expiration date for the `link_token`, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format. A `link_token` created to generate a `public_token` that will be exchanged for a new `access_token` expires after 4 hours. A `link_token` created for an existing Item (such as when updating an existing `access_token` by launching Link in update mode) expires after 30 minutes.
    pub expiration: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    pub item_id: String,
    ///The Plaid Institution ID associated with the Item. Field is `null` for Items created via Same Day Micro-deposits.
    pub institution_id: Option<String>,
    ///The URL registered to receive webhooks for the Item.
    pub webhook: Option<String>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Error,
    ///A list of products available for the Item that have not yet been accessed.
    pub available_products: Vec<Products>,
    /**A list of products that have been billed for the Item. Note - `billed_products` is populated in all environments but only requests in Production are billed.
*/
    pub billed_products: Vec<Products>,
    /**A list of authorized products for the Item.
*/
    pub products: Option<Vec<Products>>,
    /**The RFC 3339 timestamp after which the consent provided by the end user will expire. Upon consent expiration, the item will enter the `ITEM_LOGIN_REQUIRED` error state. To circumvent the `ITEM_LOGIN_REQUIRED` error and maintain continuous consent, the end user can reauthenticate via Link’s update mode in advance of the consent expiration time.

Note - This is only relevant for certain OAuth-based institutions. For all other institutions, this field will be null.
*/
    pub consent_expiration_time: Option<String>,
    /**Indicates whether an Item requires user interaction to be updated, which can be the case for Items with some forms of two-factor authentication.

`background` - Item can be updated in the background

`user_present_required` - Item requires user interaction to be updated*/
    pub update_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlaidError(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    ///A broad categorization of the error. Safe for programmatic use.
    pub error_type: String,
    ///The particular error code. Safe for programmatic use.
    pub error_code: String,
    ///A developer-friendly representation of the error code. This may change over time and is not safe for programmatic use.
    pub error_message: String,
    /**A user-friendly representation of the error code. `null` if the error is not related to user action.

This may change over time and is not safe for programmatic use.*/
    pub display_message: Option<String>,
    ///A unique ID identifying the request, to be used for troubleshooting purposes. This field will be omitted in errors provided by webhooks.
    pub request_id: Option<String>,
    /**In the Assets product, a request can pertain to more than one Item. If an error is returned for such a request, `causes` will return an array of errors containing a breakdown of these errors on the individual Item level, if any can be identified.

`causes` will only be provided for the `error_type` `ASSET_REPORT_ERROR`. `causes` will also not be populated inside an error nested within a `warning` object.*/
    pub causes: Option<Vec<serde_json::Value>>,
    ///The HTTP status code associated with the error. This will only be returned in the response body when the error information is provided via a webhook.
    pub status: Option<f64>,
    ///The URL of a Plaid documentation page with more information about the error
    pub documentation_url: Option<String>,
    ///Suggested steps for resolving the error
    pub suggested_action: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusTransactions {
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful transactions update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed transactions update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusInvestments {
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last successful investments update for the Item. The status will update each time Plaid successfully connects with the institution, regardless of whether any new data is available in the update.
    pub last_successful_update: Option<String>,
    ///[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of the last failed investments update for the Item. The status will update each time Plaid fails an attempt to connect with the institution, regardless of whether any new data is available in the update.
    pub last_failed_update: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatusLastWebhook {
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) timestamp of when the webhook was fired.
*/
    pub sent_at: Option<String>,
    ///The last webhook code sent.
    pub code_sent: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemStatus {
    ///Information about the last successful and failed investments update for the Item.
    pub investments: ItemStatusInvestments,
    ///Information about the last successful and failed transactions update for the Item.
    pub transactions: ItemStatusTransactions,
    ///Information about the last webhook fired for the Item.
    pub last_webhook: ItemStatusLastWebhook,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct OverrideAccountType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBase {
    /**Plaid’s unique identifier for the account. This value will not change unless Plaid can't reconcile the account with the data returned by the financial institution. This may occur, for example, when the name of the account changes. If this happens a new `account_id` will be assigned to the account.

The `account_id` can also change if the `access_token` is deleted and the same credentials that were used to generate that `access_token` are used to generate a new `access_token` on a later date. In that case, the new `account_id` will be different from the old `account_id`.

If an account with a specific `account_id` disappears instead of changing, the account is likely closed. Closed accounts are not returned by the Plaid API.

Like all Plaid identifiers, the `account_id` is case sensitive.*/
    pub account_id: String,
    ///A set of fields describing the balance for an account. Balance information may be cached unless the balance object was returned by `/accounts/balance/get`.
    pub balances: AccountBalance,
    ///The last 2-4 alphanumeric characters of an account's official account number. Note that the mask may be non-unique between an Item's accounts, and it may also not match the mask that the bank displays to the user.
    pub mask: Option<String>,
    ///The name of the account, either assigned by the user or by the financial institution itself
    pub name: String,
    ///The official name of the account as given by the financial institution
    pub official_name: Option<String>,
    #[serde(rename = "type")]
    /**`investment:` Investment account. In API versions 2018-05-22 and earlier, this type is called `brokerage` instead.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`brokerage`: An investment account. Used for `/assets/` endpoints only; other endpoints use `investment`.

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    pub type_: AccountType,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: AccountSubtype,
    /**The current verification status of an Auth Item initiated through Automated or Manual micro-deposits.  Returned for Auth Items only.

`pending_automatic_verification`: The Item is pending automatic verification

`pending_manual_verification`: The Item is pending manual micro-deposit verification. Items remain in this state until the user successfully verifies the two amounts.

`automatically_verified`: The Item has successfully been automatically verified	

`manually_verified`: The Item has successfully been manually verified

`verification_expired`: Plaid was unable to automatically verify the deposit within 7 calendar days and will no longer attempt to validate the Item. Users may retry by submitting their information again through Link.

`verification_failed`: The Item failed manual micro-deposit verification because the user exhausted all 3 verification attempts. Users may retry by submitting their information again through Link.	
	*/
    pub verification_status: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
    /**The amount of funds available to be withdrawn from the account, as determined by the financial institution.

For `credit`-type accounts, the `available` balance typically equals the `limit` less the `current` balance, less any pending outflows plus any pending inflows.

For `depository`-type accounts, the `available` balance typically equals the `current` balance less any pending outflows plus any pending inflows. For `depository`-type accounts, the `available` balance does not include the overdraft limit.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the `available` balance is the total cash available to withdraw as presented by the institution.

Note that not all institutions calculate the `available`  balance. In the event that `available` balance is unavailable, Plaid will return an `available` balance value of `null`.

Available balance may be cached and is not guaranteed to be up-to-date in realtime unless the value was returned by `/accounts/balance/get`.

If `current` is `null` this field is guaranteed not to be `null`.*/
    pub available: Option<f64>,
    /**The total amount of funds in or owed by the account.

For `credit`-type accounts, a positive balance indicates the amount owed; a negative amount indicates the lender owing the account holder.

For `loan`-type accounts, the current balance is the principal remaining on the loan, except in the case of student loan accounts at Sallie Mae (`ins_116944`). For Sallie Mae student loans, the account's balance includes both principal and any outstanding interest.

For `investment`-type accounts (or `brokerage`-type accounts for API versions 2018-05-22 and earlier), the current balance is the total value of assets as presented by the institution.

Note that balance information may be cached unless the value was returned by `/accounts/balance/get`; if the Item is enabled for Transactions, the balance will be at least as recent as the most recent Transaction update. If you require realtime balance information, use the `available` balance as provided by `/accounts/balance/get`.

When returned by `/accounts/balance/get`, this field may be `null`. When this happens, `available` is guaranteed not to be `null`.*/
    pub current: Option<f64>,
    /**For `credit`-type accounts, this represents the credit limit.

For `depository`-type accounts, this represents the pre-arranged overdraft limit, which is common for current (checking) accounts in Europe.

In North America, this field is typically only available for `credit`-type accounts.*/
    pub limit: Option<f64>,
    ///The ISO-4217 currency code of the balance. Always null if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the balance. Always null if `iso_currency_code` is non-null. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `unofficial_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    /**Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (`YYYY-MM-DDTHH:mm:ssZ`) indicating the last time that the balance for the given account has been updated

This is currently only provided when the `min_last_updated_datetime` is passed when calling `/accounts/balance/get` for `ins_128026` (Capital One).*/
    pub last_updated_datetime: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSubtype(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersACH {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    /**The ACH account number for the account.

Note that when using OAuth with Chase Bank (`ins_56`), Chase will issue "tokenized" routing and account numbers, which are not the user's actual account and routing numbers. These tokenized numbers should work identically to normal account and routing numbers. The digits returned in the `mask` field will continue to reflect the actual account number, rather than the tokenized account number; for this reason, when displaying account numbers to the user to help them identify their account in your UI, always use the `mask` rather than truncating the `account` number. If a user revokes their permissions to your app, the tokenized numbers will continue to work for ACH deposits, but not withdrawals.*/
    pub account: String,
    ///The ACH routing number for the account. If the institution is `ins_56`, this may be a tokenized routing number. For more information, see the description of the `account` field.
    pub routing: String,
    ///The wire transfer routing number for the account, if available
    pub wire_routing: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersACHNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEFT {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The EFT account number for the account
    pub account: String,
    ///The EFT institution number for the account
    pub institution: String,
    ///The EFT branch number for the account
    pub branch: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersEFTNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternational {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The International Bank Account Number (IBAN) for the account
    pub iban: String,
    ///The Bank Identifier Code (BIC) for the account
    pub bic: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersInternationalNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBACS {
    ///The Plaid account ID associated with the account numbers
    pub account_id: String,
    ///The BACS account number for the account
    pub account: String,
    ///The BACS sort code for the account
    pub sort_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumbersBACSNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBACS {
    ///The account number of the account. Maximum of 10 characters.
    pub account: Option<String>,
    ///The 6-character sort code of the account.
    pub sort_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecipientBACSNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SenderBACSNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationOptionalRestrictionBacs(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct RemovedTransaction {
    ///The ID of the removed transaction.
    pub transaction_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionBase {
    /**Please use the `payment_channel` field, `transaction_type` will be deprecated in the future.

`digital:` transactions that took place online.

`place:` transactions that were made at a physical location.

`special:` transactions that relate to banks, e.g. fees or deposits.

`unresolved:` transactions that do not fit into the other three types.
*/
    pub transaction_type: Option<String>,
    ///The ID of a posted transaction's associated pending transaction, where applicable.
    pub pending_transaction_id: Option<String>,
    /**The ID of the category to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category_id: Option<String>,
    /**A hierarchical array of the categories to which this transaction belongs. For a full list of categories, see [`/categories/get`](https://plaid.com/docs/api/products/#categoriesget).

If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub category: Option<Vec<String>>,
    ///A representation of where a transaction took place
    pub location: Option<Location>,
    /**Transaction information specific to inter-bank transfers. If the transaction was not an inter-bank transfer, all fields will be `null`.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, the `payment_meta` key will always appear, but no data elements are guaranteed. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub payment_meta: Option<PaymentMeta>,
    ///The name of the account owner. This field is not typically populated and only relevant when dealing with sub-accounts.
    pub account_owner: Option<String>,
    /**The merchant name or transaction description.

If the `transactions` object was returned by a Transactions endpoint such as `/transactions/get`, this field will always appear. If the `transactions` object was returned by an Assets endpoint such as `/asset_report/get/` or `/asset_report/pdf/get`, this field will only appear in an Asset Report with Insights.*/
    pub name: Option<String>,
    ///The string returned by the financial institution to describe the transaction. For transactions returned by `/transactions/get`, this field is in beta and will be omitted unless the client is both enrolled in the closed beta program and has set `options.include_original_description` to `true`.
    pub original_description: Option<String>,
    ///The ID of the account in which this transaction occurred.
    pub account_id: String,
    ///The settled value of the transaction, denominated in the account's currency, as stated in `iso_currency_code` or `unofficial_currency_code`. Positive values when money moves out of the account; negative values when money moves in. For example, debit card purchases are positive; credit card payments, direct deposits, and refunds are negative.
    pub amount: f64,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the transaction. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///For pending transactions, the date that the transaction occurred; for posted transactions, the date that the transaction posted. Both dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ( `YYYY-MM-DD` ).
    pub date: String,
    ///When `true`, identifies the transaction as pending or unsettled. Pending transaction details (name, type, amount, category ID) may change before they are settled.
    pub pending: bool,
    ///The unique ID of the transaction. Like all Plaid identifiers, the `transaction_id` is case sensitive.
    pub transaction_id: String,
    ///The merchant name, as extracted by Plaid from the `name` field.
    pub merchant_name: Option<String>,
    ///The check number of the transaction. This field is only populated for check transactions.
    pub check_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    ///The street address where the transaction occurred.
    pub address: Option<String>,
    ///The city where the transaction occurred.
    pub city: Option<String>,
    ///The region or state where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `state`.
    pub region: Option<String>,
    ///The postal code where the transaction occurred. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code where the transaction occurred.
    pub country: Option<String>,
    ///The latitude where the transaction occurred.
    pub lat: Option<f64>,
    ///The longitude where the transaction occurred.
    pub lon: Option<f64>,
    ///The merchant defined store number where the transaction occurred.
    pub store_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStream {
    ///The ID of the account to which the stream belongs
    pub account_id: String,
    ///A unique id for the stream
    pub stream_id: String,
    ///The ID of the category to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category_id: String,
    ///A hierarchical array of the categories to which this transaction belongs. See [Categories](https://plaid.com/docs/#category-overview).
    pub category: Vec<String>,
    ///A description of the transaction stream.
    pub description: String,
    ///The posted date of the earliest transaction in the stream.
    pub first_date: String,
    ///The posted date of the latest transaction in the stream.
    pub last_date: String,
    ///describes the frequency of the transaction stream.
    pub frequency: RecurringTransactionFrequency,
    ///An array of Plaid transaction IDs belonging to the stream, sorted by posted date.
    pub transaction_ids: Vec<String>,
    ///Object with data pertaining to an amount on the transaction stream.
    pub average_amount: TransactionStreamAmount,
    ///indicates whether the transaction stream is still live.
    pub is_active: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionStreamAmount {
    ///represents the numerical value of an amount.
    pub amount: Option<f64>,
    /**The ISO-4217 currency code of the amount. Always `null` if `unofficial_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub iso_currency_code: Option<String>,
    ///The unofficial currency code of the amount. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringTransactionFrequency(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Institution {
    ///Unique identifier for the institution
    pub institution_id: String,
    ///The official name of the institution
    pub name: String,
    ///A list of the Plaid products supported by the institution. Note that only institutions that support Instant Auth will return `auth` in the product array; institutions that do not list `auth` may still support other Auth methods such as Instant Match or Automated Micro-deposit Verification. To identify institutions that support those methods, use the `auth_metadata` object. For more details, see [Full Auth coverage](https://plaid.com/docs/auth/coverage/).
    pub products: Vec<Products>,
    ///A list of the country codes supported by the institution.
    pub country_codes: Vec<CountryCode>,
    ///The URL for the institution's website
    pub url: Option<String>,
    ///Hexadecimal representation of the primary color used by the institution
    pub primary_color: Option<String>,
    ///Base64 encoded representation of the institution's logo
    pub logo: Option<String>,
    ///A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    pub routing_numbers: Vec<String>,
    ///Indicates that the institution has an OAuth login flow.
    pub oauth: bool,
    /**The status of an institution is determined by the health of its Item logins, Transactions updates, Investments updates, Liabilities updates, Auth requests, Balance requests, Identity requests, Investments requests, and Liabilities requests. A login attempt is conducted during the initial Item add in Link. If there is not enough traffic to accurately calculate an institution's status, Plaid will return null rather than potentially inaccurate data.

Institution status is accessible in the Dashboard and via the API using the `/institutions/get_by_id` endpoint with the `include_status` option set to true. Note that institution status is not available in the Sandbox environment.
*/
    pub status: InstitutionStatus,
    ///Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    pub payment_initiation_metadata: PaymentInitiationMetadata,
    ///Metadata that captures information about the Auth features of an institution.
    pub auth_metadata: AuthMetadata,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionStatus {
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub item_logins: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub transactions_updates: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub auth: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub identity: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments_updates: ProductStatus,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities_updates: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub liabilities: Option<ProductStatus>,
    ///A representation of the status health of a request type. Auth requests, Balance requests, Identity requests, Investments requests, Liabilities requests, Transactions updates, Investments updates, Liabilities updates, and Item logins each have their own status object.
    pub investments: Option<ProductStatus>,
    ///Details of recent health incidents associated with the institution.
    pub health_incidents: Option<Vec<HealthIncident>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CountryCode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMeta {
    ///The transaction reference number supplied by the financial institution.
    pub reference_number: Option<String>,
    ///The ACH PPD ID for the payer.
    pub ppd_id: Option<String>,
    ///For transfers, the party that is receiving the transaction.
    pub payee: Option<String>,
    ///The party initiating a wire transfer. Will be `null` if the transaction is not a wire transfer.
    pub by_order_of: Option<String>,
    ///For transfers, the party that is paying the transaction.
    pub payer: Option<String>,
    ///The type of transfer, e.g. 'ACH'
    pub payment_method: Option<String>,
    ///The name of the payment processor
    pub payment_processor: Option<String>,
    ///The payer-supplied description of the transfer.
    pub reason: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCode(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    ///An identifying number for the category. `category_id` is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    pub category_id: String,
    ///`place` for physical transactions or `special` for other transactions such as bank charges.
    pub group: String,
    ///A hierarchical array of the categories to which this `category_id` belongs.
    pub hierarchy: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalFinanceCategory {
    ///A high level category that communicates the broad category of the transaction.
    pub primary: String,
    ///Provides additional granularity to the primary categorization.
    pub detailed: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenNullable(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferAccessToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct APISecret(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct APIClientID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionsRemovedWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`TRANSACTIONS_REMOVED`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///An array of `transaction_ids` corresponding to the removed transactions
    pub removed_transactions: Vec<String>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new transactions detected since the last time this webhook was fired.
    pub new_transactions: f64,
    ///The `item_id` of the Item the webhook relates to.
    pub item_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`HISTORICAL_UPDATE`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new, unfetched transactions available
    pub new_transactions: f64,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InitialUpdateWebhook {
    ///`TRANSACTIONS`
    pub webhook_type: String,
    ///`INITIAL_UPDATE`
    pub webhook_code: String,
    ///The error code associated with the webhook.
    pub error: Option<String>,
    ///The number of new, unfetched transactions available.
    pub new_transactions: f64,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    ///The phone number.
    pub data: String,
    ///When `true`, identifies the phone number as the primary number on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of phone number.
    pub type_: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Email {
    ///The email address.
    pub data: String,
    ///When `true`, identifies the email address as the primary email on an account.
    pub primary: bool,
    #[serde(rename = "type")]
    ///The type of email account as described by the financial institution.
    pub type_: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    ///Data about the components comprising an address.
    pub data: AddressData,
    ///When `true`, identifies the address as the primary address on an account.
    pub primary: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressDataNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressData {
    ///The full city name
    pub city: String,
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBalance {
    ///The date of the calculated historical balance, in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD)
    pub date: String,
    /**The total amount of funds in the account, calculated from the `current` balance in the `balance` object by subtracting inflows and adding back outflows according to the posted date of each transaction.

If the account has any pending transactions, historical balance amounts on or after the date of the earliest pending transaction may differ if retrieved in subsequent Asset Reports as a result of those pending transactions posting.*/
    pub current: f64,
    ///The ISO-4217 currency code of the balance. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the balance. Always `null` if `iso_currency_code` is non-`null`.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    /**A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. If the name of a business is reported, please contact Plaid Support. In the case of a joint account, Plaid will make a best effort to report the names of all account holders.

If an Item contains multiple accounts with different owner names, some institutions will report all names associated with the Item in each account's `names` array.*/
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub phone_numbers: Vec<PhoneNumber>,
    ///A list of email addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub emails: Vec<Email>,
    ///Data about the various addresses associated with the account by the financial institution. May be an empty array if no relevant information is returned from the financial institution.
    pub addresses: Vec<Address>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerOverride {
    ///A list of names associated with the account by the financial institution. These should always be the names of individuals, even for business accounts. Note that the same name data will be used for all accounts associated with an Item.
    pub names: Vec<String>,
    ///A list of phone numbers associated with the account.
    pub phone_numbers: Vec<PhoneNumber>,
    ///A list of email addresses associated with the account.
    pub emails: Vec<Email>,
    ///Data about the various addresses associated with the account.
    pub addresses: Vec<Address>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesObject {
    ///The credit accounts returned.
    pub credit: Option<Vec<CreditCardLiability>>,
    ///The mortgage accounts returned.
    pub mortgage: Option<Vec<MortgageLiability>>,
    ///The student loan accounts returned.
    pub student: Option<Vec<StudentLoan>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoan {
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    ///The account number of the loan. For some institutions, this may be a masked version of the number (e.g., the last 4 digits instead of the entire number).
    pub account_number: Option<String>,
    ///The dates on which loaned funds were disbursed or will be disbursed. These are often in the past. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub disbursement_dates: Option<Vec<String>>,
    ///The date when the student loan is expected to be paid off. Availability for this field is limited. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub expected_payoff_date: Option<String>,
    ///The guarantor of the student loan.
    pub guarantor: Option<String>,
    ///The interest rate on the loan as a percentage.
    pub interest_rate_percentage: f64,
    ///`true` if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: Option<String>,
    ///The type of loan, e.g., "Consolidation Loans".
    pub loan_name: Option<String>,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    /**The minimum payment due for the next billing cycle. There are some exceptions:
Some institutions require a minimum payment across all loans associated with an account number. Our API presents that same minimum payment amount on each loan. The institutions that do this are: Great Lakes ( `ins_116861`), Firstmark (`ins_116295`), Commonbond Firstmark Services (`ins_116950`), Nelnet (`ins_116528`), EdFinancial Services (`ins_116304`), Granite State (`ins_116308`), and Oklahoma Student Loan Authority (`ins_116945`).
Firstmark (`ins_116295` ) and Navient (`ins_116248`) will display as $0 if there is an autopay program in effect.*/
    pub minimum_payment_amount: Option<f64>,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. A payment is not expected if `loan_status.type` is `deferment`, `in_school`, `consolidated`, `paid in full`, or `transferred`. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    /**The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub origination_date: Option<String>,
    ///The original principal balance of the loan.
    pub origination_principal_amount: Option<f64>,
    ///The total dollar amount of the accrued interest balance. For Sallie Mae ( `ins_116944`), this amount is included in the current balance of the loan, so this field will return as `null`.
    pub outstanding_interest_amount: Option<f64>,
    ///The relevant account number that should be used to reference this loan for payments. In the majority of cases, `payment_reference_number` will match a`ccount_number,` but in some institutions, such as Great Lakes (`ins_116861`), it will be different.
    pub payment_reference_number: Option<String>,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`). 
    pub pslf_status: PSLFStatus,
    ///An object representing the repayment plan for the student loan
    pub repayment_plan: StudentRepaymentPlan,
    ///The sequence number of the student loan. Heartland ECSI (`ins_116948`) does not make this field available.
    pub sequence_number: Option<String>,
    ///The address of the student loan servicer. This is generally the remittance address to which payments should be sent.
    pub servicer_address: ServicerAddressData,
    ///The year to date (YTD) interest paid. Availability for this field is limited.
    pub ytd_interest_paid: Option<f64>,
    ///The year to date (YTD) principal paid. Availability for this field is limited.
    pub ytd_principal_paid: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditCardLiability {
    ///The ID of the account that this liability belongs to.
    pub account_id: Option<String>,
    ///The various interest rates that apply to the account.
    pub aprs: Vec<APR>,
    ///true if a payment is currently overdue. Availability for this field is limited.
    pub is_overdue: Option<bool>,
    ///The amount of the last payment.
    pub last_payment_amount: f64,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited.
    pub last_payment_date: Option<String>,
    ///The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_statement_issue_date: String,
    ///The total amount owed as of the last statement issued
    pub last_statement_balance: f64,
    ///The minimum payment due for the next billing cycle.
    pub minimum_payment_amount: f64,
    ///The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageLiability {
    ///The ID of the account that this liability belongs to.
    pub account_id: String,
    ///The account number of the loan.
    pub account_number: String,
    ///The current outstanding amount charged for late payment.
    pub current_late_fee: Option<f64>,
    ///Total amount held in escrow to pay taxes and insurance on behalf of the borrower.
    pub escrow_balance: Option<f64>,
    ///Indicates whether the borrower has private mortgage insurance in effect.
    pub has_pmi: Option<bool>,
    ///Indicates whether the borrower will pay a penalty for early payoff of mortgage.
    pub has_prepayment_penalty: Option<bool>,
    ///Object containing metadata about the interest rate for the mortgage.
    pub interest_rate: MortgageInterestRate,
    ///The amount of the last payment.
    pub last_payment_amount: Option<f64>,
    ///The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub last_payment_date: Option<String>,
    ///Description of the type of loan, for example `conventional`, `fixed`, or `variable`. This field is provided directly from the loan servicer and does not have an enumerated set of possible values.
    pub loan_type_description: Option<String>,
    ///Full duration of mortgage as at origination (e.g. `10 year`).
    pub loan_term: Option<String>,
    ///Original date on which mortgage is due in full. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub maturity_date: Option<String>,
    ///The amount of the next payment.
    pub next_monthly_payment: Option<f64>,
    ///The due date for the next payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub next_payment_due_date: Option<String>,
    ///The date on which the loan was initially lent. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub origination_date: Option<String>,
    ///The original principal balance of the mortgage.
    pub origination_principal_amount: Option<f64>,
    ///Amount of loan (principal + interest) past due for payment.
    pub past_due_amount: Option<f64>,
    ///Object containing fields describing property address.
    pub property_address: MortgagePropertyAddress,
    ///The year to date (YTD) interest paid.
    pub ytd_interest_paid: Option<f64>,
    ///The YTD principal paid.
    pub ytd_principal_paid: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgageInterestRate {
    ///Percentage value (interest rate of current mortgage, not APR) of interest payable on a loan.
    pub percentage: Option<f64>,
    #[serde(rename = "type")]
    ///The type of interest charged (fixed or variable).
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MortgagePropertyAddress {
    ///The city name.
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    ///The five or nine digit postal code.
    pub postal_code: Option<String>,
    ///The region or state (example "NC").
    pub region: Option<String>,
    ///The full street address (example "564 Main Street, Apt 15").
    pub street: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanStatus {
    /**The date until which the loan will be in its current status. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
*/
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    ///The status type of the student loan
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentRepaymentPlan {
    ///The description of the repayment plan as provided by the servicer.
    pub description: Option<String>,
    #[serde(rename = "type")]
    ///The type of the repayment plan.
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PSLFStatus {
    ///The estimated date borrower will have completed 120 qualifying monthly payments. Returned in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD).
    pub estimated_eligibility_date: Option<String>,
    ///The number of qualifying payments that have been made.
    pub payments_made: Option<f64>,
    ///The number of qualifying payments remaining.
    pub payments_remaining: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicerAddressData {
    ///The full city name
    pub city: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    ///The postal code
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct APR {
    /**Annual Percentage Rate applied.
*/
    pub apr_percentage: f64,
    ///The type of balance to which the APR applies.
    pub apr_type: String,
    ///Amount of money that is subjected to the APR if a balance was carried beyond payment due date. How it is calculated can vary by card issuer. It is often calculated as an average daily balance.
    pub balance_subject_to_apr: Option<f64>,
    ///Amount of money charged due to interest from last statement.
    pub interest_charge_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthMetadata {
    ///Metadata specifically related to which auth methods an institution supports.
    pub supported_methods: AuthSupportedMethods,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSupportedMethods {
    ///Indicates if instant auth is supported.
    pub instant_auth: bool,
    ///Indicates if instant match is supported.
    pub instant_match: bool,
    ///Indicates if automated microdeposits are supported.
    pub automated_micro_deposits: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    ///Indicates whether the institution supports payments from a different country.
    pub supports_international_payments: bool,
    /**A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.

Example: `{"GBP": "10000"}`
*/
    pub maximum_payment_amount: PaymentInitiationMaximumPaymentAmount,
    ///Indicates whether the institution supports returning refund details when initiating a payment.
    pub supports_refund_details: bool,
    ///Metadata specifically related to valid Payment Initiation standing order configurations for the institution.
    pub standing_order_metadata: PaymentInitiationStandingOrderMetadata,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationMaximumPaymentAmount {}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationStandingOrderMetadata {
    ///Indicates whether the institution supports closed-ended standing orders by providing an end date.
    pub supports_standing_order_end_date: bool,
    ///This is only applicable to `MONTHLY` standing orders. Indicates whether the institution supports negative integers (-1 to -5) for setting up a `MONTHLY` standing order relative to the end of the month.
    pub supports_standing_order_negative_execution_days: bool,
    ///A list of the valid standing order intervals supported by the institution.
    pub valid_standing_order_intervals: Vec<PaymentScheduleInterval>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentInitiationAddress {
    ///An array of length 1-2 representing the street address where the recipient is located. Maximum of 70 characters.
    pub street: Vec<String>,
    ///The city where the recipient is located. Maximum of 35 characters.
    pub city: String,
    ///The postal code where the recipient is located. Maximum of 16 characters.
    pub postal_code: String,
    ///The ISO 3166-1 alpha-2 country code where the recipient is located.
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleBase {
    ///The frequency interval of the payment.
    pub interval: Option<PaymentScheduleInterval>,
    /**The day of the interval on which to schedule the payment.

If the payment interval is weekly, `interval_execution_day` should be an integer from 1 (Monday) to 7 (Sunday).

If the payment interval is monthly, `interval_execution_day` should be an integer indicating which day of the month to make the payment on. Integers from 1 to 28 can be used to make a payment on that day of the month. Negative integers from -1 to -5 can be used to make a payment relative to the end of the month. To make a payment on the last day of the month, use -1; to make the payment on the second-to-last day, use -2, and so on.*/
    pub interval_execution_day: Option<i64>,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will begin on the first `interval_execution_day` on or after the `start_date`.

If the first `interval_execution_day` on or after the start date is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make the first payment on that day, but it is not guaranteed to do so.*/
    pub start_date: Option<String>,
    /**A date in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Standing order payments will end on the last `interval_execution_day` on or before the `end_date`.
If the only `interval_execution_day` between the start date and the end date (inclusive) is also the same day that `/payment_initiation/payment/create` was called, the bank *may* make a payment on that day, but it is not guaranteed to do so.*/
    pub end_date: Option<String>,
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, this field will be `null`.
    pub adjusted_start_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentScheduleInterval(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentScheme(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    ///When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    pub request_refund_details: Option<bool>,
    ///The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to send payments only from the specified bank account.
    pub iban: Option<String>,
    ///An optional object used to restrict the accounts used for payments. If provided, the end user will be able to send payments only from the specified bank account.
    pub bacs: PaymentInitiationOptionalRestrictionBacs,
    ///The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    pub wallet_id: Option<String>,
    /**Payment scheme. If not specified - the default in the region will be used (e.g. `SEPA_CREDIT_TRANSFER` for EU). Using unsupported values will result in a failed payment.

`FASTER_PAYMENTS`: Enables payments to move quickly between UK bank accounts. Default value in the UK.

`SEPA_CREDIT_TRANSFER`: The standard payment to a beneficiary within the SEPA area.

`SEPA_CREDIT_TRANSFER_INSTANT`: Instant payment within the SEPA area. May involve additional fees and may not be available at some banks.*/
    pub scheme: PaymentScheme,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentRefundDetails {
    ///The name of the account holder.
    pub name: String,
    ///The International Bank Account Number (IBAN) for the account.
    pub iban: Option<String>,
    ///An object containing a BACS account number and sort code. If an IBAN is not provided or if this recipient needs to accept domestic GBP-denominated payments, BACS data is required.
    pub bacs: RecipientBACSNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalPaymentScheduleGet(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Products(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatus {
    /**This field is deprecated in favor of the `breakdown` object, which provides more granular institution health data.

`HEALTHY`: the majority of requests are successful
`DEGRADED`: only some requests are successful
`DOWN`: all requests are failing*/
    pub status: String,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) formatted timestamp of the last status change for the institution.
*/
    pub last_status_change: String,
    ///A detailed breakdown of the institution's performance for a request type. The values for `success`, `error_plaid`, and `error_institution` sum to 1.
    pub breakdown: ProductStatusBreakdown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStatusBreakdown {
    ///The percentage of login attempts that are successful, expressed as a decimal.
    pub success: f64,
    /**The percentage of logins that are failing due to an internal Plaid issue, expressed as a decimal.
*/
    pub error_plaid: f64,
    ///The percentage of logins that are failing due to an issue in the institution's system, expressed as a decimal.
    pub error_institution: f64,
    ///The `refresh_interval` may be `DELAYED` or `STOPPED` even when the success rate is high. This value is only returned for Transactions status breakdowns.
    pub refresh_interval: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCustomPassword {
    ///The version of the password schema to use, possible values are 1 or 2. The default value is 2. You should only specify 1 if you know it is necessary for your test suite.
    pub version: Option<String>,
    /**A seed, in the form of a string, that will be used to randomly generate account and transaction data, if this data is not specified using the `override_accounts` argument. If no seed is specified, the randomly generated data will be different each time.

Note that transactions data is generated relative to the Item's creation date. Different Items created on different dates with the same seed for transactions data will have different dates for the transactions. The number of days between each transaction and the Item creation will remain constant. For example, an Item created on December 15 might show a transaction on December 14. An Item created on December 20, using the same seed, would show that same transaction occurring on December 19.*/
    pub seed: String,
    ///An array of account overrides to configure the accounts for the Item. By default, if no override is specified, transactions and account data will be randomly generated based on the account type and subtype, and other products will have fixed or empty data.
    pub override_accounts: Vec<OverrideAccounts>,
    ///Specifies the multi-factor authentication settings to use with this test account
    pub mfa: MFA,
    ///You may trigger a reCAPTCHA in Plaid Link in the Sandbox environment by using the recaptcha field. Possible values are `good` or `bad`. A value of `good` will result in successful Item creation and `bad` will result in a `RECAPTCHA_BAD` error to simulate a failed reCAPTCHA. Both values require the reCAPTCHA to be manually solved within Plaid Link.
    pub recaptcha: String,
    /**An error code to force on Item creation. Possible values are:

`"INSTITUTION_NOT_RESPONDING"`
`"INSTITUTION_NO_LONGER_SUPPORTED"`
`"INVALID_CREDENTIALS"`
`"INVALID_MFA"`
`"ITEM_LOCKED"`
`"ITEM_LOGIN_REQUIRED"`
`"ITEM_NOT_SUPPORTED"`
`"INVALID_LINK_TOKEN"`
`"MFA_NOT_SUPPORTED"`
`"NO_ACCOUNTS"`
`"PLAID_ERROR"`
`"PRODUCTS_NOT_SUPPORTED"`
`"USER_SETUP_REQUIRED"`*/
    pub force_error: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MFA {
    #[serde(rename = "type")]
    /**Possible values are `device`, `selections`, or `questions`.

If value is `device`, the MFA answer is `1234`.

If value is `selections`, the MFA answer is always the first option.

If value is `questions`, the MFA answer is  `answer_<i>_<j>` for the j-th question in the i-th round, starting from 0. For example, the answer to the first question in the second round is `answer_1_0`.*/
    pub type_: String,
    ///Number of rounds of questions. Required if value of `type` is `questions`. 
    pub question_rounds: f64,
    ///Number of questions per round. Required if value of `type` is `questions`. If value of type is `selections`, default value is 2.
    pub questions_per_round: f64,
    ///Number of rounds of selections, used if `type` is `selections`. Defaults to 1.
    pub selection_rounds: f64,
    /**Number of available answers per question, used if `type` is `selection`. Defaults to 2.
*/
    pub selections_per_question: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OverrideAccounts {
    #[serde(rename = "type")]
    /**`investment:` Investment account.

`credit:` Credit card

`depository:` Depository account

`loan:` Loan account

`payroll:` Payroll account

`other:` Non-specified account type

See the [Account type schema](https://plaid.com/docs/api/accounts#account-type-schema) for a full listing of account types and corresponding subtypes.*/
    pub type_: OverrideAccountType,
    ///See the [Account type schema](https://plaid.com/docs/api/accounts/#account-type-schema) for a full listing of account types and corresponding subtypes.
    pub subtype: AccountSubtype,
    /**If provided, the account will start with this amount as the current balance.
*/
    pub starting_balance: f64,
    ///If provided, the account will always have this amount as its  available balance, regardless of current balance or changes in transactions over time.
    pub force_available_balance: f64,
    ///ISO-4217 currency code. If provided, the account will be denominated in the given currency. Transactions will also be in this currency by default.
    pub currency: String,
    ///Allows specifying the metadata of the test account
    pub meta: Meta,
    ///Account and bank identifier number data used to configure the test account. All values are optional.
    pub numbers: Numbers,
    ///Specify the list of transactions on the account.
    pub transactions: Vec<TransactionOverride>,
    ///Specify the holdings on the account.
    pub holdings: Option<HoldingsOverride>,
    ///Specify the list of investments transactions on the account.
    pub investment_transactions: Option<Investments_TransactionsOverride>,
    ///Data about the owner or owners of an account. Any fields not specified will be filled in with default Sandbox information.
    pub identity: OwnerOverride,
    ///Used to configure Sandbox test data for the Liabilities product
    pub liability: LiabilityOverride,
    ///The `inflow_model` allows you to foo a test account that receives regular income or make regular payments on a loan. Any transactions generated by the `inflow_model` will appear in addition to randomly generated test data or transactions specified by `override_accounts`.
    pub inflow_model: InflowModel,
    ///Specify payroll data on the account.
    pub income: Option<IncomeOverride>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    ///The account's name
    pub name: String,
    ///The account's official name
    pub official_name: String,
    ///The account's limit
    pub limit: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Numbers {
    ///Will be used for the account number.
    pub account: Option<String>,
    ///Must be a valid ACH routing number.
    pub ach_routing: Option<String>,
    ///Must be a valid wire transfer routing number.
    pub ach_wire_routing: Option<String>,
    ///EFT institution number. Must be specified alongside `eft_branch`.
    pub eft_institution: Option<String>,
    ///EFT branch number. Must be specified alongside `eft_institution`.
    pub eft_branch: Option<String>,
    ///Bank identifier code (BIC). Must be specified alongside `international_iban`.
    pub international_bic: Option<String>,
    ///International bank account number (IBAN). If no account number is specified via `account`, will also be used as the account number by default. Must be specified alongside `international_bic`.
    pub international_iban: Option<String>,
    ///BACS sort code
    pub bacs_sort_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionOverride {
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/#transactions-get-response-transactions-authorized-date) field.
    pub date_transacted: String,
    ///The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions.
    pub date_posted: String,
    ///The transaction amount. Can be negative.
    pub amount: f64,
    ///The transaction description.
    pub description: String,
    ///The ISO-4217 format currency code for the transaction.
    pub currency: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityOverride {
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    ///A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsOverride {
    ///The last price given by the institution for this security
    pub institution_price: f64,
    ///The date at which `institution_price` was current. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub institution_price_as_of: Option<String>,
    ///The average original value of the holding. Multiple cost basis values for the same security purchased at different prices are not supported.
    pub cost_basis: Option<f64>,
    ///The total quantity of the asset held, as reported by the financial institution.
    pub quantity: f64,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: SecurityOverride,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Investments_TransactionsOverride {
    ///Posting date for the transaction. Must be formatted as an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) date.
    pub date: String,
    ///The institution's description of the transaction.
    pub name: String,
    ///The number of units of the security involved in this transaction. Must be positive if the type is a buy and negative if the type is a sell.
    pub quantity: f64,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    ///The combined value of all fees applied to this transaction.
    pub fees: Option<f64>,
    #[serde(rename = "type")]
    /**The type of the investment transaction. Possible values are:
`buy`: Buying an investment
`sell`: Selling an investment
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer*/
    pub type_: String,
    ///Either a valid `iso_currency_code` or `unofficial_currency_code`
    pub currency: String,
    ///Specify the security associated with the holding or investment transaction. When inputting custom security data to the Sandbox, Plaid will perform post-data-retrieval normalization and enrichment. These processes may cause the data returned by the Sandbox to be slightly different from the data you input. An ISO-4217 currency code and a security identifier (`ticker_symbol`, `cusip`, `isin`, or `sedol`) are required.
    pub security: Option<SecurityOverride>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilityOverride {
    #[serde(rename = "type")]
    ///The type of the liability object, either `credit` or `student`. Mortgages are not currently supported in the custom Sandbox.
    pub type_: String,
    ///The purchase APR percentage value. For simplicity, this is the only interest rate used to calculate interest charges. Can only be set if `type` is `credit`.
    pub purchase_apr: f64,
    ///The cash APR percentage value. Can only be set if `type` is `credit`.
    pub cash_apr: f64,
    ///The balance transfer APR percentage value. Can only be set if `type` is `credit`. Can only be set if `type` is `credit`.
    pub balance_transfer_apr: f64,
    ///The special APR percentage value. Can only be set if `type` is `credit`.
    pub special_apr: f64,
    ///Override the `last_payment_amount` field. Can only be set if `type` is `credit`.
    pub last_payment_amount: f64,
    ///Override the `minimum_payment_amount` field. Can only be set if `type` is `credit` or `student`.
    pub minimum_payment_amount: f64,
    ///Override the `is_overdue` field
    pub is_overdue: bool,
    ///The date on which the loan was initially lent, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Can only be set if `type` is `student`.
    pub origination_date: String,
    ///The original loan principal. Can only be set if `type` is `student`.
    pub principal: f64,
    ///The interest rate on the loan as a percentage. Can only be set if `type` is `student`.
    pub nominal_apr: f64,
    ///If set, interest capitalization begins at the given number of months after loan origination. By default interest is never capitalized. Can only be set if `type` is `student`.
    pub interest_capitalization_grace_period_months: f64,
    ///Student loan repayment information used to configure Sandbox test data for the Liabilities product
    pub repayment_model: StudentLoanRepaymentModel,
    ///Override the `expected_payoff_date` field. Can only be set if `type` is `student`.
    pub expected_payoff_date: String,
    ///Override the `guarantor` field. Can only be set if `type` is `student`.
    pub guarantor: String,
    ///Override the `is_federal` field. Can only be set if `type` is `student`.
    pub is_federal: bool,
    ///Override the `loan_name` field. Can only be set if `type` is `student`.
    pub loan_name: String,
    ///An object representing the status of the student loan
    pub loan_status: StudentLoanStatus,
    ///Override the `payment_reference_number` field. Can only be set if `type` is `student`.
    pub payment_reference_number: String,
    ///Information about the student's eligibility in the Public Service Loan Forgiveness program. This is only returned if the institution is Fedloan (`ins_116527`). 
    pub pslf_status: PSLFStatus,
    ///Override the `repayment_plan.description` field. Can only be set if `type` is `student`.
    pub repayment_plan_description: String,
    ///Override the `repayment_plan.type` field. Can only be set if `type` is `student`. Possible values are: `"extended graduated"`, `"extended standard"`, `"graduated"`, `"income-contingent repayment"`, `"income-based repayment"`, `"interest only"`, `"other"`, `"pay as you earn"`, `"revised pay as you earn"`, or `"standard"`.
    pub repayment_plan_type: String,
    ///Override the `sequence_number` field. Can only be set if `type` is `student`.
    pub sequence_number: String,
    ///A physical mailing address.
    pub servicer_address: Address,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StudentLoanRepaymentModel {
    #[serde(rename = "type")]
    ///The only currently supported value for this field is `standard`.
    pub type_: String,
    ///Configures the number of months before repayment starts.
    pub non_repayment_months: f64,
    ///Configures the number of months of repayments before the loan is paid off.
    pub repayment_months: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InflowModel {
    #[serde(rename = "type")]
    /**Inflow foo. One of the following:

`none`: No income

`monthly-income`: Income occurs once per month `monthly-balance-payment`: Pays off the balance on a liability account at the given statement day of month.

`monthly-interest-only-payment`: Makes an interest-only payment on a liability account at the given statement day of month. 

Note that account types supported by Liabilities will accrue interest in the Sandbox. The types impacted are account type `credit` with subtype `credit` or `paypal`, and account type `loan` with subtype `student` or `mortgage`.*/
    pub type_: String,
    ///Amount of income per month. This value is required if `type` is `monthly-income`.
    pub income_amount: f64,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the income transaction will appear. The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub payment_day_of_month: f64,
    ///The name of the income transaction. This field is required if `type` is `monthly-income`, `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub transaction_name: String,
    ///Number between 1 and 28, or `last` meaning the last day of the month. The day of the month on which the balance is calculated for the next payment. The name of the income transaction. This field is required if `type` is `monthly-balance-payment` or `monthly-interest-only-payment`.
    pub statement_day_of_month: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeOverride {
    ///A list of paystubs associated with the account.
    pub paystubs: Option<Vec<PaystubOverride>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverride {
    ///The employer on the paystub.
    pub employer: Option<PaystubOverrideEmployer>,
    ///The employee on the paystub.
    pub employee: Option<PaystubOverrideEmployee>,
    ///
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    ///Details about the pay period.
    pub pay_period_details: Option<PayPeriodDetails>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployer {
    ///The name of the employer.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployee {
    ///The name of the employee.
    pub name: Option<String>,
    ///The address of the employee.
    pub address: Option<PaystubOverrideEmployeeAddress>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubOverrideEmployeeAddress {
    ///The full city name.
    pub city: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    ///5 digit postal code.
    pub postal_code: Option<String>,
    ///The country of the address.
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AutomaticallyVerifiedWebhook {
    ///`AUTH`
    pub webhook_type: String,
    ///`AUTOMATICALLY_VERIFIED`
    pub webhook_code: String,
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct JWTHeader {
    ///
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationExpiredWebhook {
    ///`AUTH`
    pub webhook_type: String,
    ///`VERIFICATION_EXPIRED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The `account_id` of the account associated with the webhook
    pub account_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookUpdateAcknowledgedWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`WEBHOOK_UPDATE_ACKNOWLEDGED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The new webhook URL
    pub new_webhook_url: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PendingExpirationWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`PENDING_EXPIRATION`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The date and time at which the Item's access consent will expire, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format
    pub consent_expiration_time: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemErrorWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`ERROR`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: PlaidError,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemProductReadyWebhook {
    ///`INCOME`
    pub webhook_type: String,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Recaptcha_RequiredError {
    ///RECAPTCHA_ERROR
    pub error_type: String,
    ///RECAPTCHA_REQUIRED
    pub error_code: String,
    ///
    pub display_message: String,
    ///400
    pub http_code: String,
    ///Your user will be prompted to solve a Google reCAPTCHA challenge in the Link Recaptcha pane. If they solve the challenge successfully, the user's request is resubmitted and they are directed to the next Item creation step.
    pub link_user_experience: String,
    ///Plaid's fraud system detects abusive traffic and considers a variety of parameters throughout Item creation requests. When a request is considered risky or possibly fraudulent, Link presents a reCAPTCHA for the user to solve.
    pub common_causes: String,
    /**Link will automatically guide your user through reCAPTCHA verification. As a general rule, we recommend instrumenting basic fraud monitoring to detect and protect your website from spam and abuse.

If your user cannot verify their session, please submit a Support ticket with the following identifiers: `link_session_id` or `request_id`*/
    pub troubleshooting_steps: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfersEventsUpdateWebhook {
    ///`BANK_TRANSFERS`
    pub webhook_type: String,
    ///`BANK_TRANSFERS_EVENTS_UPDATE`
    pub webhook_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentsDefaultUpdateWebhook {
    ///`INVESTMENTS_TRANSACTIONS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new transactions reported since the last time this webhook was fired.
    pub new_investments_transactions: f64,
    ///The number of canceled transactions reported since the last time this webhook was fired.
    pub canceled_investments_transactions: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HoldingsDefaultUpdateWebhook {
    ///`HOLDINGS`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///The number of new holdings reported since the last time this webhook was fired.
    pub new_holdings: f64,
    ///The number of updated holdings reported since the last time this webhook was fired.
    pub updated_holdings: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesDefaultUpdateWebhook {
    ///`LIABILITIES`
    pub webhook_type: String,
    ///`DEFAULT_UPDATE`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: PlaidError,
    ///An array of `account_id`'s for accounts that contain new liabilities.'
    pub account_ids_with_new_liabilities: Vec<String>,
    /**An object with keys of `account_id`'s that are mapped to their respective liabilities fields that changed.

Example: `{ "XMBvvyMGQ1UoLbKByoMqH3nXMj84ALSdE5B58": ["past_amount_due"] }`
*/
    pub account_ids_with_updated_liabilities: LiabilitiesAccountIdsWithUpdatedLiabilities,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LiabilitiesAccountIdsWithUpdatedLiabilities {}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsProductReadyWebhook {
    ///`ASSETS`
    pub webhook_type: String,
    ///`PRODUCT_READY`
    pub webhook_code: String,
    ///The `asset_report_id` that can be provided to `/asset_report/get` to retrieve the Asset Report.
    pub asset_report_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsErrorWebhook {
    ///`ASSETS`
    pub webhook_type: String,
    ///`ERROR`
    pub webhook_code: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: PlaidError,
    ///The ID associated with the Asset Report.
    pub asset_report_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Cause {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: PlaidError,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Warning {
    ///The warning type, which will always be `ASSET_REPORT_WARNING`
    pub warning_type: String,
    ///The warning code identifies a specific kind of warning. Currently, the only possible warning code is `OWNERS_UNAVAILABLE`, which indicates that account-owner information is not available.
    pub warning_code: String,
    ///An error object and associated `item_id` used to identify a specific Item and error when a batch operation operating on multiple Items has encountered an error in one of the Items.
    pub cause: Cause,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentAmount {
    ///The ISO-4217 currency code of the payment. For standing orders, `"GBP"` must be used.
    pub currency: String,
    ///The amount of the payment. Must contain at most two digits of precision e.g. `1.23`. Minimum accepted value is `1`.
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportUser {
    ///An identifier you determine and submit for the user.
    pub client_user_id: Option<String>,
    ///The user's first name. Required for the Fannie Mae Day 1 Certainty™ program.
    pub first_name: Option<String>,
    ///The user's middle name
    pub middle_name: Option<String>,
    ///The user's last name.  Required for the Fannie Mae Day 1 Certainty™ program.
    pub last_name: Option<String>,
    /**The user's Social Security Number. Required for the Fannie Mae Day 1 Certainty™ program.

Format: "ddd-dd-dddd"*/
    pub ssn: Option<String>,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567". Phone numbers provided in other formats will be parsed on a best-effort basis.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportRefreshAssetReportToken(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneCurrencyCodeList {
    ///Plaid supports all ISO 4217 currency codes.
    pub iso_currency_code: String,
    ///List of unofficial currency codes
    pub unofficial_currency_code: UnofficialCurrencyCodeList,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnofficialCurrencyCodeList(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneAccountType {
    ///An account type holding cash, in which funds are deposited. Supported products for `depository` accounts are: Auth (`checking` and `savings` types only), Balance, Transactions, Identity, Payment Initiation, and Assets.
    pub depository: DepositoryAccount,
    ///A credit card type account. Supported products for `credit` accounts are: Balance, Transactions, Identity, and Liabilities.
    pub credit: CreditAccount,
    ///A loan type account. Supported products for `loan` accounts are: Balance, Liabilities, and Transactions.
    pub loan: LoanAccount,
    ///An investment account. Supported products for `investment` accounts are: Balance and Investments. In API versions 2018-05-22 and earlier, this type is called `brokerage`.
    pub investment: InvestmentAccountSubtypeStandalone,
    ///Other or unknown account type. Supported products for `other` accounts are: Balance, Transactions, Identity, and Assets.
    pub other: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtypeStandalone(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReport {
    ///A unique ID identifying an Asset Report. Like all Plaid identifiers, this ID is case sensitive.
    pub asset_report_id: AssetReportId,
    ///An identifier you determine and submit for the Asset Report.
    pub client_report_id: Option<String>,
    ///The date and time when the Asset Report was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: String,
    ///The duration of transaction history you requested
    pub days_requested: f64,
    ///The user object allows you to provide additional information about the user to be appended to the Asset Report. All fields are optional. The `first_name`, `last_name`, and `ssn` fields are required if you would like the Report to be eligible for Fannie Mae’s Day 1 Certainty™ program.
    pub user: AssetReportUser,
    ///Data returned by Plaid about each of the Items included in the Asset Report.
    pub items: Vec<AssetReportItem>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportItem {
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///The full financial institution name associated with the Item.
    pub institution_name: String,
    ///The id of the financial institution associated with the Item.
    pub institution_id: String,
    ///The date and time when this Item’s data was last retrieved from the financial institution, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub date_last_updated: String,
    ///Data about each of the accounts open on the Item.
    pub accounts: Vec<AccountAssets>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentStatusUpdateWebhook {
    ///`PAYMENT_INITIATION`
    pub webhook_type: String,
    ///`PAYMENT_STATUS_UPDATE`
    pub webhook_code: String,
    ///The `payment_id` for the payment being updated
    pub payment_id: String,
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub new_payment_status: PaymentInitiationPaymentStatus,
    /**The status of the payment.

`PAYMENT_STATUS_INPUT_NEEDED`: This is the initial state of all payments. It indicates that the payment is waiting on user input to continue processing. A payment may re-enter this state later on if further input is needed.

`PAYMENT_STATUS_INITIATED`: The payment has been successfully authorised and accepted by the financial institution but has not been executed.

`PAYMENT_STATUS_INSUFFICIENT_FUNDS`: The payment has failed due to insufficient funds.

`PAYMENT_STATUS_FAILED`: The payment has failed to be initiated. This error is retryable once the root cause is resolved.

`PAYMENT_STATUS_BLOCKED`: The payment has been blocked. This is a retryable error.

`PAYMENT_STATUS_AUTHORISING`: The payment is currently being processed. The payment will automatically exit this state when the financial institution has authorised the transaction.

`PAYMENT_STATUS_CANCELLED`: The payment was cancelled during authorisation.

`PAYMENT_STATUS_EXECUTED`: The payment has been successfully initiated and is considered complete.

`PAYMENT_STATUS_ESTABLISHED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.

`PAYMENT_STATUS_REJECTED`: The payment was rejected by the financial institution.

Deprecated:
These statuses will be removed in a future release.

`PAYMENT_STATUS_UNKNOWN`: The payment status is unknown.

`PAYMENT_STATUS_PROCESSING`: The payment is currently being processed. The payment will automatically exit this state when processing is complete.

`PAYMENT_STATUS_COMPLETED`: Indicates that the standing order has been successfully established. This state is only used for standing orders.*/
    pub old_payment_status: PaymentInitiationPaymentStatus,
    ///The original value of the reference when creating the payment.
    pub original_reference: Option<String>,
    ///The value of the reference sent to the bank after adjustment to pass bank validation rules.
    pub adjusted_reference: Option<String>,
    ///The original value of the `start_date` provided during the creation of a standing order. If the payment is not a standing order, this field will be `null`.
    pub original_start_date: Option<String>,
    ///The start date sent to the bank after adjusting for holidays or weekends.  Will be provided in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). If the start date did not require adjustment, or if the payment is not a standing order, this field will be `null`.
    pub adjusted_start_date: Option<String>,
    ///The timestamp of the update, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2017-09-14T14:42:19.350Z"`
    pub timestamp: String,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Holding {
    ///The Plaid `account_id` associated with the holding.
    pub account_id: String,
    ///The Plaid `security_id` associated with the holding.
    pub security_id: String,
    ///The last price given by the institution for this security.
    pub institution_price: f64,
    ///The date at which `institution_price` was current.
    pub institution_price_as_of: Option<String>,
    ///The value of the holding, as reported by the institution.
    pub institution_value: f64,
    ///The cost basis of the holding.
    pub cost_basis: Option<f64>,
    ///The total quantity of the asset held, as reported by the financial institution. If the security is an option, `quantity` will reflect the total number of options (typically the number of contracts multiplied by 100), not the number of contracts.
    pub quantity: f64,
    ///The ISO-4217 currency code of the holding. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.
*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Security {
    ///A unique, Plaid-specific identifier for the security, used to associate securities with holdings. Like all Plaid identifiers, the `security_id` is case sensitive.
    pub security_id: String,
    ///12-character ISIN, a globally unique securities identifier.
    pub isin: Option<String>,
    ///9-character CUSIP, an identifier assigned to North American securities.
    pub cusip: Option<String>,
    ///7-character SEDOL, an identifier assigned to securities in the UK.
    pub sedol: Option<String>,
    ///An identifier given to the security by the institution
    pub institution_security_id: Option<String>,
    ///If `institution_security_id` is present, this field indicates the Plaid `institution_id` of the institution to whom the identifier belongs.
    pub institution_id: Option<String>,
    ///In certain cases, Plaid will provide the ID of another security whose performance resembles this security, typically when the original security has low volume, or when a private security can be modeled with a publicly traded security.
    pub proxy_security_id: Option<String>,
    ///A descriptive name for the security, suitable for display.
    pub name: Option<String>,
    ///The security’s trading symbol for publicly traded securities, and otherwise a short identifier if available.
    pub ticker_symbol: Option<String>,
    ///Indicates that a security is a highly liquid asset and can be treated like cash.
    pub is_cash_equivalent: Option<bool>,
    #[serde(rename = "type")]
    /**The security type of the holding. Valid security types are:

`cash`: Cash, currency, and money market funds

`derivative`: Options, warrants, and other derivative instruments

`equity`: Domestic and foreign equities

`etf`: Multi-asset exchange-traded investment funds

`fixed income`: Bonds and certificates of deposit (CDs)

`loan`: Loans and loan receivables.

`mutual fund`: Open- and closed-end vehicles pooling funds of multiple investors.

`other`: Unknown or other investment types*/
    pub type_: Option<String>,
    ///Price of the security at the close of the previous trading session. `null` for non-public securities. If the security is a foreign currency or a cryptocurrency this field will be updated daily and will be priced in USD.
    pub close_price: Option<f64>,
    ///Date for which `close_price` is accurate. Always `null` if `close_price` is `null`.
    pub close_price_as_of: Option<String>,
    ///The ISO-4217 currency code of the price given. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    ///The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive.
    pub investment_transaction_id: String,
    ///A legacy field formerly used internally by Plaid to identify certain canceled transactions.
    pub cancel_transaction_id: Option<String>,
    ///The `account_id` of the account against which this transaction posted.
    pub account_id: String,
    ///The `security_id` to which this transaction is related.
    pub security_id: Option<String>,
    ///The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction.
    pub date: String,
    ///The institution’s description of the transaction.
    pub name: String,
    ///The number of units of the security involved in this transaction.
    pub quantity: f64,
    ///The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities.
    pub amount: f64,
    ///The price of the security at which this transaction occurred.
    pub price: f64,
    ///The combined value of all fees applied to this transaction
    pub fees: Option<f64>,
    #[serde(rename = "type")]
    /**Value is one of the following:
`buy`: Buying an investment
`sell`: Selling an investment
`cancel`: A cancellation of a pending transaction
`cash`: Activity that modifies a cash position
`fee`: A fee on the account
`transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer

For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).*/
    pub type_: String,
    ///For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema).
    pub subtype: String,
    ///The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionType {
    ///Buying an investment
    pub buy: StandaloneInvestmentTransactionBuyType,
    ///Selling an investment
    pub sell: StandaloneInvestmentTransactionSellType,
    ///A cancellation of a pending transaction
    pub cancel: String,
    ///Activity that modifies a cash position
    pub cash: StandaloneInvestmentTransactionCashType,
    ///Fees on the account, e.g. commission, bookkeeping, options-related.
    pub fee: StandaloneInvestmentTransactionFeeType,
    ///Activity that modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer
    pub transfer: StandaloneInvestmentTransactionTransferType,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionBuyType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionCashType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionFeeType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionSellType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct StandaloneInvestmentTransactionTransferType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSubtypes(pub Vec<AccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermissionRevokedWebhook {
    ///`ITEM`
    pub webhook_type: String,
    ///`USER_PERMISSION_REVOKED`
    pub webhook_code: String,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: ItemId,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the deposit switch
    pub deposit_switch_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchGetResponse {
    ///The ID of the deposit switch.
    pub deposit_switch_id: String,
    ///The ID of the bank account the direct deposit was switched to.
    pub target_account_id: Option<String>,
    ///The ID of the Item the direct deposit was switched to.
    pub target_item_id: Option<String>,
    /**
The state, or status, of the deposit switch.

- `initialized` – The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

- `processing` – The deposit switch request has been submitted and is being processed.

- `completed` – The user's employer has fulfilled the deposit switch request.

- `error` – There was an error processing the deposit switch request.*/
    pub state: String,
    /**The method used to make the deposit switch.

- `instant` – User instantly switched their direct deposit to a new or existing bank account by connecting their payroll or employer account.

- `mail` – User requested that Plaid contact their employer by mail to make the direct deposit switch.

- `pdf` – User generated a PDF or email to be sent to their employer with the information necessary to make the deposit switch.'*/
    pub switch_method: Option<String>,
    ///When `true`, user’s direct deposit goes to multiple banks. When false, user’s direct deposit only goes to the target account. Always `null` if the deposit switch has not been completed.
    pub account_has_multiple_allocations: Option<bool>,
    ///When `true`, the target account is allocated the remainder of direct deposit after all other allocations have been deducted. When `false`, user’s direct deposit is allocated as a percent or amount. Always `null` if the deposit switch has not been completed.
    pub is_allocated_remainder: Option<bool>,
    ///The percentage of direct deposit allocated to the target account. Always `null` if the target account is not allocated a percentage or if the deposit switch has not been completed or if `is_allocated_remainder` is true.
    pub percent_allocated: Option<f64>,
    ///The dollar amount of direct deposit allocated to the target account. Always `null` if the target account is not allocated an amount or if the deposit switch has not been completed.
    pub amount_allocated: Option<f64>,
    ///The name of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_name: Option<String>,
    ///The ID of the employer selected by the user. If the user did not select an employer, the value returned is `null`.
    pub employer_id: Option<String>,
    ///The name of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_name: Option<String>,
    ///The ID of the institution selected by the user. If the user did not select an institution, the value returned is `null`.
    pub institution_id: Option<String>,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was created.
*/
    pub date_created: String,
    /**[ISO 8601](https://wikipedia.org/wiki/ISO_8601) date the deposit switch was completed. Always `null` if the deposit switch has not been completed.
*/
    pub date_completed: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchStateUpdateWebhook {
    ///`"DEPOSIT_SWITCH"`
    pub webhook_type: Option<String>,
    ///`"SWITCH_STATE_UPDATE"`
    pub webhook_code: Option<String>,
    /**
The state, or status, of the deposit switch.

`initialized`: The deposit switch has been initialized with the user entering the information required to submit the deposit switch request.

`processing`: The deposit switch request has been submitted and is being processed.

`completed`: The user's employer has fulfilled and completed the deposit switch request.

`error`: There was an error processing the deposit switch request.

For more information, see the [Deposit Switch API reference](/docs/deposit-switch/reference#deposit_switchget).*/
    pub state: Option<String>,
    ///The ID of the deposit switch.
    pub deposit_switch_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportAuditCopyGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The `audit_copy_token` granting access to the Audit Copy you would like to get.
    pub audit_copy_token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferGetResponse {
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferGetResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepID(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Transfer {
    ///Plaid’s unique identifier for a transfer.
    pub id: TransferID,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The account ID that should be credited/debited for this transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The description of the transfer.
    pub description: String,
    ///The datetime when this transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    ///The status of the transfer.
    pub status: TransferStatus,
    /**The status of the sweep for the transfer.
`unswept`: The transfer hasn't been swept yet.
`swept`: The transfer was swept to the sweep account.
`reverse_swept`: The transfer was reversed, funds were pulled back or pushed back to the sweep account.
`null`: The transfer will never be swept (e.g. if the transfer is cancelled or reversed before being swept)*/
    pub sweep_status: TransferSweepStatus,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`.
    pub network: TransferNetwork,
    ///When `true`, you can still cancel this transfer.
    pub cancellable: bool,
    ///The failure reason if the event type for a transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: TransferFailure,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: TransferMetadata,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: TransferAuthorizationGuaranteeDecision,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: TransferAuthorizationGuaranteeDecisionRationale,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransfer {
    ///Plaid’s unique identifier for a bank transfer.
    pub id: BankTransferID,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The account ID that should be credited/debited for this bank transfer.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
    ///The description of the transfer.
    pub description: String,
    ///The datetime when this bank transfer was created. This will be of the form `2006-01-02T15:04:05Z`
    pub created: String,
    ///The status of the transfer.
    pub status: BankTransferStatus,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///When `true`, you can still cancel this bank transfer.
    pub cancellable: bool,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: BankTransferFailure,
    ///A string containing the custom tag provided by the client in the create request. Will be null if not provided.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: BankTransferMetadata,
    ///Plaid’s unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: BankTransferDirection,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ACHClass(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepAmount(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetFailureReason {
    ///A broad categorization of the error.
    pub error_type: Option<String>,
    /**A code representing the reason for a failed transfer intent (i.e., an API error or the authorization being declined).

For a full listing of bank transfer errors, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/).*/
    pub error_code: Option<String>,
    ///A human-readable description of the code associated with a failed transfer intent.
    pub error_message: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateMode(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferAmount(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInRequest {
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///The address associated with the account holder.
    pub address: Option<TransferUserAddressInRequest>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserInResponse {
    ///The user's legal name.
    pub legal_name: String,
    ///The user's phone number.
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///The address associated with the account holder.
    pub address: TransferUserAddressInResponse,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInRequest {
    ///The street number and name (i.e., "100 Market St.").
    pub street: Option<String>,
    ///Ex. "San Francisco"
    pub city: Option<String>,
    ///The state or province (e.g., "California").
    pub region: Option<String>,
    ///The postal code (e.g., "94103").
    pub postal_code: Option<String>,
    ///A two-letter country code (e.g., "US").
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferUserAddressInResponse {
    ///The street number and name (i.e., "100 Market St.").
    pub street: Option<String>,
    ///Ex. "San Francisco"
    pub city: Option<String>,
    ///The state or province (e.g., "California").
    pub region: Option<String>,
    ///The postal code (e.g., "94103").
    pub postal_code: Option<String>,
    ///A two-letter country code (e.g., "US").
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferUser {
    ///The account holder’s full legal name. If the transfer `ach_class` is `ccd`, this should be the business name of the account holder.
    pub legal_name: String,
    ///The account holder’s email.
    pub email_address: Option<String>,
    ///The account holder's routing number. This field is only used in response data. Do not provide this field when making requests.
    pub routing_number: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDecisionRationale {
    /**A code representing the rationale for permitting or declining the proposed transfer. Possible values are:

`NSF` – Transaction likely to result in a return due to insufficient funds.

`RISK` - Transaction is high-risk.

`MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid can only offer `permitted` as a transaction decision.

`LOGIN_REQUIRED` – Unable to collect the account information required for an authorization decision due to Item staleness. Can be rectified using Link update mode.

`ERROR` – Unable to collect the account information required for an authorization decision due to an error.*/
    pub code: String,
    ///A human-readable description of the code associated with a permitted transfer or transfer decline.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecision(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationGuaranteeDecisionRationale {
    /**A code representing the reason Plaid declined to guarantee this transfer:

`RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.

`RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.

`GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.

`RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item.*/
    pub code: String,
    ///A human-readable description of why the transfer cannot be guaranteed.
    pub description: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationProposedTransfer {
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The network or rails used for the transfer.
    pub network: String,
    ///Plaid's unique identifier for the origination account that was used for this transfer.
    pub origination_account_id: String,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationDevice {
    ///The IP address of the device being used to initiate the authorization.
    pub ip_address: Option<String>,
    ///The user agent of the device being used to initiate the authorization.
    pub user_agent: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferMetadata {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMetadata {}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepStatus(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferNetwork(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferNetwork(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Transfer errors](https://plaid.com/docs/errors/transfer/#ach-return-codes).
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    pub description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferFailure {
    ///The ACH return code, e.g. `R01`.  A return code will be provided if and only if the transfer status is `reversed`. For a full listing of ACH return codes, see [Bank Transfers errors](https://plaid.com/docs/errors/bank-transfers/#ach-return-codes).
    pub ach_return_code: Option<String>,
    ///A human-readable description of the reason for the failure or reversal.
    pub description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: TransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`.
    pub network: TransferNetwork,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    ///Information about the device being used to initiate the authorization.
    pub device: Option<TransferAuthorizationDevice>,
    ///Plaid's unique identifier for the origination account for this authorization. If not specified, the default account will be used.
    pub origination_account_id: Option<String>,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**Deprecated. `authorization_id` is now for used idempotency instead.

A random key provided by the client, per unique transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single transfer is created.*/
    pub idempotency_key: Option<TransferCreateIdempotencyKey>,
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: TransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    ///Plaid’s unique identifier for a transfer authorization. This parameter also serves the purpose of acting as an idempotency identifier.
    pub authorization_id: String,
    #[serde(rename = "type")]
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: TransferType,
    ///The network or rails used for the transfer. Valid options are `ach` or `same-day-ach`.
    pub network: TransferNetwork,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: TransferMetadata,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
    ///The currency of the transfer amount. The default value is "USD".
    pub iso_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**A random key provided by the client, per unique bank transfer. Maximum of 50 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. For example, if a request to create a bank transfer fails due to a network connection error, you can retry the request with the same idempotency key to guarantee that only a single bank transfer is created.*/
    pub idempotency_key: BankTransferIdempotencyKey,
    ///The Plaid `access_token` for the account that will be debited or credited.
    pub access_token: BankTransferAccessToken,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: String,
    #[serde(rename = "type")]
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub type_: BankTransferType,
    ///The network or rails used for the transfer. Valid options are `ach`, `same-day-ach`, or `wire`.
    pub network: BankTransferNetwork,
    ///The amount of the bank transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: BankTransferAmount,
    ///The currency of the transfer amount – should be set to "USD".
    pub iso_currency_code: String,
    ///The transfer description. Maximum of 10 characters.
    pub description: String,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: Option<ACHClass>,
    ///The legal name and other information for the account holder.
    pub user: BankTransferUser,
    ///An arbitrary string provided by the client for storage with the bank transfer. May be up to 100 characters.
    pub custom_tag: Option<String>,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: BankTransferMetadata,
    ///Plaid’s unique identifier for the origination account for this transfer. If you have more than one origination account, this value must be specified. Otherwise, this field should be left blank.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorizationCreateResponse {
    ///Contains the authorization decision for a proposed transfer
    pub authorization: TransferAuthorization,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAuthorization {
    ///Plaid’s unique identifier for a transfer authorization.
    pub id: TransferAuthorizationID,
    ///The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`.
    pub created: String,
    /**
A decision regarding the proposed transfer.

`approved` – The proposed transfer has received the end user's consent and has been approved for processing. Plaid has also reviewed the proposed transfer and has approved it for processing. 

`permitted` – Plaid was unable to fetch the information required to approve or decline the proposed transfer. You may proceed with the transfer, but further review is recommended. Plaid is awaiting further instructions from the client.

`declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details.*/
    pub decision: String,
    ///The rationale for Plaid's decision regarding a proposed transfer. Will be null for `approved` decisions.
    pub decision_rationale: TransferAuthorizationDecisionRationale,
    ///Indicates whether the transfer is guaranteed by Plaid (Guaranteed ACH customers only). This field will contain either `GUARANTEED` or `NOT_GUARANTEED` indicating whether Plaid will guarantee the transfer. If the transfer is not guaranteed, additional information will be provided in the `guarantee_decision_rationale` field. Refer to the `code` field in `guarantee_decision_rationale` for details.
    pub guarantee_decision: TransferAuthorizationGuaranteeDecision,
    ///The rationale for Plaid's decision to not guarantee a transfer. Will be `null` unless `guarantee_decision` is `NOT_GUARANTEED`.
    pub guarantee_decision_rationale: TransferAuthorizationGuaranteeDecisionRationale,
    ///Details regarding the proposed transfer.
    pub proposed_transfer: TransferAuthorizationProposedTransfer,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCreateResponse {
    ///Represents a transfer within the Transfers API.
    pub transfer: Transfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCreateResponse {
    ///Represents a bank transfer within the Bank Transfers API.
    pub bank_transfer: BankTransfer,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///The maximum number of transfers to return.
    pub count: Option<i64>,
    ///The number of transfers to skip before returning results.
    pub offset: Option<i64>,
    ///Filter transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///The maximum number of bank transfers to return.
    pub count: Option<i64>,
    ///The number of bank transfers to skip before returning results.
    pub offset: Option<i64>,
    ///Filter bank transfers to only those originated through the specified origination account.
    pub origination_account_id: Option<String>,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: BankTransferDirection,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferListResponse {
    ///
    pub transfers: Vec<Transfer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferListResponse {
    ///
    pub bank_transfers: Vec<BankTransfer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferDirection(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferCancelResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: Option<String>,
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub transfer_type: Option<String>,
    ///Filter events by event type.
    pub event_types: Option<Vec<TransferEventType>>,
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: Option<String>,
    ///The maximum number of transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    ///The offset into the list of transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub start_date: Option<String>,
    ///The end datetime of bank transfers to list. This should be in RFC 3339 format (i.e. `2019-12-06T22:35:49Z`)
    pub end_date: Option<String>,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: Option<String>,
    ///The account ID to get events for all transactions to/from an account.
    pub account_id: Option<String>,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into your origination account; a `credit` indicates a transfer of money out of your origination account.
    pub bank_transfer_type: Option<String>,
    ///Filter events by event type.
    pub event_types: Option<Vec<BankTransferEventType>>,
    ///The maximum number of bank transfer events to return. If the number of events matching the above parameters is greater than `count`, the most recent events will be returned.
    pub count: Option<i64>,
    ///The offset into the list of bank transfer events. When `count`=25 and `offset`=0, the first 25 events will be returned. When `count`=25 and `offset`=25, the next 25 bank transfer events will be returned.
    pub offset: Option<i64>,
    ///The origination account ID to get events for transfers from a specific origination account.
    pub origination_account_id: Option<String>,
    /**Indicates the direction of the transfer: `outbound`: for API-initiated transfers
`inbound`: for payments received by the FBO account.*/
    pub direction: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEvent {
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    /**The type of event that this transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`reversed`: A posted transfer was reversed.

`swept`: The transfer was swept to / from the sweep account.

`reverse_swept`: Due to the transfer reversing, funds were pulled from or pushed back to the sweep account.*/
    pub event_type: TransferEventType,
    ///The account ID associated with the transfer.
    pub account_id: String,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferID,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///The type of transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub transfer_type: TransferType,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub transfer_amount: TransferAmount,
    ///The failure reason if the event type for a transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: TransferFailure,
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: TransferSweepID,
    ///A signed amount of how much was `swept` or `reverse_swept` (decimal string with two digits of precision e.g. "-5.50").
    pub sweep_amount: TransferSweepAmount,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEvent {
    ///Plaid’s unique identifier for this event. IDs are sequential unsigned 64-bit integers.
    pub event_id: i64,
    ///The datetime when this event occurred. This will be of the form `2006-01-02T15:04:05Z`.
    pub timestamp: String,
    /**The type of event that this bank transfer represents.

`pending`: A new transfer was created; it is in the pending state.

`cancelled`: The transfer was cancelled by the client.

`failed`: The transfer failed, no funds were moved.

`posted`: The transfer has been successfully submitted to the payment network.

`reversed`: A posted transfer was reversed.*/
    pub event_type: BankTransferEventType,
    ///The account ID associated with the bank transfer.
    pub account_id: String,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferID,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///The type of bank transfer. This will be either `debit` or `credit`.  A `debit` indicates a transfer of money into the origination account; a `credit` indicates a transfer of money out of the origination account.
    pub bank_transfer_type: BankTransferType,
    ///The bank transfer amount.
    pub bank_transfer_amount: String,
    ///The currency of the bank transfer amount.
    pub bank_transfer_iso_currency_code: String,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: BankTransferFailure,
    ///Indicates the direction of the transfer: `outbound` for API-initiated transfers, or `inbound` for payments received by the FBO account.
    pub direction: BankTransferDirection,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventListResponse {
    ///
    pub transfer_events: Vec<TransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventListResponse {
    ///
    pub bank_transfer_events: Vec<BankTransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    ///The maximum number of bank transfer events to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The latest (largest) `event_id` fetched via the sync endpoint, or 0 initially.
    pub after_id: i64,
    ///The maximum number of transfer events to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferEventSyncResponse {
    ///
    pub bank_transfer_events: Vec<BankTransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferEventSyncResponse {
    ///
    pub transfer_events: Vec<TransferEvent>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Identifier of the sweep.
    pub sweep_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a sweep.
    pub sweep_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepGetResponse {
    ///BankTransferSweep describes a sweep transfer.
    pub sweep: BankTransferSweep,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepGetResponse {
    /**Describes a sweep of funds to / from the sweep account.

A sweep is associated with many sweep events (events of type `swept` or `reverse_swept`) which can be retrieved by invoking the `/transfer/event/list` endpoint with the corresponding `sweep_id`.

`swept` events occur when the transfer amount is credited or debited from your sweep account, depending on the `type` of the transfer. `reverse_swept` events occur when a transfer is reversed and Plaid undoes the credit or debit.

The total sum of the `swept` and `reverse_swept` events is equal to the `amount` of the sweep Plaid creates and matches the amount of the entry on your sweep account ledger.*/
    pub sweep: TransferSweep,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account that the sweeps belong to.
    pub origination_account_id: Option<String>,
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_time: Option<String>,
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_time: Option<String>,
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of sweeps to return (RFC 3339 format).
    pub start_date: Option<String>,
    ///The end datetime of sweeps to return (RFC 3339 format).
    pub end_date: Option<String>,
    ///The maximum number of sweeps to return.
    pub count: Option<i64>,
    ///The number of sweeps to skip before returning results.
    pub offset: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweepListResponse {
    ///
    pub sweeps: Vec<TransferSweep>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweepListResponse {
    ///
    pub sweeps: Vec<BankTransferSweep>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferSweep {
    ///Identifier of the sweep.
    pub id: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created_at: String,
    ///The amount of the sweep.
    pub amount: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferSweep {
    ///Identifier of the sweep.
    pub id: String,
    ///The datetime when the sweep occurred, in RFC 3339 format.
    pub created: String,
    /**Signed decimal amount of the sweep as it appears on your sweep account ledger (e.g. "-10.00")

If amount is not present, the sweep was net-settled to zero and outstanding debits and credits between the sweep account and Plaid are balanced.*/
    pub amount: String,
    ///The currency of the sweep, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SimulatedTransferSweep {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///If multiple origination accounts are available, `origination_account_id` must be used to specify the account for which balance will be returned.
    pub origination_account_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalanceGetResponse {
    ///Information about the balance of a bank transfer
    pub balance: BankTransferBalance,
    ///The ID of the origination account that this balance belongs to.
    pub origination_account_id: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferBalance {
    ///The total available balance - the sum of all successful debit transfer amounts minus all credit transfer amounts.
    pub available: String,
    ///The transactable balance shows the amount in your account that you are able to use for transfers, and is essentially your available balance minus your minimum balance.
    pub transactable: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The user's account number.
    pub account_number: String,
    ///The user's routing number.
    pub routing_number: String,
    ///The type of the bank account (`checking` or `savings`).
    pub account_type: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankTransferMigrateAccountResponse {
    ///The Plaid `access_token` for the newly created Item.
    pub access_token: String,
    ///The Plaid `account_id` for the newly created Item.
    pub account_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The start datetime of repayments to return (RFC 3339 format).
    pub start_date: Option<String>,
    ///The end datetime of repayments to return (RFC 3339 format).
    pub end_date: Option<String>,
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    ///The number of repayments to skip before returning results.
    pub offset: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentListResponse {
    ///
    pub repayments: Vec<TransferRepayment>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepayment {
    ///Identifier of the repayment.
    pub repayment_id: String,
    ///The datetime when the repayment occurred, in RFC 3339 format.
    pub created: String,
    ///Decimal amount of the repayment as it appears on your account ledger.
    pub amount: String,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Identifier of the repayment to query.
    pub repayment_id: String,
    ///The maximum number of repayments to return.
    pub count: Option<i64>,
    ///The number of repayments to skip before returning results.
    pub offset: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturnListResponse {
    ///
    pub repayment_returns: Vec<TransferRepaymentReturn>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRepaymentReturn {
    ///The unique identifier of the guaranteed transfer that was returned.
    pub transfer_id: String,
    ///The unique identifier of the corresponding `reversed` transfer event.
    pub event_id: i64,
    ///The value of the returned transfer.
    pub amount: String,
    ///The currency of the repayment, e.g. "USD".
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: APIClientID,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: APISecret,
    ///The Plaid `account_id` for the account that will be debited or credited.
    pub account_id: Option<String>,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: Option<String>,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInRequest,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: TransferMetadata,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreate {
    ///Plaid's unique identifier for the transfer intent object.
    pub id: String,
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: String,
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    ///Plaid’s unique identifier for the origination account for the intent. If not provided, the default account will be used.
    pub origination_account_id: String,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: TransferMetadata,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentCreateResponse {
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentCreate,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: APIClientID,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: APISecret,
    ///Plaid's unique identifier for a transfer intent object.
    pub transfer_intent_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGet {
    ///Plaid's unique identifier for a transfer intent object.
    pub id: String,
    ///The datetime the transfer was created. This will be of the form `2006-01-02T15:04:05Z`.
    pub created: String,
    /**The status of the transfer intent.

- `PENDING` – The transfer intent is pending.
- `SUCCEEDED` – The transfer intent was successfully created.
- `FAILED` – The transfer intent was unable to be created.*/
    pub status: String,
    ///Plaid's unique identifier for the transfer created through the UI. Returned only if the transfer was successfully created. Null value otherwise.
    pub transfer_id: Option<String>,
    ///The reason for a failed transfer intent. Returned only if the transfer intent status is `failed`. Null otherwise.
    pub failure_reason: TransferIntentGetFailureReason,
    /**
A decision regarding the proposed transfer.

`APPROVED` – The proposed transfer has received the end user's consent and has been approved for processing. Plaid has also reviewed the proposed transfer and has approved it for processing. 

`PERMITTED` – Plaid was unable to fetch the information required to approve or decline the proposed transfer. You may proceed with the transfer, but further review is recommended. Plaid is awaiting further instructions from the client.

`DECLINED` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details. Null value otherwise.*/
    pub authorization_decision: Option<String>,
    ///The rationale for Plaid's decision regarding a proposed transfer. Will be null for `approved` decisions.
    pub authorization_decision_rationale: TransferAuthorizationDecisionRationale,
    ///The Plaid `account_id` for the account that will be debited or credited. Returned only if `account_id` was set on intent creation.
    pub account_id: Option<String>,
    ///Plaid’s unique identifier for the origination account used for the transfer.
    pub origination_account_id: String,
    ///The amount of the transfer (decimal string with two digits of precision e.g. "10.00").
    pub amount: TransferAmount,
    /**The direction of the flow of transfer funds.

- `PAYMENT` – Transfers funds from an end user's account to your business account.

- `DISBURSEMENT` – Transfers funds from your business account to an end user's account.*/
    pub mode: TransferIntentCreateMode,
    /**Specifies the use case of the transfer.  Required for transfers on an ACH network. In Sandbox, only `ccd`, `ppd`, or `web` can be used.

`"arc"` - Accounts Receivable Entry

`"cbr`" - Cross Border Entry

`"ccd"` - Corporate Credit or Debit - fund transfer between two corporate bank accounts

`"cie"` - Customer Initiated Entry

`"cor"` - Automated Notification of Change

`"ctx"` - Corporate Trade Exchange

`"iat"` - International

`"mte"` - Machine Transfer Entry

`"pbr"` - Cross Border Entry

`"pop"` - Point-of-Purchase Entry

`"pos"` - Point-of-Sale Entry

`"ppd"` - Prearranged Payment or Deposit - the transfer is part of a pre-existing relationship with a consumer, eg. bill payment

`"rck"` - Re-presented Check Entry

`"tel"` - Telephone-Initiated Entry

`"web"` - Internet-Initiated Entry - debits from a consumer’s account where their authorization is obtained over the Internet*/
    pub ach_class: ACHClass,
    ///The legal name and other information for the account holder.
    pub user: TransferUserInResponse,
    ///A description for the underlying transfer. Maximum of 8 characters.
    pub description: String,
    /**The Metadata object is a mapping of client-provided string fields to any string value. The following limitations apply:
- The JSON values must be Strings (no nested JSON objects allowed)
- Only ASCII characters may be used
- Maximum of 50 key/value pairs
- Maximum key length of 40 characters
- Maximum value length of 500 characters
*/
    pub metadata: TransferMetadata,
    ///The currency of the transfer amount, e.g. "USD"
    pub iso_currency_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransferIntentGetResponse {
    ///Represents a transfer intent within Transfer UI.
    pub transfer_intent: TransferIntentGet,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a bank transfer.
    pub bank_transfer_id: BankTransferID,
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `reversed`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `reversed`
*/
    pub event_type: String,
    ///The failure reason if the type of this transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: BankTransferFailure,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Plaid’s unique identifier for a transfer.
    pub transfer_id: TransferID,
    /**The asynchronous event to be simulated. May be: `posted`, `failed`, or `reversed`.

An error will be returned if the event type is incompatible with the current transfer status. Compatible status --> event type transitions include:

`pending` --> `failed`

`pending` --> `posted`

`posted` --> `reversed`
*/
    pub event_type: String,
    ///The failure reason if the event type for a transfer is `"failed"` or `"reversed"`. Null value otherwise.
    pub failure_reason: TransferFailure,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferSweepSimulateResponse {
    /**A sweep returned from the `/sandbox/transfer/sweep/simulate` endpoint.
Can be null if there are no transfers to include in a sweep.*/
    pub sweep: Option<SimulatedTransferSweep>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxTransferRepaymentSimulateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFiltersResponse {
    ///A filter to apply to `depository`-type accounts
    pub depository: Option<DepositoryFilter>,
    ///A filter to apply to `credit`-type accounts
    pub credit: Option<CreditFilter>,
    ///A filter to apply to `loan`-type accounts
    pub loan: Option<LoanFilter>,
    ///A filter to apply to `investment`-type accounts (or `brokerage`-type acconunts for API versions 2018-05-22 and earlier).
    pub investment: Option<InvestmentFilter>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionsSearchAccountFilter {
    ///
    pub loan: Option<Vec<AccountSubtype>>,
    ///
    pub depository: Option<Vec<AccountSubtype>>,
    ///
    pub credit: Option<Vec<AccountSubtype>>,
    ///
    pub investment: Option<Vec<AccountSubtype>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountIdentity(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAssets(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: DepositoryAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: CreditAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: LoanAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentFilter {
    ///An array of account subtypes to display in Link. If not specified, all account subtypes will be shown. For a full list of valid types and subtypes, see the [Account schema](https://plaid.com/docs/api/accounts#account-type-schema). 
    pub account_subtypes: InvestmentAccountSubtypes,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccountSubtypes(pub Vec<DepositoryAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccountSubtypes(pub Vec<CreditAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccountSubtypes(pub Vec<LoanAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtypes(pub Vec<InvestmentAccountSubtype>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositoryAccountSubtype(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditAccountSubtype(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAccountSubtype(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvestmentAccountSubtype(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The employer name to be searched for.
    pub query: String,
    ///The Plaid products the returned employers should support. Currently, this field must be set to `"deposit_switch"`.
    pub products: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersSearchResponse {
    ///A list of employers matching the search criteria.
    pub employers: Vec<Employer>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Employer {
    ///Plaid's unique identifier for the employer.
    pub employer_id: String,
    ///The name of the employer
    pub name: String,
    ///Data about the components comprising an address.
    pub address: AddressDataNullable,
    ///A number from 0 to 1 indicating Plaid's level of confidence in the pairing between the employer and the institution (not yet implemented).
    pub confidence_score: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The URL endpoint to which Plaid should send webhooks related to the progress of the income verification process.
    pub webhook: String,
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow.
    pub precheck_id: Option<String>,
    ///Optional arguments for `/income/verification/create`
    pub options: Option<IncomeVerificationCreateRequestOptions>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateRequestOptions {
    ///An array of access tokens corresponding to the Items that will be cross-referenced with the product data. Plaid will attempt to correlate transaction history from these Items with data from the user's paystub, such as date and amount. The `verification` status of the paystub as returned by `/income/verification/paystubs/get` will indicate if the verification status was successful, or, if not, why it failed. If the `transactions` product was not initialized for the Items during Link, it will be initialized after this Link session.
    pub access_tokens: Option<Vec<AccessToken>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationCreateResponse {
    ///ID of the verification. This ID is persisted throughout the lifetime of the verification.
    pub income_verification_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Information about the user whose eligibility is being evaluated.
    pub user: IncomeVerificationPrecheckUser,
    ///Information about the end user's employer
    pub employer: IncomeVerificationPrecheckEmployer,
    ///
    pub transactions_access_token: Option<serde_json::Value>,
    ///An array of access tokens corresponding to Items belonging to the user whose eligibility is being checked. Note that if the Items specified here are not already initialized with `transactions`, providing them in this field will cause these Items to be initialized with (and billed for) the Transactions product.
    pub transactions_access_tokens: Option<Vec<AccessToken>>,
    ///Data about military info in the income verification precheck.
    pub us_military_info: IncomeVerificationPrecheckMilitaryInfo,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployer {
    ///The employer's name
    pub name: Option<String>,
    ///The address of the employer
    pub address: IncomeVerificationPrecheckEmployerAddress,
    ///The employer's tax id
    pub tax_id: Option<String>,
    ///The URL for the employer's public website
    pub url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddress {}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckEmployerAddressData {
    ///The full city name
    pub city: Option<String>,
    /**The region or state. In API versions 2018-05-22 and earlier, this field is called `state`.
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    ///The postal code. In API versions 2018-05-22 and earlier, this field is called `zip`.
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckMilitaryInfo {
    ///Is the user currently active duty in the US military
    pub is_active_duty: Option<bool>,
    ///If the user is currently serving in the US military, the branch of the military they are serving in
    pub branch: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckUser {
    ///The user's first name
    pub first_name: Option<String>,
    ///The user's last name
    pub last_name: Option<String>,
    ///The user's email address
    pub email_address: Option<String>,
    ///Data about the components comprising an address.
    pub home_address: SignalAddressData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckResponse {
    ///ID of the precheck. Provide this value when calling `/link/token/create` in order to optimize Link conversion.
    pub precheck_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
    /**The confidence that Plaid can support the user in the digital income verification flow instead of requiring a manual paystub upload. One of the following:

`"HIGH"`: It is very likely that this user can use the digital income verification flow.

"`LOW`": It is unlikely that this user can use the digital income verification flow.

`"UNKNOWN"`: It was not possible to determine if the user is supportable with the information passed.*/
    pub confidence: IncomeVerificationPrecheckConfidence,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPrecheckConfidence(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LinkTokenCreateRequestIncomeVerification {
    ///The `income_verification_id` of the verification instance, as provided by `/income/verification/create`.
    pub income_verification_id: Option<String>,
    ///The `asset_report_id` of an asset report associated with the user, as provided by `/asset_report/create`. Providing an `asset_report_id` is optional and can be used to verify the user through a streamlined flow. If provided, the bank linking flow will be skipped.
    pub asset_report_id: Option<String>,
    ///The ID of a precheck created with `/income/verification/precheck`. Will be used to improve conversion of the income verification flow by streamlining the Link interface presented to the end user.
    pub precheck_id: Option<String>,
    ///An array of access tokens corresponding to the Items that will be cross-referenced with the product data. If the `transactions` product was not initialized for the Items during link, it will be initialized after this Link session.
    pub access_tokens: Option<Vec<AccessToken>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationStatusWebhook {
    ///`"INCOME"`
    pub webhook_type: String,
    ///`income_verification`
    pub webhook_code: String,
    ///The `income_verification_id` of the verification instance whose status is being reported.
    pub income_verification_id: String,
    ///The Item ID associated with the verification.
    pub item_id: String,
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationSummaryGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationSummaryGetResponse {
    ///A list of income summaries.
    pub income_summaries: Vec<IncomeSummary>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationRefreshResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
    /**The verification refresh status. One of the following:

`"VERIFICATION_REFRESH_STATUS_USER_PRESENCE_REQUIRED"` User presence is required to refresh an income verification.
`"VERIFICATION_REFRESH_SUCCESSFUL"` The income verification refresh was successful.
`"VERIFICATION_REFRESH_NOT_FOUND"` No new data was found after the income verification refresh.*/
    pub verification_refresh_status: VerificationRefreshStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummary {
    ///The name of the employer, as reported on the paystub.
    pub employer_name: EmployerIncomeSummaryFieldString,
    ///The name of the employee, as reported on the paystub.
    pub employee_name: EmployeeIncomeSummaryFieldString,
    ///Year-to-date pre-tax earnings, as reported on the paystub.
    pub ytd_gross_income: YTDGrossIncomeSummaryFieldNumber,
    ///Year-to-date earnings after any tax withholdings, benefit payments or deductions, as reported on the paystub.
    pub ytd_net_income: YTDNetIncomeSummaryFieldNumber,
    ///The frequency of the pay period.
    pub pay_frequency: PayFrequency,
    ///The employee's estimated annual salary, as derived from information reported on the paystub.
    pub projected_wage: ProjectedIncomeSummaryFieldNumber,
    ///Information about the matched direct deposit transaction used to verify a user's payroll information.
    pub verified_transaction: TransactionData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
    ///The description of the transaction.
    pub description: String,
    ///The amount of the transaction.
    pub amount: f64,
    ///The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub date: String,
    ///A unique identifier for the end user's account.
    pub account_id: String,
    ///A unique identifier for the transaction.
    pub transaction_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldString {
    ///The value of the field.
    pub value: String,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeIncomeSummaryFieldString(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSummaryFieldNumber {
    ///The value of the field.
    pub value: f64,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct YTDGrossIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct YTDNetIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectedIncomeSummaryFieldNumber(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PayFrequency {
    ///The frequency of the pay period.
    pub value: PayFrequencyValue,
    /**The verification status. One of the following:

`"VERIFIED"`: The information was successfully verified.

`"UNVERIFIED"`: The verification has not yet been performed.

`"NEEDS_INFO"`: The verification was attempted but could not be completed due to missing information.

"`UNABLE_TO_VERIFY`": The verification was performed and the information could not be verified.

`"UNKNOWN"`: The verification status is unknown.*/
    pub verification_status: VerificationStatus,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayFrequencyValue(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRefreshStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification for which to get paystub information.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubGetResponse {
    ///An object representing data extracted from the end user's paystub.
    pub paystub: Paystub,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification for which to get paystub information.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationPaystubsGetResponse {
    ///Metadata for an income document.
    pub document_metadata: Option<Vec<DocumentMetadata>>,
    ///
    pub paystubs: Vec<Paystub>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    ///The name of the document.
    pub name: Option<String>,
    ///The processing status of the document.
    pub status: Option<String>,
    ///An identifier of the document that is also present in the paystub response.
    pub doc_id: Option<String>,
    /**The type of document.

`DOCUMENT_TYPE_PAYSTUB`: A paystub.

`DOCUMENT_TYPE_BANK_STATEMENT`: A bank statement.

`DOCUMENT_TYPE_US_TAX_W2`: A W-2 wage and tax statement provided by a US employer reflecting wages earned by the employee.

`DOCUMENT_TYPE_US_MILITARY_ERAS`: An electronic Retirement Account Statement (eRAS) issued by the US military.

`DOCUMENT_TYPE_US_MILITARY_LES`: A Leave and Earnings Statement (LES) issued by the US military.

`DOCUMENT_TYPE_US_MILITARY_CLES`: A Civilian Leave and Earnings Statment (CLES) issued by the US military.

`DOCUMENT_TYPE_GIG`: Used to indicate that the income is related to gig work. Does not necessarily correspond to a specific document type.

`DOCUMENT_TYPE_NONE`: Used to indicate that there is no underlying document for the data.

`UNKNOWN`: Document type could not be determined.*/
    pub doc_type: Option<DocType>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Paystub {
    ///An object with the deduction information found on a paystub.
    pub deductions: Deductions,
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: String,
    ///An object representing both a breakdown of earnings on a paystub and the total earnings.
    pub earnings: Earnings,
    ///Data about the employee.
    pub employee: Employee,
    ///Information about the employer on the paystub
    pub employer: PaystubEmployer,
    ///An object representing employment details found on a paystub.
    pub employment_details: Option<EmploymentDetails>,
    ///An object representing information about the net pay amount on the paystub.
    pub net_pay: NetPay,
    ///Details about the pay period.
    pub pay_period_details: PayPeriodDetails,
    ///An object representing details that can be found on the paystub.
    pub paystub_details: Option<PaystubDetails>,
    ///
    pub income_breakdown: Option<Vec<IncomeBreakdown>>,
    ///The amount of income earned year to date, as based on paystub data.
    pub ytd_earnings: Option<PaystubYTDDetails>,
    ///An object containing details on the paystub's verification status. This object will only be populated if the [`income_verification.access_tokens`](/docs/api/tokens/#link-token-create-request-income-verification-access-tokens) parameter was provided during the `/link/token/create` call or if a problem was detected with the information supplied by the user; otherwise it will be `null`.
    pub verification: PaystubVerification,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Deductions {
    ///
    pub subtotals: Option<Vec<Total>>,
    ///
    pub breakdown: Vec<DeductionsBreakdown>,
    ///
    pub totals: Option<Vec<Total>>,
    ///An object representing the total deductions for the pay period
    pub total: DeductionsTotal,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsBreakdown {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///Description of the deduction line item
    pub description: Option<String>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DeductionsTotal {
    ///Raw amount of the deduction
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date total amount of the deductions
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Total {
    ///Commonly used term to describe the line item.
    pub canonical_description: TotalCanonicalDescription,
    ///Text of the line item as printed on the paystub.
    pub description: Option<String>,
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
    ///An object representing a monetary amount.
    pub ytd_pay: Option<Pay>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TotalCanonicalDescription(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Pay {
    ///A numerical amount of a specific currency.
    pub amount: Option<f64>,
    ///Currency code, e.g. USD
    pub currency: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Earnings {
    ///
    pub subtotals: Option<Vec<EarningsTotal>>,
    ///
    pub totals: Option<Vec<EarningsTotal>>,
    ///
    pub breakdown: Option<Vec<EarningsBreakdown>>,
    ///An object representing both the current pay period and year to date amount for an earning category.
    pub total: Option<EarningsTotal>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsBreakdown {
    ///Commonly used term to describe the earning line item.
    pub canonical_description: EarningsBreakdownCanonicalDescription,
    ///Raw amount of the earning line item.
    pub current_amount: Option<f64>,
    ///Description of the earning line item.
    pub description: Option<String>,
    ///Number of hours applicable for this earning.
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///Hourly rate applicable for this earning.
    pub rate: Option<f64>,
    /**The unofficial currency code associated with the line item. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the deduction.
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsBreakdownCanonicalDescription(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct EarningsTotal {
    ///Total amount of the earnings for this pay period
    pub current_amount: Option<f64>,
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
    ///An object representing a monetary amount.
    pub ytd_pay: Option<Pay>,
    ///Total number of hours worked for this pay period
    pub hours: Option<f64>,
    ///The ISO-4217 currency code of the line item. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the security. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The total year-to-date amount of the earnings
    pub ytd_amount: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentDetails {
    ///An object representing a monetary amount.
    pub annual_salary: Option<Pay>,
    ///Date on which the employee was hired, in the YYYY-MM-DD format.
    pub hire_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NetPay {
    ///Raw amount of the net pay for the pay period
    pub current_amount: Option<f64>,
    ///Description of the net pay
    pub description: Option<String>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///The year-to-date amount of the net pay
    pub ytd_amount: Option<f64>,
    ///An object representing both the current pay period and year to date amount for a category.
    pub total: Option<Total>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDetails {
    ///Beginning date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_start_date: Option<String>,
    ///Ending date of the pay period on the paystub in the 'YYYY-MM-DD' format.
    pub pay_period_end_date: Option<String>,
    ///Pay date on the paystub in the 'YYYY-MM-DD' format.
    pub pay_date: Option<String>,
    ///The name of the payroll provider that generated the paystub, e.g. ADP
    pub paystub_provider: Option<String>,
    ///The frequency at which the employee is paid. Possible values: `MONTHLY`, `BI-WEEKLY`, `WEEKLY`, `SEMI-MONTHLY`.
    pub pay_frequency: PaystubPayFrequency,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubPayFrequency(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeBreakdown {
    #[serde(rename = "type")]
    /**The type of income. Possible values include:
  `"regular"`: regular income
  `"overtime"`: overtime income
  `"bonus"`: bonus income*/
    pub type_: IncomeBreakdownType,
    ///The hourly rate at which the income is paid.
    pub rate: Option<f64>,
    ///The number of hours logged for this income for this pay period.
    pub hours: Option<f64>,
    ///The total pay for this pay period.
    pub total: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeBreakdownType(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    ///Address on the paystub
    pub address: PaystubAddress,
    ///The name of the employee.
    pub name: Option<String>,
    ///Marital status of the employee - either `single` or `married`.
    pub marital_status: Option<String>,
    ///Taxpayer ID of the individual receiving the paystub.
    pub taxpayer_id: Option<TaxpayerID>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxpayerID {
    ///Type of ID, e.g. 'SSN'
    pub id_type: Option<String>,
    ///ID mask; i.e. last 4 digits of the taxpayer ID
    pub id_mask: Option<String>,
    ///Last 4 digits of unique number of ID.
    pub last_4_digits: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubEmployer {
    ///Address on the paystub
    pub address: Option<PaystubAddress>,
    ///The name of the employer on the paystub.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubAddress {
    ///The full city name.
    pub city: Option<String>,
    ///The ISO 3166-1 alpha-2 country code.
    pub country: Option<String>,
    ///The postal code of the address.
    pub postal_code: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    ///The full street address.
    pub street: Option<String>,
    ///Street address line 1.
    pub line1: Option<String>,
    ///Street address line 2.
    pub line2: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub state_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PayPeriodDetails {
    ///The amount of the paycheck.
    pub check_amount: Option<f64>,
    ///
    pub distribution_breakdown: Option<Vec<DistributionBreakdown>>,
    ///The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub end_date: Option<String>,
    ///Total earnings before tax/deductions.
    pub gross_earnings: Option<f64>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_date: Option<String>,
    ///The frequency at which an individual is paid.
    pub pay_frequency: Option<String>,
    ///The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format ("yyyy-mm-dd").
    pub pay_day: Option<String>,
    ///The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: "yyyy-mm-dd".
    pub start_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DistributionBreakdown {
    ///Name of the account for the given distribution.
    pub account_name: Option<String>,
    ///The name of the bank that the payment is being deposited to.
    pub bank_name: Option<String>,
    ///The amount distributed to this account.
    pub current_amount: Option<f64>,
    ///The ISO-4217 currency code of the net pay. Always `null` if `unofficial_currency_code` is non-null.
    pub iso_currency_code: Option<String>,
    ///The last 2-4 alphanumeric characters of an account's official account number.
    pub mask: Option<String>,
    #[serde(rename = "type")]
    ///Type of the account that the paystub was sent to (e.g. 'checking').
    pub type_: Option<String>,
    /**The unofficial currency code associated with the net pay. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.

See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s.*/
    pub unofficial_currency_code: Option<String>,
    ///An object representing a monetary amount.
    pub current_pay: Option<Pay>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubDeduction {
    #[serde(rename = "type")]
    ///The description of the deduction, as provided on the paystub. For example: `"401(k)"`, `"FICA MED TAX"`.
    pub type_: Option<String>,
    ///`true` if the deduction is pre-tax; `false` otherwise.
    pub is_pretax: Option<bool>,
    ///The amount of the deduction.
    pub total: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubYTDDetails {
    ///Year-to-date gross earnings.
    pub gross_earnings: Option<f64>,
    ///Year-to-date net (take home) earnings.
    pub net_earnings: Option<f64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubVerification {
    ///Derived verification status.
    pub verification_status: PaystubVerificationStatus,
    ///
    pub verification_attributes: Vec<VerificationAttribute>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaystubVerificationStatus(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationAttribute {
    #[serde(rename = "type")]
    ///Message indicating the reason as to why the verification failed
    pub type_: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationDocumentsDownloadRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
    ///The document ID to download. If passed, a single document will be returned in the resulting zip file, rather than all document
    pub document_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification.
    pub income_verification_id: Option<String>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationTaxformsGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<RequestID>,
    ///
    pub document_metadata: Vec<DocumentMetadata>,
    ///A list of forms.
    pub taxforms: Vec<Taxform>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Taxform {
    ///An identifier of the document referenced by the document metadata.
    pub doc_id: Option<String>,
    ///The type of tax document. Currently, the only supported value is `w2`.
    pub document_type: String,
    ///W2 is an object that represents income data taken from a W2 tax document.
    pub w2: Option<W2>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2 {
    ///Information about the employer on the paystub
    pub employer: Option<PaystubEmployer>,
    ///Data about the employee.
    pub employee: Option<Employee>,
    ///The tax year of the W2 document.
    pub tax_year: Option<String>,
    ///An employee identification number or EIN.
    pub employer_id_number: Option<String>,
    ///Wages from tips and other compensation.
    pub wages_tips_other_comp: Option<String>,
    ///Federal income tax withheld for the tax year.
    pub federal_income_tax_withheld: Option<String>,
    ///Wages from social security.
    pub social_security_wages: Option<String>,
    ///Social security tax withheld for the tax year.
    pub social_security_tax_withheld: Option<String>,
    ///Wages and tips from medicare.
    pub medicare_wages_and_tips: Option<String>,
    ///Medicare tax withheld for the tax year.
    pub medicare_tax_withheld: Option<String>,
    ///Tips from social security.
    pub social_security_tips: Option<String>,
    ///Allocated tips.
    pub allocated_tips: Option<String>,
    ///Contents from box 9 on the W2.
    pub box_9: Option<String>,
    ///Dependent care benefits.
    pub dependent_care_benefits: Option<String>,
    ///Nonqualified plans.
    pub nonqualified_plans: Option<String>,
    ///
    pub box_12: Option<Vec<W2Box12>>,
    ///Statutory employee.
    pub statutory_employee: Option<String>,
    ///Retirement plan.
    pub retirement_plan: Option<String>,
    ///Third party sick pay.
    pub third_party_sick_pay: Option<String>,
    ///Other.
    pub other: Option<String>,
    ///
    pub state_and_local_wages: Option<Vec<W2StateAndLocalWages>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2Box12 {
    ///W2 Box 12 code.
    pub code: Option<String>,
    ///W2 Box 12 amount.
    pub amount: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2StateAndLocalWages {
    ///State associated with the wage.
    pub state: Option<String>,
    ///State identification number of the employer.
    pub employer_state_id_number: Option<String>,
    ///Wages and tips from the specified state.
    pub state_wages_tips: Option<String>,
    ///Income tax from the specified state.
    pub state_income_tax: Option<String>,
    ///Wages and tips from the locality.
    pub local_wages_tips: Option<String>,
    ///Income tax from the locality.
    pub local_income_tax: Option<String>,
    ///Name of the locality.
    pub locality_name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeVerificationWebhookStatus {
    ///
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationGetResponse {
    ///A list of employment verification summaries.
    pub employments: Vec<EmploymentVerification>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerification {
    ///Current employment status.
    pub status: EmploymentVerificationStatus,
    ///Start of employment in ISO 8601 format (YYYY-MM-DD).
    pub start_date: Option<String>,
    ///End of employment, if applicable. Provided in ISO 8601 format (YYY-MM-DD).
    pub end_date: Option<String>,
    ///An object containing employer data.
    pub employer: Option<EmployerVerification>,
    ///Current title of employee.
    pub title: Option<String>,
    ///An object containing a set of ids related to an employee
    pub platform_ids: Option<PlatformIds>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentVerificationStatus(pub Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerVerification {
    ///Name of employer.
    pub name: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformIds {
    ///The ID of an employee as given by their employer
    pub employee_id: Option<String>,
    ///The ID of an employee as given by their payroll
    pub payroll_id: Option<String>,
    ///The ID of the position of the employee
    pub position_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetReportTransaction(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthIncident {
    ///The start date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub start_date: String,
    ///The end date of the incident, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub end_date: Option<String>,
    ///The title of the incident
    pub title: String,
    ///Updates on the health incident.
    pub incident_updates: Vec<IncidentUpdate>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncidentUpdate {
    ///The content of the update.
    pub description: Option<String>,
    ///The status of the incident.
    pub status: Option<String>,
    ///The date when the update was published, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format, e.g. `"2020-10-30T15:26:48Z"`.
    pub updated_date: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The deposit switch destination account
    pub target_account: DepositSwitchTargetAccount,
    ///The deposit switch target user
    pub target_user: DepositSwitchTargetUser,
    ///Options to configure the `/deposit_switch/create` request. If provided, cannot be `null`.
    pub options: Option<DepositSwitchCreateRequestOptions>,
    ///ISO-3166-1 alpha-2 country code standard.
    pub country_code: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAltCreateResponse {
    ///ID of the deposit switch. This ID is persisted throughout the lifetime of the deposit switch.
    pub deposit_switch_id: String,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetAccount {
    ///Account number for deposit switch destination
    pub account_number: String,
    ///Routing number for deposit switch destination
    pub routing_number: String,
    ///The name of the deposit switch destination account, as it will be displayed to the end user in the Deposit Switch interface. It is not required to match the name used in online banking.
    pub account_name: String,
    ///The account subtype of the account, either `checking` or `savings`.
    pub account_subtype: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchTargetUser {
    ///The given name (first name) of the user.
    pub given_name: String,
    ///The family name (last name) of the user.
    pub family_name: String,
    ///The phone number of the user. The endpoint can accept a variety of phone number formats, including E.164.
    pub phone: String,
    ///The email address of the user.
    pub email: String,
    ///The user's address.
    pub address: Option<DepositSwitchAddressData>,
    ///The taxpayer ID of the user, generally their SSN, EIN, or TIN.
    pub tax_payer_id: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositSwitchAddressData {
    ///The full city name
    pub city: String,
    /**The region or state
Example: `"NC"`*/
    pub region: String,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: String,
    ///The postal code
    pub postal_code: String,
    ///The ISO 3166-1 alpha-2 country code
    pub country: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The URL to which the webhook should be sent.
    pub webhook: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxBankTransferFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationID(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Application {
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationID,
    ///The name of the application
    pub name: String,
    ///The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub created_at: Option<String>,
    ///The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub join_date: String,
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    ///The URL for the application's website
    pub application_url: Option<String>,
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: APIClientID,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: APISecret,
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationGetResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
    ///Metadata about the application
    pub application: Application,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductAccess {
    ///Allow access to statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    ///Allow access to the Identity product (name, email, phone, address). Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub identity: Option<bool>,
    ///Allow access to account number details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub auth: Option<bool>,
    ///Allow access to transaction details. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub transactions: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountAccess {
    ///The unique account identifier for this account. This value must match that returned by the data access API for this account.
    pub unique_id: String,
    ///Allow the application to see this account (and associated details, including balance) in the list of accounts  If unset, defaults to `true`.
    pub authorized: Option<bool>,
    ///Allow the application to access specific products on this account
    pub account_product_access: AccountProductAccessNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccessNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountProductAccess {
    ///Allow the application to access account data. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub account_data: Option<bool>,
    ///Allow the application to access bank statements. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub statements: Option<bool>,
    ///Allow the application to access tax documents. Only used by certain partners. If relevant to the partner and unset, defaults to `true`.
    pub tax_documents: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesNullable(pub Option<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Scopes {
    ///The product access being requested. Used to or disallow product access across all accounts. If unset, defaults to all products allowed.
    pub product_access: Option<ProductAccess>,
    ///
    pub accounts: Option<Vec<AccountAccess>>,
    ///Allow access to newly opened accounts as they are opened. If unset, defaults to `true`.
    pub new_accounts: Option<bool>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestedScopes {
    ///Enumerates the account subtypes that the application wishes for the user to be able to select from. For more details refer to Plaid documentation on account filters.
    pub account_filters: Option<AccountFilter>,
    /**The application requires that accounts be limited to a specific cardinality.
`MULTI_SELECT`: indicates that the user should be allowed to pick multiple accounts.
`SINGLE_SELECT`: indicates that the user should be allowed to pick only a single account.
`ALL`: indicates that the user must share all of their accounts and should not be given the opportunity to de-select*/
    pub account_selection_cardinality: AccountSelectionCardinality,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesState(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ScopesContext(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationID,
    ///The scopes object
    pub scopes: Scopes,
    ///When scopes are updated during enrollment, this field must be populated with the state sent to the partner in the OAuth Login URI. This field is required when the context is `ENROLLMENT`.
    pub state: Option<ScopesState>,
    ///An indicator for when scopes are being updated. When scopes are updated via enrollment (i.e. OAuth), the partner must send `ENROLLMENT`. When scopes are updated in a post-enrollment view, the partner must send `PORTAL`.
    pub context: ScopesContext,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationScopesUpdateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessTokenNullable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: Option<RequestID>,
    ///A list of connected applications.
    pub applications: Vec<ConnectedApplication>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectedApplication {
    ///This field will map to the application ID that is returned from /item/applications/list, or provided to the institution in an oauth redirect.
    pub application_id: ApplicationID,
    ///The name of the application
    pub name: String,
    ///A URL that links to the application logo image (will be deprecated in the future, please use logo_url).
    pub logo: Option<String>,
    ///A URL that links to the application logo image.
    pub logo_url: Option<String>,
    ///The URL for the application's website
    pub application_url: Option<String>,
    ///A string provided by the connected app stating why they use their respective enabled products.
    pub reason_for_access: Option<String>,
    ///The date this application was linked in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub created_at: String,
    ///The date this application was granted production access at Plaid in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format in UTC.
    pub join_date: String,
    ///(Deprecated) A list of enums representing the data collected and products enabled for this connected application.
    pub product_data_types: Vec<String>,
    ///The scopes object
    pub scopes: ScopesNullable,
    ///Scope of required and optional account features or content from a ConnectedApplication.
    pub requested_scopes: Option<RequestedScopes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSelectionCardinality(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilter {
    ///A list of account subtypes to be filtered.
    pub depository: Option<AccountFilterSubtypes>,
    ///A list of account subtypes to be filtered.
    pub credit: Option<AccountFilterSubtypes>,
    ///A list of account subtypes to be filtered.
    pub loan: Option<AccountFilterSubtypes>,
    ///A list of account subtypes to be filtered.
    pub investment: Option<AccountFilterSubtypes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountFilterSubtypes(pub Vec<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the verification.
    pub income_verification_id: String,
    ///The Item ID associated with the verification.
    pub item_id: String,
    ///The URL to which the webhook should be sent.
    pub webhook: String,
    /**`VERIFICATION_STATUS_PROCESSING_COMPLETE`: The income verification status processing has completed. If the user uploaded multiple documents, this webhook will fire when all documents have finished processing. Call the `/income/verification/paystubs/get` endpoint and check the document metadata to see which documents were successfully parsed.

`VERIFICATION_STATUS_PROCESSING_FAILED`: A failure occurred when attempting to process the verification documentation.

`VERIFICATION_STATUS_PENDING_APPROVAL`: The income verification has been sent to the user for review.*/
    pub verification_status: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxIncomeFireWebhookResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemApplicationListUserAuth {
    ///Account username.
    pub user_id: Option<String>,
    ///Account username hashed by FI.
    pub fi_username_hash: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The access token associated with the Item data is being requested for.
    pub access_token: AccessToken,
    ///The `account_id` of the account whose verification status is to be modified
    pub account_id: String,
    ///The unique ID that you would like to use to refer to this transaction. For your convenience mapping your internal data, you could use your internal ID/identifier for this transaction. The max length for this field is 36 characters.
    pub client_transaction_id: String,
    ///The transaction amount, in USD (e.g. `102.05`)
    pub amount: f64,
    ///`true` if the end user is present while initiating the ACH transfer and the endpoint is being called; `false` otherwise (for example, when the ACH transfer is scheduled and the end user is not present, or you call this endpoint after the ACH transfer but before submitting the Nacha file for ACH processing).
    pub user_present: Option<bool>,
    ///A unique ID that identifies the end user in your system. This ID is used to correlate requests by a user with multiple Items. The max length for this field is 36 characters.
    pub client_user_id: Option<String>,
    ///Details about the end user initiating the transaction (i.e., the account holder).
    pub user: Option<SignalUser>,
    ///Details about the end user's device
    pub device: Option<SignalDevice>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalUser {
    ///The user's legal name
    pub name: SignalPersonName,
    ///The user's phone number, in E.164 format: +{countrycode}{number}. For example: "+14151234567"
    pub phone_number: Option<String>,
    ///The user's email address.
    pub email_address: Option<String>,
    ///Data about the components comprising an address.
    pub address: SignalAddressData,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalPersonName {
    ///The user's name prefix (e.g. "Mr.")
    pub prefix: Option<String>,
    ///The user's given name. If the user has a one-word name, it should be provided in this field.
    pub given_name: Option<String>,
    ///The user's middle name
    pub middle_name: Option<String>,
    ///The user's family name / surname
    pub family_name: Option<String>,
    ///The user's name suffix (e.g. "II")
    pub suffix: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalAddressData {
    ///The full city name
    pub city: Option<String>,
    /**The region or state
Example: `"NC"`*/
    pub region: Option<String>,
    /**The full street address
Example: `"564 Main Street, APT 15"`*/
    pub street: Option<String>,
    ///The postal code
    pub postal_code: Option<String>,
    ///The ISO 3166-1 alpha-2 country code
    pub country: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDevice {
    ///The IP address of the device that initiated the transaction
    pub ip_address: Option<String>,
    ///The user agent of the device that initiated the transaction (e.g. "Mozilla/5.0")
    pub user_agent: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
    ///Risk scoring details broken down by risk category.
    pub scores: SignalScores,
    /**The core attributes object contains additional data that can be used to assess the ACH return risk. Examples of data include:

`days_since_first_plaid_connection`: The number of days since the first time the Item was connected to an application via Plaid
`plaid_connections_count_7d`: The number of times the Item has been connected to applications via Plaid over the past 7 days
`plaid_connections_count_30d`: The number of times the Item has been connected to applications via Plaid over the past 30 days
`total_plaid_connections_count`: The number of times the Item has been connected to applications via Plaid
`is_savings_or_money_market_account`: Indicates whether the ACH transaction funding account is a savings/money market account

For the full list and detailed documentation of core attributes available, or to request that core attributes not be returned, contact Sales or your Plaid account manager*/
    pub core_attributes: Option<SignalEvaluateCoreAttributes>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScores {
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk of an unauthorized debit. Common return codes in this category include: "R05", "R07", "R10", "R11", "R29". These returns typically have a return time frame of up to 60 calendar days. During this period, the customer of financial institutions can dispute a transaction as unauthorized.
    pub customer_initiated_return_risk: Option<CustomerInitiatedReturnRisk>,
    ///The object contains a risk score and a risk tier that evaluate the transaction return risk because an account is overdrawn or because an ineligible account is used. Common return codes in this category include: "R01", "R02", "R03", "R04", "R06", "R08",  "R09", "R13", "R16", "R17", "R20", "R23". These returns have a turnaround time of 2 banking days.
    pub bank_initiated_return_risk: Option<BankInitiatedReturnRisk>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalScore(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInitiatedReturnRisk {
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: SignalScore,
    /**A tier corresponding to the projected likelihood that the transaction, if initiated, will be subject to a return.

In the `customer_initiated_return_risk` object, there are five risk tiers corresponding to the scores:
  1: Predicted customer-initiated return incidence rate between 0.00% - 0.02%
  2: Predicted customer-initiated return incidence rate between 0.02% - 0.05%
  3: Predicted customer-initiated return incidence rate between 0.05% - 0.1%
  4: Predicted customer-initiated return incidence rate between 0.1% - 0.5%
  5: Predicted customer-initiated return incidence rate greater than 0.5%
*/
    pub risk_tier: CustomerInitiatedRiskTier,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedRiskTier(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct BankInitiatedReturnRisk {
    ///A score from 0-99 that indicates the transaction return risk: a higher risk score suggests a higher return likelihood.
    pub score: SignalScore,
    /**In the `bank_initiated_return_risk` object, there are eight risk tiers corresponding to the scores:
  1: Predicted bank-initiated return incidence rate between 0.0% - 0.5%
  2: Predicted bank-initiated return incidence rate between 0.5% - 1.5%
  3: Predicted bank-initiated return incidence rate between 1.5% - 3%
  4: Predicted bank-initiated return incidence rate between 3% - 5%
  5: Predicted bank-initiated return incidence rate between 5% - 10%
  6: Predicted bank-initiated return incidence rate between 10% - 15%
  7: Predicted bank-initiated return incidence rate between 15% and 50%
  8: Predicted bank-initiated return incidence rate greater than 50%
*/
    pub risk_tier: BankInitiatedRiskTier,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalEvaluateCoreAttributes {
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 7 days from the account that will be debited.
    pub unauthorized_transactions_count_7d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 30 days from the account that will be debited.
    pub unauthorized_transactions_count_30d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 60 days from the account that will be debited.
    pub unauthorized_transactions_count_60d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to unauthorized transactions over the past 90 days from the account that will be debited.
    pub unauthorized_transactions_count_90d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 7 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_7d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 30 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_30d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 60 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_60d: Option<i64>,
    ///We parse and analyze historical transaction metadata to identify the number of possible past returns due to non-sufficient funds/overdrafts over the past 90 days from the account that will be debited.
    pub nsf_overdraft_transactions_count_90d: Option<i64>,
    ///The number of days since the first time the Item was connected to an application via Plaid
    pub days_since_first_plaid_connection: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 7 days
    pub plaid_connections_count_7d: Option<i64>,
    ///The number of times the Item has been connected to applications via Plaid over the past 30 days
    pub plaid_connections_count_30d: Option<i64>,
    ///The total number of times the Item has been connected to applications via Plaid
    pub total_plaid_connections_count: Option<i64>,
    ///Indicates if the ACH transaction funding account is a savings/money market account
    pub is_savings_or_money_market_account: Option<bool>,
    ///The total credit (inflow) transaction amount over the past 10 days from the account that will be debited
    pub total_credit_transactions_amount_10d: Option<f64>,
    ///The total debit (outflow) transaction amount over the past 10 days from the account that will be debited
    pub total_debit_transactions_amount_10d: Option<f64>,
    ///The 50th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p50_credit_transactions_amount_28d: Option<f64>,
    ///The 50th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p50_debit_transactions_amount_28d: Option<f64>,
    ///The 95th percentile of all credit (inflow) transaction amounts over the past 28 days from the account that will be debited
    pub p95_credit_transactions_amount_28d: Option<f64>,
    ///The 95th percentile of all debit (outflow) transaction amounts over the past 28 days from the account that will be debited
    pub p95_debit_transactions_amount_28d: Option<f64>,
    ///The number of days within the past 90 days when the account that will be debited had a negative end-of-day available balance
    pub days_with_negative_balance_count_90d: Option<i64>,
    ///The 90th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p90_eod_balance_30d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p90_eod_balance_60d: Option<f64>,
    ///The 90th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p90_eod_balance_90d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 30 days of the account that will be debited
    pub p10_eod_balance_30d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 60 days of the account that will be debited
    pub p10_eod_balance_60d: Option<f64>,
    ///The 10th percentile of the end-of-day available balance over the past 90 days of the account that will be debited
    pub p10_eod_balance_90d: Option<f64>,
    ///Available balance, as of the `balance_last_updated` time. The available balance is the current balance less any outstanding holds or debits that have not yet posted to the account.
    pub available_balance: Option<f64>,
    ///Current balance, as of the `balance_last_updated` time. The current balance is the total amount of funds in the account.
    pub current_balance: Option<f64>,
    ///Timestamp in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DDTHH:mm:ssZ) indicating the last time that the balance for the given account has been updated.
    pub balance_last_updated: Option<String>,
    ///The number of times the account's phone numbers on file have changed over the past 28 days
    pub phone_change_count_28d: Option<i64>,
    ///The number of times the account's phone numbers on file have changed over the past 90 days
    pub phone_change_count_90d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 28 days
    pub email_change_count_28d: Option<i64>,
    ///The number of times the account's email addresses on file have changed over the past 90 days
    pub email_change_count_90d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 28 days
    pub address_change_count_28d: Option<i64>,
    ///The number of times the account's addresses on file have changed over the past 90 days
    pub address_change_count_90d: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    ///`true` if the ACH transaction was initiated, `false` otherwise.
    pub initiated: bool,
    ///The actual number of days (hold time) since the ACH debit transaction that you wait before making funds available to your customers. The holding time could affect the ACH return rate. For example, use 0 if you make funds available to your customers instantly or the same day following the debit transaction, or 1 if you make funds available the next day following the debit initialization.
    pub days_funds_on_hold: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalDecisionReportResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///Must be the same as the `client_transaction_id` supplied when calling `/signal/evaluate`
    pub client_transaction_id: String,
    ///Must be a valid ACH return code (e.g. "R01")
    pub return_code: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignalReturnReportResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsRequest {
    ///
    pub oauth_state_id: String,
    ///
    pub accounts: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxOauthSelectAccountsResponse {}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountsAvailableWebhook {
    ///`ITEM`
    pub webhook_type: Option<String>,
    ///`NEW_ACCOUNTS_AVAILABLE`
    pub webhook_code: Option<String>,
    ///The `item_id` of the Item associated with this webhook, warning, or error
    pub item_id: Option<ItemId>,
    ///We use standard HTTP response codes for success and failure notifications, and our errors are further classified by `error_type`. In general, 200 HTTP codes correspond to success, 40X codes are for developer- or user-related failures, and 50X codes are for Plaid-related issues.  Error fields will be `null` if no error has occurred.
    pub error: Option<PlaidError>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the e-wallet
    pub wallet_id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletGetResponse {
    ///A unique ID identifying the e-wallet
    pub wallet_id: String,
    ///An object representing the e-wallet balance
    pub balance: WalletBalance,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletBalance {
    ///The ISO-4217 currency code of the balance
    pub iso_currency_code: String,
    ///The total amount of funds in the account
    pub current: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    /**A random key provided by the client, per unique wallet transaction. Maximum of 128 characters.

The API supports idempotency for safely retrying requests without accidentally performing the same operation twice. If a request to execute a wallet transaction fails due to a network connection error, then after a minimum delay of one minute, you can retry the request with the same idempotency key to guarantee that only a single wallet transaction is created. If the request was successfully processed, it will prevent any transaction that uses the same idempotency key, and was received within 24 hours of the first request, from being processed.*/
    pub idempotency_key: WalletTransactionIdempotencyKey,
    ///The ID of the e-wallet to debit from
    pub wallet_id: String,
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    ///A reference for the transaction. This must be an alphanumeric string with at most 18 characters and must not contain any special characters or spaces.
    pub reference: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionIdempotencyKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterparty {
    ///The name of the counterparty
    pub name: String,
    ///The counterparty's bank account numbers
    pub numbers: WalletTransactionCounterpartyNumbers,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyNumbers {
    ///The account number and sort code of the counterparty's account
    pub bacs: WalletTransactionCounterpartyBACS,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionCounterpartyBACS(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionAmount {
    ///The ISO-4217 currency code of the transaction. Currently, only `"GBP"` is supported.
    pub iso_currency_code: String,
    ///The amount of the transaction. Must contain at most two digits of precision e.g. `1.23`.
    pub value: f64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionExecuteResponse {
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: WalletTransactionStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionStatus(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListRequest {
    ///Your Plaid API `client_id`. The `client_id` is required and may be provided either in the `PLAID-CLIENT-ID` header or as part of a request body.
    pub client_id: Option<APIClientID>,
    ///Your Plaid API `secret`. The `secret` is required and may be provided either in the `PLAID-SECRET` header or as part of a request body.
    pub secret: Option<APISecret>,
    ///The ID of the e-wallet to fetch transactions from
    pub wallet_id: String,
    ///A base64 value representing the latest transaction that has already been requested. Set this to `next_cursor` received from the previous `/wallet/transactions/list` request. If provided, the response will only contain transactions created before that transaction. If omitted, the response will contain transactions starting from the most recent, and in descending order by the `created_at` time.
    pub cursor: Option<String>,
    ///The number of transactions to fetch
    pub count: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransactionsListResponse {
    ///An array of transactions of an e-wallet, associated with the given `wallet_id`
    pub transactions: Vec<WalletTransaction>,
    ///Cursor used for fetching transactions created before the latest transaction provided in this response
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: RequestID,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletTransaction {
    ///A unique ID identifying the transaction
    pub transaction_id: String,
    ///A reference for the transaction
    pub reference: String,
    #[serde(rename = "type")]
    ///The type of of the transaction. Currently, only `"PAYOUT"` is supported.
    pub type_: String,
    ///The amount and currency of a transaction
    pub amount: WalletTransactionAmount,
    ///An object representing the e-wallet transaction's counterparty
    pub counterparty: WalletTransactionCounterparty,
    /**The status of the transaction.

`INITIATED`: This is the initial state of all transactions. It indicates that the transaction has been initiated and is currently being processed.

`EXECUTED`: The transaction has been successfully executed.

`FAILED`: The transaction failed to process successfully. This is a terminal status.

`BLOCKED`: The transaction has been blocked for violating compliance rules. This is a terminal status.*/
    pub status: WalletTransactionStatus,
    ///Timestamp when the transaction was created, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format.
    pub created_at: String,
}
