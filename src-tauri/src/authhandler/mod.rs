use std::{thread::sleep, time::Duration};

use mockall::automock;

use crate::redirecthandler::{RedirectAdapter, RedirectAdapterResponse};

pub struct AuthHandlerStruct {
    redirect_handler: Box<dyn RedirectAdapter>,
}

#[automock]
pub trait AuthHandler {
    fn poll_for_access_status(&self) -> Result<RedirectAdapterResponse, String>;
}

impl AuthHandlerStruct {
    pub fn new(redirect_handler: Box<dyn RedirectAdapter>) -> Self {
        AuthHandlerStruct { redirect_handler }
    }
}

impl AuthHandler for AuthHandlerStruct {
    fn poll_for_access_status(&self) -> Result<RedirectAdapterResponse, String> {
        let poll_buffer = Duration::from_secs(2);

        let attempts = 5;

        let error_value = Err("No access confirmation".to_string());

        while attempts > 0 {
            if let Some(response) = self.redirect_handler.check_access_status() {
                return Ok(response);
            }
            sleep(poll_buffer);
        }

        error_value
    }
}

#[cfg(test)]
pub mod tests {
    use super::AuthHandlerStruct;
    use crate::{
        authhandler::AuthHandler,
        redirecthandler::{MockRedirectAdapter, RedirectAdapterResponse},
    };

    #[test]
    pub fn should_return_redirect_adapter_response_when_access_granted() {
        let mut mock_redirect_adapter = MockRedirectAdapter::new();
        let mocked_response = Some(RedirectAdapterResponse::new());
        mock_redirect_adapter
            .expect_check_access_status()
            .returning(move || mocked_response.clone());
        let auth_handler = AuthHandlerStruct::new(Box::new(mock_redirect_adapter));
        let expected_response = mocked_response;
        let response = auth_handler.poll_for_access_status();
        assert_eq!(expected_response, response.ok());
    }
}
