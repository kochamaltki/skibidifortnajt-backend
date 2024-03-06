use sqlite::Connection;

pub fn purge_data(connection: &Connection, user_id: i64) {
    let query = "DELETE FROM posts WHERE user_id = ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, user_id)).unwrap();
    statement.next().unwrap();
}
