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
                deleted timestamptz not null default 'infinity',
                email text unique not null,
                sub text unique not null,
                role int not null,
                subscription_id text not null default '',
                subscription_end timestamptz not null default '-infinity',
                subscription_check timestamptz not null default current_timestamp,
                unique (email, sub)
            );
            drop trigger if exists set_timestamp on users;
            create trigger set_timestamp before update on users for each row execute procedure trigger_set_timestamp();

            create table if not exists profiles (
                id uuid primary key,
                created timestamptz not null default current_timestamp,
                updated timestamptz not null default current_timestamp,
                deleted timestamptz not null default 'infinity',
                user_id uuid not null references users(id) on delete cascade,
                name text not null,
                about text not null,
                avatar_id text not null default '',
                avatar_url text not null default '',
                cover_id text not null default '',
                cover_url text not null default '',
                resume_id text not null default '',
                unique (user_id)
            );
            drop trigger if exists set_timestamp on profiles;
            create trigger set_timestamp before update on profiles for each row execute procedure trigger_set_timestamp();
    "#,
        )
        .await?;

    Ok(())
}
