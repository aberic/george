use crate::utils::deploy::GLOBAL_CONFIG;

/// 数据根目录 /var/lib/georgedb/data
pub fn data_path() -> String {
    format!(
        "{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data"
    )
}

/// 库根目录 /var/lib/georgedb/data/database
pub fn database_path(database_id: String) -> String {
    format!(
        "{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/view
pub fn view_path(database_id: String, view_id: String) -> String {
    format!(
        "{}/{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id
    )
}

/// 引导文件目录 /var/lib/georgedb/data/bootstrap.sr
pub fn bootstrap_file_path() -> String {
    format!(
        "{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "bootstrap.sr"
    )
}

/// 库根目录 /var/lib/georgedb/data/database/db.sr
pub fn database_file_path(database_id: String) -> String {
    format!(
        "{}/{}/{}/db.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id
    )
}

/// 视图根目录 /var/lib/georgedb/data/database/view/view.sr
pub fn view_file_path(database_id: String, view_id: String) -> String {
    format!(
        "{}/{}/{}/{}/view.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/view/index.sr
pub fn index_file_path(database_id: String, view_id: String, index_name: String) -> String {
    format!(
        "{}/{}/{}/{}/{}.sr",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id,
        index_name
    )
}

/// 索引文件目录 /var/lib/georgedb/data/database/view/index.sr
pub fn index_file_path_yet(
    database_id: String,
    view_id: String,
    index_file_name: String,
) -> String {
    format!(
        "{}/{}/{}/{}/{}",
        GLOBAL_CONFIG.read().unwrap().data_dir.clone(),
        "data",
        database_id,
        view_id,
        index_file_name
    )
}
