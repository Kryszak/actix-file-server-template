#[derive(Clone)]
pub struct Creds {
    pub username: String,
    pub password: String,
}

impl Creds {
    pub fn new(user: String, passwd: String) -> Self {
        return Creds {
            username: user,
            password: passwd,
        };
    }
}
