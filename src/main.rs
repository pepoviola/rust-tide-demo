use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tide::{Body, Request, Response};
use tide::prelude::*;
use uuid::Uuid;

// We need to implement the "Clone" trait in order to
// call the "cloned" method in the "get_dogs" route.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Dog {
    id: Option<String>,
    breed: String,
    name: String,
}

type DogMap = HashMap<String, Dog>;

#[derive(Clone)]
struct State {
    dog_map: Arc<RwLock<DogMap>>
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut dog_map: HashMap<String, Dog> = HashMap::new();

    let id = Uuid::new_v4().to_string();
    let dog = Dog {
        id: Some(id.clone()),
        name: "Comet".to_string(),
        breed: "Whippet".to_string(),
    };
    dog_map.insert(id, dog);

    let state = State { dog_map: Arc::new(RwLock::new(dog_map)) };
    let mut app = tide::with_state(state);

    // Can test this with:
    // curl http://localhost:1234/dog
    app.at("/dog")
        .get(|req: Request<State>| async move {
            println!("get entered");
            let dog_map = &req.state().clone().dog_map.read().await;
            let dogs: Vec<Dog> = dog_map.values().cloned().collect();
            let mut res = Response::new(200);
            res.set_body(Body::from_json(&dogs)?);
            Ok(res)
        });

    // Can test this with:
    // curl -X POST -H 'Content-Type: application/json' -d '{"name": "Oscar", "breed": "GSP"}' http://localhost:1234/dog
    app.at("/dog")
        .post(|mut req: Request<State>| async move {
            println!("post entered"); // This is output.
            let mut dog: Dog = req.body_json().await?;
            let id = Uuid::new_v4().to_string();
            dog.id = Some(id);
            println!("post got dog"); // This is not output.
            let mut dog_map = &req.state().clone().dog_map.write().await;
            dog_map.insert( dog.id.clone(), dog.clone());
            let mut res = tide::Response::new(200);
            res.set_body(Body::from_json(&dog)?);
            Ok(res)
        });

    app.listen("127.0.0.1:1234").await?;
    Ok(())
}
