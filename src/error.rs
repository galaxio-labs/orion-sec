use derive_more::From;
use orion_error::{OrionError, StructError, UnifiedReason};
use serde_derive::Serialize;

#[derive(Debug, PartialEq, Serialize, From, OrionError)]
pub enum OrionSecReason {
    #[orion_error(transparent)]
    Sec(SecReason),
    #[orion_error(transparent)]
    General(UnifiedReason),
}

#[derive(Debug, PartialEq, Serialize, OrionError)]
pub enum SecReason {
    #[orion_error(identity = "biz.sensitive_msg", code = 101)]
    SensitiveMsg(String),
    #[orion_error(identity = "biz.no_permission", code = 201)]
    NoPermission(String),
    #[orion_error(identity = "biz.deception", code = 301)]
    Deception(String),
    #[orion_error(identity = "biz.un_authenticated", code = 401)]
    UnAuthenticated(String),
}

impl From<orion_conf::ConfIOReason> for OrionSecReason {
    fn from(r: orion_conf::ConfIOReason) -> Self {
        match r {
            orion_conf::ConfIOReason::General(u) => OrionSecReason::General(u),
            _ => OrionSecReason::General(UnifiedReason::system_error()),
        }
    }
}

pub type SecError = StructError<OrionSecReason>;
pub type SecResult<T> = Result<T, SecError>;
