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
            create table if not exists users (
                id uuid primary key,
                created timestamptz not null default current_timestamp,
                updated timestamptz not null default current_timestamp,
                deleted timestamptz not null default '-infinity',
                email text unique not null,
                sub text unique not null,
                role int not null,
                name text not null default '',
                avatar_id uuid not null default '00000000-0000-0000-0000-000000000000',
                payment_id text not null default ''
            );
            drop trigger if exists set_timestamp on users;
            create trigger set_timestamp before update on users for each row execute procedure trigger_set_timestamp();

    "#,
        )
        .await?;

    Ok(())
}
