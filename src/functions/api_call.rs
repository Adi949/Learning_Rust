use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

// #[tokio::main]
pub fn get() -> Result<String, Error> {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Todo {
        #[serde(rename = "userId")]
        user_id: i64,
        id: i64,
        title: String,
        completed: bool,
    }

    let first = Todo {
        user_id: 1,
        id: 1,
        title: "uhad".to_string(),
        completed: true,
    };
    let json: String = serde_json::to_string(&first).expect("Serialization failed");

    Ok(json)

    // println!("{:#?}", body);
}
