pub mod account;
pub mod user;

use std::collections::HashMap;

pub use account::Account;
use serde::{Deserialize, Serialize};
pub use user::User;

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

/// Common claims data
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Data {
    #[serde(rename = "aud", default, skip_serializing_if = "str::is_empty")]
    pub audience: String,

    /// Time when the token expires (in seconds since the unix epoch)
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,

    #[serde(rename = "jti", default, skip_serializing_if = "str::is_empty")]
    pub id: String,

    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<i64>,

    #[serde(rename = "iss", default, skip_serializing_if = "str::is_empty")]
    pub issuer: String,

    #[serde(default, skip_serializing_if = "str::is_empty")]
    pub name: String,

    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,

    #[serde(rename = "sub", default, skip_serializing_if = "str::is_empty")]
    pub subject: String,
}

impl IsEmpty for Data {
    fn is_empty(&self) -> bool {
        let Self {
            audience,
            expires,
            id,
            issued_at,
            issuer,
            name,
            not_before,
            subject,
        } = self;

        todo!()
    }
}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Import {}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Export {}

pub type SigningKeys = HashMap<String, UserScope>;

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UserScope {}

pub type RevocationList = HashMap<String, i64>;

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Permissions {}

impl IsEmpty for Permissions {
    fn is_empty(&self) -> bool {
        todo!()
    }
}

pub type Mapping = HashMap<Subject, Vec<WeightedMapping>>;

pub type Subject = String;

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WeightedMapping {}

impl IsEmpty for WeightedMapping {
    fn is_empty(&self) -> bool {
        todo!()
    }
}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Info {}

impl IsEmpty for Info {
    fn is_empty(&self) -> bool {
        todo!()
    }
}

// TODO
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GenericFields {}

impl IsEmpty for GenericFields {
    fn is_empty(&self) -> bool {
        todo!()
    }
}

