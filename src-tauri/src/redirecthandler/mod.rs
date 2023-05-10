use mockall::automock;

pub struct RedirectAdapterStruct;

#[automock]
pub trait RedirectAdapter {
    fn check_access_status(&self) -> Option<RedirectAdapterResponse>;
}

impl RedirectAdapter for RedirectAdapterStruct {
    fn check_access_status(&self) -> Option<RedirectAdapterResponse> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RedirectAdapterResponse;
impl RedirectAdapterResponse {
    pub fn new() -> Self {
        RedirectAdapterResponse {}
    }
}
