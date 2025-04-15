use dioxus::prelude::*;

#[cfg(any(feature = "server", feature = "mobile"))]
thread_local! {
    pub static DB: rusqlite::Connection = {
        #[cfg(all(target_os = "android", not(feature = "server")))]
        let db_path = {
            use dioxus::mobile::wry::prelude::jni;
            let ctx = ndk_context::android_context();
            let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
            let mut env = vm.attach_current_thread().unwrap();
            let ctx = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };
            let cache_dir = env
                .call_method(ctx, "getFilesDir", "()Ljava/io/File;", &[]).unwrap()
                .l().unwrap();
            let cache_dir: jni::objects::JString = env
                .call_method(&cache_dir, "toString", "()Ljava/lang/String;", &[]).unwrap()
                .l().unwrap()
                .try_into().unwrap();
            let cache_dir = env.get_string(&cache_dir).unwrap();
            let cache_dir = cache_dir.to_str().unwrap();

            format!("{cache_dir}/hotdog.db")
        };
        #[cfg(all(target_os = "android", not(feature = "server")))]
        let db_path = db_path.as_str();
        #[cfg(all(feature = "server", not(target_os = "android")))]
        let db_path = "hotdog.db";

        let conn = rusqlite::Connection::open(db_path).expect("Failed to open database");

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );"
        ).unwrap();

        conn
    };
}

#[server]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    DB.with(|conn| conn.execute("INSERT INTO dogs (url) values (?1)", &[&image]))?;
    Ok(())
}

#[server]
pub async fn list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server]
pub async fn delete_dog(id: usize) -> Result<(), ServerFnError> {
    DB.with(|conn| {
        conn.prepare("DELETE FROM dogs WHERE id = ?1")
            .unwrap()
            .execute(&[&id])
            .unwrap();
    });

    Ok(())
}
