use anyhow::Result;

pub async fn run_migrations(pool: &deadpool_postgres::Pool) -> Result<()> {
    let client = pool.get().await?;
    client
        .batch_execute(
            r#"
            create or replace function trigger_set_timestamp ()
            returns trigger
            as $$
            begin
                new.updated = now();
                return new;
            end;
            $$ language plpgsql;

            create table if not exists notes (
                id uuid primary key,
                created timestamptz not null default current_timestamp,
                updated timestamptz not null default current_timestamp,
                deleted timestamptz not null default 'infinity',
                user_id uuid not null,
                title text not null,
                content text not null
            );
            drop trigger if exists set_timestamp on notes;
            create trigger set_timestamp before update on notes for each row execute procedure trigger_set_timestamp();
    "#,
        )
        .await?;

    Ok(())
}
