use mockall::automock;

pub struct RedirectServiceAdapterStruct;

#[automock]
pub trait RedirectServiceAdapter {
    fn check_access_status(&self) -> Option<RedirectServiceAdapterResponse>;
}

impl RedirectServiceAdapter for RedirectServiceAdapterStruct {
    fn check_access_status(&self) -> Option<RedirectServiceAdapterResponse> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RedirectServiceAdapterResponse;
impl RedirectServiceAdapterResponse {
    pub fn new() -> Self {
        RedirectServiceAdapterResponse {}
    }
}
