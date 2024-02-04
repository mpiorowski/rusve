use anyhow::Result;
use std::str::FromStr;
use stripe::{
    Client, CreateCustomer, Customer, CustomerId, ListSubscriptions, Subscription,
    SubscriptionStatus,
};
use tonic::{Request, Response, Status};

pub async fn check_subscription(
    env: &service_users::Env,
    conn: &deadpool_postgres::Object,
    user: &crate::proto::User,
) -> Result<bool> {
    if user.subscription_id.is_empty() {
        return Ok(false);
    }

    // Check if subscription is still active versus the current time plus 2 days
    if user.subscription_end != "-infinity" {
        let subscription_end = time::OffsetDateTime::parse(
            &user.subscription_end,
            &time::format_description::well_known::Iso8601::DEFAULT,
        )?;
        if subscription_end >= time::OffsetDateTime::now_utc() {
            return Ok(true);
        }
    }

    // Check if subscription was checked in the last hour
    if user.subscription_check != "-infinity" {
        let subscription_check = time::OffsetDateTime::parse(
            &user.subscription_check,
            &time::format_description::well_known::Iso8601::DEFAULT,
        )?;
        if subscription_check >= time::OffsetDateTime::now_utc() - time::Duration::hours(1) {
            return Ok(false);
        }
    }
    let _ = crate::stripe_db::update_user_subscription_check(conn, &user.id).await?;

    let secret_key = env.stripe_api_key.clone();
    let client = Client::new(secret_key);

    let customer_id: CustomerId = CustomerId::from_str(&user.subscription_id)?;
    let mut params: ListSubscriptions<'_> = ListSubscriptions::new();
    params.customer = Some(customer_id);

    let subscriptions = Subscription::list(&client, &params).await?.data;
    for subscription in subscriptions {
        if subscription.status == SubscriptionStatus::Active {
            crate::stripe_db::update_user_subscription_end(
                conn,
                &user.id,
                subscription.current_period_end,
            )
            .await?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub async fn create_stripe_checkout(
    env: &service_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Empty>,
) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let user_id = service_users::decode_token(metadata, &env.jwt_secret)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;
    let user =
        crate::user_db::select_user_by_id(&conn, crate::user_db::StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

    let url = create_checkout(env, conn, user)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create checkout session: {:?}", e);
            Status::internal("Failed to create checkout session")
        })?;

    tracing::info!("create_stripe_checkout: {:?}", start.elapsed());
    Ok(Response::new(crate::proto::StripeUrlResponse { url }))
}

pub async fn create_stripe_portal(
    env: &service_users::Env,
    pool: &deadpool_postgres::Pool,
    request: Request<crate::proto::Empty>,
) -> Result<Response<crate::proto::StripeUrlResponse>, Status> {
    let start = std::time::Instant::now();
    let metadata = request.metadata();
    let user_id = service_users::decode_token(metadata, &env.jwt_secret)?.id;

    let conn = pool.get().await.map_err(|e| {
        tracing::error!("Failed to get connection: {:?}", e);
        Status::internal("Failed to get connection")
    })?;
    let user =
        crate::user_db::select_user_by_id(&conn, crate::user_db::StringOrUuid::String(user_id))
            .await
            .map_err(|e| {
                tracing::error!("Failed to auth user: {:?}", e);
                Status::unauthenticated("Failed to auth user")
            })?;

    let url = create_portal(env, &conn, user).await.map_err(|e| {
        tracing::error!("Failed to create portal session: {:?}", e);
        Status::internal("Failed to create portal session")
    })?;

    tracing::info!("create_stripe_portal: {:?}", start.elapsed());
    Ok(Response::new(crate::proto::StripeUrlResponse { url }))
}

async fn create_checkout(
    env: &service_users::Env,
    conn: deadpool_postgres::Object,
    user: crate::proto::User,
) -> Result<String> {
    let secret_key = env.stripe_api_key.clone();
    let price_id = env.stripe_price_id.clone();
    let client_url = env.client_url.clone();
    let client = Client::new(secret_key);

    let mut customer_id = user.subscription_id;
    if customer_id.is_empty() {
        customer_id = create_customer(&client, &user.email).await?;
        crate::stripe_db::update_user_subscription_id(&conn, &user.id, &customer_id.clone())
            .await?;
    }

    let success_url = format!("{}/subscription?success", client_url);
    let cancel_url = format!("{}/subscription?cancel", client_url);

    let mut params = stripe::CreateCheckoutSession::new(&success_url);
    params.customer = Some(CustomerId::from_str(&customer_id)?);
    params.cancel_url = Some(&cancel_url);
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

    let _ = crate::stripe_db::remove_user_subscription_check(&conn, &user.id).await?;

    Ok(session_url)
}

async fn create_customer(client: &Client, email: &str) -> Result<String> {
    let mut customer: CreateCustomer<'_> = CreateCustomer::new();
    customer.email = Some(email);
    let customer = Customer::create(client, customer).await?;

    Ok(customer.id.to_string())
}

async fn create_portal(
    env: &service_users::Env,
    conn: &deadpool_postgres::Object,
    user: crate::proto::User,
) -> Result<String> {
    let secret_key = env.stripe_api_key.clone();
    let client = Client::new(secret_key);

    let mut customer_id = user.subscription_id;
    if customer_id.is_empty() {
        customer_id = create_customer(&client, &user.email).await?;
        crate::stripe_db::update_user_subscription_id(conn, &user.id, &customer_id.clone()).await?;
    }

    let params = stripe::CreateBillingPortalSession::new(CustomerId::from_str(&customer_id)?);
    let session_url = stripe::BillingPortalSession::create(&client, params)
        .await?
        .url;

    Ok(session_url)
}
