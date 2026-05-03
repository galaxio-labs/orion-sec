use derive_more::From;
use orion_error::reason::ErrorIdentityProvider;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde_derive::Serialize;
use thiserror::Error;

#[derive(Debug, PartialEq, Serialize, From, OrionError)]
pub enum OrionSecReason {
    #[orion_error(transparent)]
    Sec(SecReason),
    #[orion_error(transparent)]
    General(UnifiedReason),
}

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum SecReason {
    #[error("sensitive msg {0}")]
    SensitiveMsg(String),
    #[error("no permission {0}")]
    NoPermission(String),
    #[error("deception {0}")]
    Deception(String),
    #[error("un authenticated {0}")]
    UnAuthenticated(String),
}

impl ErrorIdentityProvider for SecReason {
    fn stable_code(&self) -> &'static str {
        match self {
            SecReason::SensitiveMsg(_) => "sec.sensitive_msg",
            SecReason::NoPermission(_) => "sec.no_permission",
            SecReason::Deception(_) => "sec.deception",
            SecReason::UnAuthenticated(_) => "sec.un_authenticated",
        }
    }

    fn error_category(&self) -> orion_error::reason::ErrorCategory {
        orion_error::reason::ErrorCategory::Biz
    }
}

impl orion_error::reason::DomainReason for SecReason {}

impl orion_error::reason::ErrorCode for SecReason {
    fn error_code(&self) -> i32 {
        match self {
            SecReason::SensitiveMsg(_) => 101,
            SecReason::NoPermission(_) => 201,
            SecReason::Deception(_) => 301,
            SecReason::UnAuthenticated(_) => 401,
        }
    }
}

pub type SecError = StructError<OrionSecReason>;
pub type SecResult<T> = Result<T, SecError>;
