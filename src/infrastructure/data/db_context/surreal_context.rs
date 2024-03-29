use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};

pub async fn connect_db(
    addr: &str,
    user: &str,
    pass: &str,
    ns: &str,
    dbname: &str,
) -> Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(addr).await?;
    let _ = db
        .signin(Root {
            username: user,
            password: pass,
        })
        .await?;
    db.use_ns(ns).use_db(dbname).await?;

    Ok(db)
}
