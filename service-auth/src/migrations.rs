use anyhow::Result;

pub async fn run_migrations(pool: &deadpool_postgres::Pool) -> Result<()> {
    let client = pool.get().await?;
    client
        .batch_execute(
            r#"
            create table if not exists verifiers (
                id uuid primary key,
                created timestamptz not null default now(),
                csrf_token text not null,
                pkce_verifier text not null,
                unique (csrf_token, pkce_verifier)
            );
        "#,
        )
        .await?;

    Ok(())
}
