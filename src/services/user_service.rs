mod models;
mod mysql;
use models::UserModel;
use mysql::establish_connection;

fn InsertUser(){
    let new_user = UserModel{
        username: String::from("Josh"),
        pass: String::from("Test"),
        email: String::from("julises072@gmail.com"),
    };
    let connection = establish_connection();
    insert_into(users::table)
    .values(&new_user)
    .execute(&connection);
}