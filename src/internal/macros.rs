macro_rules! api_url {
    ($e:expr) => {
        concat!("https://api.skinport.com/v1/", $e).to_owned()
    };
    ($e:expr, $($rest:tt)*) => {
        format!(api!($e), $($rest)*)
    };
}

#[macro_export]
macro_rules! base64_encode {
    ($e:expr) => {{
        use base64::Engine;
        base64::engine::general_purpose::STANDARD.encode($e)
    }};
}

#[macro_export]
macro_rules! auth_header {
    ($client_id:expr, $client_secret:expr) => {{
        use crate::base64_encode;
        format!(
            "Basic {}",
            base64_encode!($client_id.to_owned() + ":" + &$client_secret.to_owned())
        )
    }};
}

#[macro_export]
macro_rules! cargo_crate_version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[macro_export]
macro_rules! user_agent_header {
    () => {{
        use crate::cargo_crate_version;
        format!("Skinport Rust Client v{}", cargo_crate_version!())
    }};
}
