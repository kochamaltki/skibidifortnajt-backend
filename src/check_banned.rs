use sqlite::{State};


pub fn check_banned(user_id: i64) -> i64{

    let connection = sqlite::open("projekt-db").unwrap();


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
