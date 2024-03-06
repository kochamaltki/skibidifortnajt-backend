use sqlite::{State, Connection};


pub fn check_banned(connection: &Connection, user_id: i64) -> i64{
   let query = "SELECT is_banned FROM users WHERE user_id = ?";
   let mut statement = connection.prepare(query).unwrap();
   statement.bind((1, user_id)).unwrap();

    let is_banned = if let Ok(State::Row) = statement.next() {
        statement.read::<i64, _>(0).unwrap()
    } else {
        1
    };

    is_banned
}
