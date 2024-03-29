use crate::model::public_data::PublicData;

pub async fn get_by_key(
    key: &str,
    pool: &sqlx::MySqlPool,
) -> Result<Option<PublicData>, sqlx::Error> {
    let query_result = sqlx::query_as::<_, PublicData>("SELECT * FROM public_data WHERE `key` = ?")
        .bind(&key.to_string())
        .fetch_optional(pool)
        .await;

    query_result
}

pub async fn upsert(
    key: &str,
    value: &str,
    pool: &sqlx::MySqlPool,
) -> Result<PublicData, sqlx::Error> {
    let _ = sqlx::query("INSERT INTO public_data (`key`, value) VALUES (?, ?) ON DUPLICATE KEY UPDATE `key`=?, value=?")
        .bind(key)
        .bind(value)
        .bind(key)
        .bind(value)
        .execute(pool)
        .await;

    Ok(get_by_key(key, pool).await?.expect("Failed to get public data after upsert"))
}

pub async fn delete_by_key(key: &str, pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("DELETE FROM public_data WHERE `key` = ?")
        .bind(key)
        .execute(pool)
        .await;

    Ok(())
}
