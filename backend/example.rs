pub mod shop {
    include!(concat!(env!("OUT_DIR"), "/shop.rs"));
}

use prost::Message;
use redis::{Commands, RedisResult};
use shop::ShopItem;

fn main() -> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let item = ShopItem {
        id: 1,
        name: "T-shirt".to_string(),
        description: "A cool T-shirt".to_string(),
        price: 19.99,
        quantity_available: 100,
        tags: vec!["clothing".to_string(), "sale".to_string()],
        image_url: "https://img.freepik.com/free-photo/shallow-focus-vertical-shot-cute-golden-retriever-puppy-sitting-grass-ground_181624-27259.jpg?semt=ais_hybrid&w=740".to_string()
    };

    let key = format!("user:cart:{}", item_id);

    // Save Item to Redis Server
    let mut buf = Vec::new();
    item.encode(&mut buf).map_err(|e| {
        redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "Protobuf encoding failed",
            e.to_string(),
        ))
    })?;

    let _: () = con.set(&key, buf)?;

    // Get Item from Redis Server
    let data: Vec<u8> = con.get(&key)?;

    let item = shop::ShopItem::decode(&*data).map_err(|e| {
        redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "Protobuf decoding failed",
            e.to_string(),
        ))
    });

    // Print
    println!("The value for item is {}", item.unwrap().name);

    Ok(())
}
