use diesel::prelude::*;
use diesel::mysql::MysqlConnection;

type InfraMysqlConnection = MysqlConnection;

pub fn establish_connection() -> Option<InfraMysqlConnection> {

    env_logger::init();
    
    info!("Holaaa");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

    let mysql_connection = MysqlConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url));
    Some(mysql_connection)

}