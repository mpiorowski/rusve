use anyhow::Result;
use std::str::FromStr;
use stripe::{
    Client, CreateCustomer, Customer, CustomerId, ListSubscriptions, Subscription,
    SubscriptionStatus,
};

pub async fn create_checkout_session(
    conn: &deadpool_postgres::Object,
    user: crate::proto::User,
) -> Result<String> {
    let secret_key =
        std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_API_KEY environment variable");
    let price_id =
        std::env::var("STRIPE_PRICE_ID").expect("Missing STRIPE_PRICE_ID environment variable");
    let client = Client::new(secret_key);

    let mut customer_id = user.subscription_id;
    if customer_id.is_empty() {
        customer_id = create_customer(&client, &user.email).await?;
        crate::users_db::update_user_subscription_id(conn, &user.id, &customer_id.clone()).await?;
    }

    let mut params = stripe::CreateCheckoutSession::new("http://localhost:8080/success");
    params.customer = Some(CustomerId::from_str(&customer_id)?);
    params.cancel_url = Some("http://localhost:8080/cancel");
    params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
        adjustable_quantity: None,
        dynamic_tax_rates: None,
        price: Some(price_id),
        price_data: None,
        quantity: Some(1),
        tax_rates: None,
    }]);
    params.mode = Some(stripe::CheckoutSessionMode::Subscription);

    let session_url = stripe::CheckoutSession::create(&client, params)
        .await?
        .url
        .ok_or_else(|| anyhow::anyhow!("Missing session url"))?;

    Ok(session_url)
}

pub async fn create_customer(client: &Client, email: &str) -> Result<String> {
    let mut customer: CreateCustomer<'_> = CreateCustomer::new();
    customer.email = Some(email);
    let customer = Customer::create(&client, customer).await?;

    Ok(customer.id.to_string())
}

pub async fn check_if_subscribed(
    conn: &deadpool_postgres::Object,
    user: crate::proto::User,
) -> Result<bool> {
    if user.subscription_id.is_empty() {
        return Ok(false);
    }
    if !user.subscription_end.is_empty() {
        let subscription_end = time::OffsetDateTime::parse(
            &user.subscription_end,
            &time::format_description::well_known::Iso8601::DEFAULT,
        )?;
        // Check if subscription is still active versus the current time plus 2 days
        if subscription_end >= time::OffsetDateTime::now_utc() + time::Duration::days(2) {
            return Ok(true);
        }
    }

    let secret_key =
        std::env::var("STRIPE_API_KEY").expect("Missing STRIPE_API_KEY environment variable");
    let client = Client::new(secret_key);

    let customer_id: CustomerId = CustomerId::from_str(&user.subscription_id)?;
    let mut params: ListSubscriptions<'_> = ListSubscriptions::new();
    params.customer = Some(customer_id);

    let subscriptions = Subscription::list(&client, &params).await?.data;
    for subscription in subscriptions {
        if subscription.status == SubscriptionStatus::Active {
            crate::users_db::update_user_subscription_end(
                conn,
                &user.id,
                &subscription.current_period_end.to_string(),
            )
            .await?;
            return Ok(true);
        }
    }
    Ok(false)
}
