#[cfg(test)]
mod test {
    use crate::{BizActivity, init_sqlite};
    use rbatis::crud::{CRUDMut, CRUD};
    use rbatis::rbatis::Rbatis;

    #[tokio::test]
    pub async fn test_use_driver_wrapper() {
        fast_log::init(fast_log::config::Config::new().console());
        let rb = init_sqlite().await;

        let name = "test";
        let w = rb
            .new_wrapper()
            .r#in("delete_flag", &[0, 1])
            .and()
            .ne("delete_flag", -1)
            .between("status", -8, 100)
            .r#if(!name.is_empty(), |w| w.and().like("name", name));
        let r: Vec<BizActivity> = rb.fetch_list_by_wrapper(w).await.unwrap();
        println!("done:{:?}", r);
    }
}
