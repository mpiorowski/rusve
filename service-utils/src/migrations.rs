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

            create table if not exists files (
                id uuid primary key,
                created timestamptz not null default current_timestamp,
                updated timestamptz not null default current_timestamp,
                deleted timestamptz not null default 'infinity',
                target_id uuid not null,
                file_name text not null,
                file_type int not null
            );
            drop trigger if exists set_timestamp on files;
            create trigger set_timestamp before update on files for each row execute procedure trigger_set_timestamp();
    "#,
        )
        .await?;

    Ok(())
}
