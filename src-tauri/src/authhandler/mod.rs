use mockall::automock;

use crate::{
    redirectserviceadapter::{RedirectServiceAdapter, RedirectServiceAdapterResponse},
    timehandler::TimeHandler,
};

pub struct AuthHandlerStruct {
    redirect_handler: Box<dyn RedirectServiceAdapter>,
    time_handler: Box<dyn TimeHandler>,
}

#[automock]
pub trait AuthHandler {
    fn poll_for_access_status(&self) -> Result<RedirectServiceAdapterResponse, String>;
}

impl AuthHandlerStruct {
    pub fn new(
        redirect_handler: Box<dyn RedirectServiceAdapter>,
        time_handler: Box<dyn TimeHandler>,
    ) -> Self {
        AuthHandlerStruct {
            redirect_handler,
            time_handler,
        }
    }
}

impl AuthHandler for AuthHandlerStruct {
    fn poll_for_access_status(&self) -> Result<RedirectServiceAdapterResponse, String> {
        let mut attempts = 5;

        let error_value = Err("No access confirmation".to_string());

        while attempts > 0 {
            if let Some(response) = self.redirect_handler.check_access_status() {
                return Ok(response);
            }
            attempts = attempts - 1;
            (self.time_handler.get_polling_delay())();
        }

        error_value
    }
}

#[cfg(test)]
pub mod tests {
    use super::AuthHandlerStruct;
    use crate::{
        authhandler::AuthHandler,
        redirectserviceadapter::{MockRedirectServiceAdapter, RedirectServiceAdapterResponse},
        timehandler::TimeHandler,
    };

    #[derive(Clone, Copy)]
    struct MockTimeHandler;

    impl TimeHandler for MockTimeHandler {
        fn get_polling_delay(&self) -> Box<dyn Fn() -> ()> {
            Box::new(|| {})
        }
    }

    static MOCK_TIME_HANDLER: MockTimeHandler = MockTimeHandler;

    #[test]
    pub fn should_return_redirect_adapter_response_when_access_granted() {
        let mut mock_redirect_adapter = MockRedirectServiceAdapter::new();
        let expected_response = Some(RedirectServiceAdapterResponse::new());
        mock_redirect_adapter
            .expect_check_access_status()
            .returning(move || expected_response.clone());
        let auth_handler = AuthHandlerStruct::new(
            Box::new(mock_redirect_adapter),
            Box::new(MOCK_TIME_HANDLER.clone()),
        );
        let response = auth_handler.poll_for_access_status();
        assert_eq!(expected_response, response.ok());
    }

    #[test]
    pub fn should_return_err_when_access_not_granted_after_5_attempts() {
        let mut mock_redirect_adapter = MockRedirectServiceAdapter::new();
        mock_redirect_adapter
            .expect_check_access_status()
            .times(5)
            .returning(|| None);
        let auth_handler = AuthHandlerStruct::new(
            Box::new(mock_redirect_adapter),
            Box::new(MOCK_TIME_HANDLER.clone()),
        );
        let response = auth_handler.poll_for_access_status();

        assert!(response.is_err());
    }
}
