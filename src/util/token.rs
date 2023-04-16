use crate::client::Client;

pub async fn validate_credentials(identifier: impl ToString, secret: impl ToString) -> bool {
    let client = Client::new(identifier, secret);
    client
        .get_account_transactions(None, Some(1), None)
        .await
        .is_ok()
}
