use crate::models::{Id, Item, Store};
use std::collections::HashMap;
use warp::{http, Rejection, Reply};

pub async fn update_grocery_list(item: Item, store: Store) -> Result<impl Reply, Rejection> {
    store.grocery_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

pub async fn delete_grocery_list_item(id: Id, store: Store) -> Result<impl Reply, Rejection> {
    store.grocery_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}

pub async fn get_grocery_list(store: Store) -> Result<impl Reply, Rejection> {
    let mut result = HashMap::new();
    let r = store.grocery_list.read();

    for (key, value) in r.iter() {
        result.insert(key.clone(), *value);
    }

    Ok(warp::reply::json(&result))
}
