struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("nitesh"),
        email: String::from("niteshjain261992@gmail.com"),
        sign_in_count: 1,
        active: true
    };

    let user2 = User {
        username: String::from("nitesh_office"),
        email: String::from("nitesh2.jain@aricent.com"),
        ..user1
    };
}
