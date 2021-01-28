//use parking_lot::RwLock;
use std::collections::HashMap;
//use std::sync::Arc;
use tide::{Body, Request, Response};
use tide::prelude::*;
use uuid::Uuid;

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: String,
    breed: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct NewDog {
    breed: String,
    name: String,
}

//type DogMap = Arc<RwLock<HashMap<String, Dog>>>;
type DogMap = HashMap<String, Dog>;

#[derive(Clone,Debug)]
struct State {
    dog_map: DogMap
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut dog_map: HashMap<String, Dog> = HashMap::new();

    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: id.clone(),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.insert(id, dog);

    //let state = State { dog_map: Arc::new(RwLock::new(dog_map)) };
    let state = State { dog_map };
    let mut app = tide::with_state(state);

    app.at("/dog")
        .get(|req: Request<State>| async move {
            //let dog_map = req.state().dog_map.read().await;
            let dog_map = &req.state().dog_map;
            let dogs: Vec<Dog> = dog_map.values().cloned().collect();
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&dogs)?);
            Ok(res)
        });

    app.at("/dog")
        // The next line gives the error "expected `()`,
        // found enum `std::result::Result`".
        .post(|req: Request<State>| async move {
            let dog: Dog = req.body_json().await.unwrap();
            let mut dog_map = &req.state().dog_map;
            dog_map.insert(dog.id, dog);
            // The next line gives the error "no method named `body_json`
            // found for struct `tide::Response` in the current scope".
            // but it looks just like an example here:
            // https://blog.yoshuawuyts.com/tide/
            tide::Response::new(200).body_json(&dog).unwrap();
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
