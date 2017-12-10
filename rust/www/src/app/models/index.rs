#[derive(Serialize, Deserialize)]
pub struct Index {
	id : i64,
	name : String,
	pets : Vec<String>
}

impl Index {
	pub fn get_by_id(id : i64) -> Index {
		let index = Index {
			id : id,
			name : "Elise".to_string(),
			pets : vec!["fish".to_string(), "dog".to_string()]
		};
		index
	}
}
