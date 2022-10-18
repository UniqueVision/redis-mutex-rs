use once_cell::sync::Lazy;
use redis::Script;
use std::time::SystemTime;

static LOCK_SCRIPT: Lazy<Script> =
    Lazy::new(|| Script::new(include_str!("./redis_scripts/batches_lock.lua")));

pub async fn lock(
    key: &str,
    field: &str,
    lock_milli_second: u128,
    redis_conn: &mut redis::aio::Connection,
) -> Result<bool, redis::RedisError> {
    let start_utsms = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut invocation = LOCK_SCRIPT.key(key);
    invocation.key(field);
    invocation.arg(start_utsms.to_string());
    invocation.arg(lock_milli_second.to_string());
    let res: Option<String> = invocation.invoke_async(redis_conn).await?;
    Ok(res.is_some())
}

#[cfg(test)]
mod tests {
    use crate::lock;

    #[tokio::test]
    async fn it_works() {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let mut con = client.get_async_connection().await.unwrap();
        let res1 = lock("mutex", "test", 100, &mut con).await.unwrap();
        assert_eq!(true, res1);
        let res2 = lock("mutex", "test", 100, &mut con).await.unwrap();
        assert_eq!(false, res2);
        tokio::time::sleep(tokio::time::Duration::from_millis(101)).await;
        let res3 = lock("mutex", "test", 100, &mut con).await.unwrap();
        assert_eq!(true, res3);
        tokio::time::sleep(tokio::time::Duration::from_millis(101)).await;
    }
}