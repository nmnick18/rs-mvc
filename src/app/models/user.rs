#[derive(RustcEncodable, RustcDecodable)]
pub struct User {
	id : i64,
	name : String,
	items : Vec<String>
}

impl User {
	pub fn get_by_id(id : i64) -> User {
		let user = User {
			id : id,
			name : "Nikhil".to_string(),
			items : vec!["xbox".to_string(), "chopsticks".to_string()]
		};
		user
	}
}