use sqlx::{PgPool, migrate::MigrateError};

// static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("rust/yandex_course/i_3_task3_actix_backend_practice/migrations");
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

// pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
pub async fn run(pool: &PgPool) -> Result<(), MigrateError> {
    MIGRATOR.run(pool).await
}