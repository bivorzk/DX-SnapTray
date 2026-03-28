use dioxus::prelude::*;

pub const OTP_LENGTH: usize = 3;
pub const AUTH_CODE: &str = "123";

#[derive(Clone, PartialEq, Debug)]
pub enum Status {
    Idle,
    Pending,
    Success,
    Error,
    Expired,
}

pub fn code_matches(code: &str) -> bool {
    code == AUTH_CODE
}

pub fn is_active(status: &Status) -> bool {
    matches!(status, Status::Idle | Status::Error)
}
