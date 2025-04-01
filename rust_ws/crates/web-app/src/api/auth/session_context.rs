
// Session context provides an information about the current user.
#[derive(Debug)]
pub struct SessionContext {
    pub account_address: Option<String>,
}

impl SessionContext {
    pub fn is_authenticated(&self) -> bool {
        self.account_address.is_some()
    }
}
