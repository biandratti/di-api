use warp::Filter;

mod models;
mod handlers;

#[tokio::main]
async fn main() {
    let store = models::Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::update_grocery_list);

    let get_items = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(handlers::get_grocery_list);

    let delete_item = warp::delete()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::delete_grocery_list_item);

    let update_item = warp::put()
        .and(warp::path("v1"))
        .and(warp::path("groceries"))
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(handlers::update_grocery_list);

    let routes = add_items.or(get_items).or(delete_item).or(update_item);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