// AccountClaims:
// ClaimsData
// Account `json:"nats,omitempty"`
//
// ClaimsData:
// Audience  string `json:"aud,omitempty"`
// Expires   int64  `json:"exp,omitempty"`
// ID        string `json:"jti,omitempty"`
// IssuedAt  int64  `json:"iat,omitempty"`
// Issuer    string `json:"iss,omitempty"`
// Name      string `json:"name,omitempty"`
// NotBefore int64  `json:"nbf,omitempty"`
// Subject   string `json:"sub,omitempty"`
//
//
// Account:
// Imports            Imports               `json:"imports,omitempty"`
// Exports            Exports               `json:"exports,omitempty"`
// Limits             OperatorLimits        `json:"limits,omitempty"`
// SigningKeys        SigningKeys           `json:"signing_keys,omitempty"`
// Revocations        RevocationList        `json:"revocations,omitempty"`
// DefaultPermissions Permissions           `json:"default_permissions,omitempty"`
// Mappings           Mapping               `json:"mappings,omitempty"`
// Authorization      ExternalAuthorization `json:"authorization,omitempty"`
// Info
// GenericFields
//
// type Imports []*Import
//
// Import:
// Name string `json:"name,omitempty"`
// Subject Subject `json:"subject,omitempty"`
// Account string  `json:"account,omitempty"`
// Token   string  `json:"token,omitempty"`
// LocalSubject RenamingSubject `json:"local_subject,omitempty"`
// Type         ExportType      `json:"type,omitempty"`
// Share        bool            `json:"share,omitempty"`
//
// type Subject string
// type RenamingSubject Subject
// type ExportType int
//
// type Exports []*Export
//
// Export:
// Name                 string          `json:"name,omitempty"`
// Subject              Subject         `json:"subject,omitempty"`
// Type                 ExportType      `json:"type,omitempty"`
// TokenReq             bool            `json:"token_req,omitempty"`
// Revocations          RevocationList  `json:"revocations,omitempty"`
// ResponseType         ResponseType    `json:"response_type,omitempty"`
// ResponseThreshold    time.Duration   `json:"response_threshold,omitempty"`
// Latency              *ServiceLatency `json:"service_latency,omitempty"`
// AccountTokenPosition uint            `json:"account_token_position,omitempty"`
// Advertise            bool            `json:"advertise,omitempty"`
// Info
//
// type RevocationList map[string]int64
// type Duration int64
//
// ServiceLatency:
// Sampling SamplingRate `json:"sampling"`
// Results  Subject      `json:"results"`
//
// type SamplingRate int
//
// Info:
// Description string `json:"description,omitempty"`
// InfoURL     string `json:"info_url,omitempty"`
//
// OperatorLimits:
// NatsLimits
// AccountLimits
// JetStreamLimits
// JetStreamTieredLimits `json:"tiered_limits,omitempty"`
//
// NatsLimits:
// Subs    int64 `json:"subs,omitempty"`    // Max number of subscriptions
// Data    int64 `json:"data,omitempty"`    // Max number of bytes
// Payload int64 `json:"payload,omitempty"` // Max message payload
//
// AccountLimits:
// Imports         int64 `json:"imports,omitempty"`         // Max number of imports
// Exports         int64 `json:"exports,omitempty"`         // Max number of exports
// WildcardExports bool  `json:"wildcards,omitempty"`       // Are wildcards allowed in exports
// DisallowBearer  bool  `json:"disallow_bearer,omitempty"` // User JWT can't be bearer token
// Conn            int64 `json:"conn,omitempty"`            // Max number of active connections
// LeafNodeConn    int64 `json:"leaf,omitempty"`            // Max number of active leaf node connections
//
// JetStreamLimits:
// MemoryStorage        int64 `json:"mem_storage,omitempty"`           // Max number of bytes stored in memory across all streams. (0 means disabled)
// DiskStorage          int64 `json:"disk_storage,omitempty"`          // Max number of bytes stored on disk across all streams. (0 means disabled)
// Streams              int64 `json:"streams,omitempty"`               // Max number of streams
// Consumer             int64 `json:"consumer,omitempty"`              // Max number of consumers
// MaxAckPending        int64 `json:"max_ack_pending,omitempty"`       // Max ack pending of a Stream
// MemoryMaxStreamBytes int64 `json:"mem_max_stream_bytes,omitempty"`  // Max bytes a memory backed stream can have. (0 means disabled/unlimited)
// DiskMaxStreamBytes   int64 `json:"disk_max_stream_bytes,omitempty"` // Max bytes a disk backed stream can have. (0 means disabled/unlimited)
// MaxBytesRequired     bool  `json:"max_bytes_required,omitempty"`    // Max bytes required by all Streams
//
// type JetStreamTieredLimits map[string]JetStreamLimits
//
// type SigningKeys map[string]Scope
// For now there is only a UserScope
//
// type Scope interface {
// 	SigningKey() string
// 	ValidateScopedSigner(claim Claims) error
// 	Validate(vr *ValidationResults)
// }
//
// Permissions:
// Pub  Permission          `json:"pub,omitempty"`
// Sub  Permission          `json:"sub,omitempty"`
// Resp *ResponsePermission `json:"resp,omitempty"`
//
// Permission:
// Allow StringList `json:"allow,omitempty"`
// Deny  StringList `json:"deny,omitempty"`
//
// ResponsePermission:
// MaxMsgs int           `json:"max"`
// Expires time.Duration `json:"ttl"`
//
// type Mapping map[Subject][]WeightedMapping
//
// Subject Subject `json:"subject"`
// Weight  uint8   `json:"weight,omitempty"`
// Cluster string  `json:"cluster,omitempty"`
//
// Subject Subject `json:"subject"`
// Weight  uint8   `json:"weight,omitempty"`
// Cluster string  `json:"cluster,omitempty"`
//
// ExternalAuthorization:
// AuthUsers       StringList `json:"auth_users"`
// AllowedAccounts StringList `json:"allowed_accounts,omitempty"`
// XKey            string     `json:"xkey,omitempty"`
//
// type StringList []string
//
// GenericFields:
// Tags    TagList   `json:"tags,omitempty"`
// Type    ClaimType `json:"type,omitempty"`
// Version int       `json:"version,omitempty"`
//
// type TagList []string
// type ClaimType string
//
// UserClaims:
// ClaimsData
// User `json:"nats,omitempty"`
//
// User:
// UserPermissionLimits
// IssuerAccount string `json:"issuer_account,omitempty"`
// GenericFields
//
// UserPermissionLimits:
// Permissions
// Limits
// BearerToken            bool       `json:"bearer_token,omitempty"`
// AllowedConnectionTypes StringList `json:"allowed_connection_types,omitempty"`
//
// Limits:
// UserLimits
// NatsLimits
//
// UserLimits:
// Src    CIDRList    `json:"src,omitempty"`
// Times  []TimeRange `json:"times,omitempty"`
// Locale string      `json:"times_location,omitempty"`
//
// type CIDRList TagList
//
// TimeRange:
// Start string `json:"start,omitempty"`
// End   string `json:"end,omitempty"`
//
// UserScope
// Kind     ScopeType            `json:"kind"`
// Key      string               `json:"key"`
// Role     string               `json:"role"`
// Template UserPermissionLimits `json:"template"`
//
// type ScopeType int
