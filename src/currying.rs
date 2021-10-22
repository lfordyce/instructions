use futures::future::BoxFuture;
use std::future::Future;

#[derive(Clone)]
struct Item {
    id: String,
    name: String,
}

async fn find_by_name(name: String) -> Option<Item> {
    println!("fetching item by name: {}", name);
    None
    // Some(Item{ id: "1".to_string(), name: "Jim".to_string()})
}

fn find_by_name_cur(conn: String) -> impl FnOnce(String) -> BoxFuture<'static, Option<Item>> {
    move |name: String| {
        Box::pin(async move {
            println!(
                "fetching item by name: {} with connection: {}",
                name.clone(),
                conn.clone()
            );
            None as Option<Item>
        })
    }
}

async fn insert(item: Item) -> () {
    println!("inserting item {} {}", &item.id, &item.name)
}

async fn create_item<FA, FB>(
    find_by_name: impl FnOnce(String) -> FA,
    insert: impl FnOnce(Item) -> FB,
    item: Item,
) -> ()
where
    FA: Future<Output = Option<Item>>,
    FB: Future<Output = ()>,
{
    let may_item = find_by_name(item.name.clone()).await;
    match may_item {
        Some(_) => (),
        None => insert(item.clone()).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn insert_curry_fn() {
        let conn = "db_connection".to_string();
        let _ = create_item(
            find_by_name_cur(conn.clone()),
            insert,
            Item {
                id: "1".to_string(),
                name: "Jim".to_string(),
            },
        )
        .await;
    }
}
