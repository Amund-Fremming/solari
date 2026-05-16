// use std::{env, error::Error};

// use solari_core::{PayRequest, PaymentStatus, SolariPaymentService, VippsConfig, VippsPayRequest};
// use tracing::{error, info};

// fn required_env(key: &str) -> Result<String, Box<dyn Error>> {
//     env::var(key).map_err(|_| format!("missing required env var: {key}").into())
// }

// fn vipps_config_from_env() -> Result<VippsConfig, Box<dyn Error>> {
//     Ok(VippsConfig::new(
//         required_env("VIPPS_BASE_URL")?,
//         required_env("VIPPS_CLIENT_ID")?,
//         required_env("VIPPS_CLIENT_SECRET")?,
//         required_env("VIPPS_SUBSCRIPTION_KEY")?,
//         required_env("VIPPS_MSN")?,
//     ))
// }

// fn expect_eq<T>(check: &str, expected: &T, actual: &T) -> Result<(), Box<dyn Error>>
// where
//     T: std::fmt::Debug + PartialEq,
// {
//     if expected == actual {
//         Ok(())
//     } else {
//         Err(format!(
//             "check failed ({check}): expected {:?}, got {:?}",
//             expected, actual
//         )
//         .into())
//     }
// }

// fn expect_true(check: &str, condition: bool, details: &str) -> Result<(), Box<dyn Error>> {
//     if condition {
//         Ok(())
//     } else {
//         Err(format!("check failed ({check}): {details}").into())
//     }
// }

// fn token_preview(token: &str) -> String {
//     token.chars().take(8).collect()
// }

// fn text_preview(value: &str, max_chars: usize) -> String {
//     let preview: String = value.chars().take(max_chars).collect();
//     if value.chars().count() > max_chars {
//         format!("{preview}...")
//     } else {
//         preview
//     }
// }

// async fn get_access_token_scenario() -> Result<(String, u64), Box<dyn Error>> {
//     let mut module = SolariPaymentService::new()?;
//     module.vipps(vipps_config_from_env()?);

//     let token = module.vipps_fetch_access_token().await?;
//     let preview = token_preview(&token.token);

//     expect_true(
//         "access token should not be empty",
//         !token.token.is_empty(),
//         "vipps access token must contain characters",
//     )?;

//     Ok((preview, token.expires_at))
// }

// async fn token_cache_scenario() -> Result<(String, String, u64), Box<dyn Error>> {
//     let mut module = SolariPaymentService::new()?;
//     module.vipps(vipps_config_from_env()?);
//     let first = module.vipps_get_valid_token().await?;
//     let second = module.vipps_get_valid_token().await?;

//     let token_details = format!(
//         "first_token_prefix={} second_token_prefix={}",
//         token_preview(&first.token),
//         token_preview(&second.token)
//     );

//     expect_true(
//         "token cache should return same token on second call",
//         first.token == second.token,
//         &token_details,
//     )?;

//     expect_eq(
//         "token cache should return same expiry on second call",
//         &first.expires_at,
//         &second.expires_at,
//     )?;

//     Ok((
//         token_preview(&first.token),
//         token_preview(&second.token),
//         first.expires_at,
//     ))
// }

// async fn create_payment_scenario(
//     amount: u32,
// ) -> Result<(PaymentStatus, PaymentStatus, String, bool, String), Box<dyn Error>> {
//     let mut module = SolariPaymentService::new()?;
//     module.vipps(vipps_config_from_env()?);

//     let payment = module
//         .pay(PayRequest::Vipps(VippsPayRequest {
//             amount,
//             return_url: None,
//         }))
//         .await?;

//     let redirect_url_present = payment
//         .redirect_url
//         .as_ref()
//         .map(|v| !v.is_empty())
//         .unwrap_or(false);
//     let redirect_url_preview = payment
//         .redirect_url
//         .as_deref()
//         .map(|v| text_preview(v, 100))
//         .unwrap_or_else(|| "<none>".to_string());

//     expect_eq(
//         "create response provider status should be pending",
//         &PaymentStatus::Pending,
//         &payment.status,
//     )?;
//     expect_true(
//         "create response should include redirect url",
//         redirect_url_present,
//         "redirect_url must be present and non-empty",
//     )?;

//     let reference = payment.reference.clone().ok_or_else(|| {
//         "vipps create payment did not return a reference; cannot fetch status".to_string()
//     })?;

//     let status_response = module.vipps_get_payment_status(&reference).await?;

//     let reference_roundtrip = payment.reference == status_response.reference;

//     if !reference_roundtrip {
//         return Err("check failed (reference should roundtrip between create and fetch)".into());
//     }

//     expect_eq(
//         "newly created payment should still be pending",
//         &PaymentStatus::Pending,
//         &status_response.status,
//     )?;

//     Ok((
//         PaymentStatus::Pending,
//         status_response.status,
//         status_response.raw_status,
//         redirect_url_present,
//         redirect_url_preview,
//     ))
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    //     info!("🔐 Scenario 1: Fetch Access Token");
    //     match get_access_token_scenario().await {
    //         Ok((token_prefix, expires_at)) => {
    //             info!("✅ Success: token fetched (first_8={token_prefix}, expires_at={expires_at})")
    //         }
    //         Err(e) => error!("❌ Failed: {e}"),
    //     }

    //     info!("");
    //     info!("🔐 Scenario 2: Verify Token Cache");
    //     match token_cache_scenario().await {
    //         Ok((first_prefix, second_prefix, expires_at)) => info!(
    //             "✅ Success: cache reused token (first_8_first={first_prefix}, first_8_second={second_prefix}, expires_at={expires_at})"
    //         ),
    //         Err(e) => error!("❌ Failed: {e}"),
    //     }

    //     info!("");
    //     info!("🔐 Scenario 3: Create Payment then Fetch Payment Status");
    //     match create_payment_scenario(100).await {
    //         Ok((expected_status, received_status, raw_status, redirect_url_present, redirect_url_preview)) => info!(
    //             "✅ Success: expected_status={expected_status:?}, received_status={received_status:?}, raw_status={raw_status}, redirect_url_present={redirect_url_present}, redirect_preview={redirect_url_preview}"
    //         ),
    //         Err(e) => error!("❌ Failed: {e}"),
    //     }

    Ok(())
}
