use rand::Rng;
use serde::Deserialize;
use serde_json;
use tokio::sync::mpsc;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Post {
    #[serde(rename = "postId")]
    post_id: u32,
    id: u32,
    name: String,
    email: String,
    body: String,
}

pub type Error = Box<(dyn std::error::Error + Send + Sync + 'static)>;

async fn generator(n_jobs: u32) -> Result<mpsc::Receiver<u32>, Error> {
    let (mut tx, rx) = mpsc::channel(10);
    let mut rng = rand::thread_rng();
    let numbs: Vec<u32> = (0..n_jobs)
        .map(|_| rng.gen_range(1..=100))
        .collect::<Vec<_>>();
    tokio::spawn(async move {
        for num in numbs {
            if let Err(err) = tx.send(num).await {
                eprintln!("receiver must be closed... {:?}", err);
                break;
            }
        }
    });
    Ok(rx)
}

async fn fan_out(
    mut rx_gen: mpsc::Receiver<u32>,
    n_jobs: u32,
) -> Result<mpsc::Receiver<String>, Error> {
    let (tx, rx) = mpsc::channel(n_jobs as usize);
    let mut handles = Vec::new();
    while let Some(num) = rx_gen.recv().await {
        let mut tx_num = tx.clone();
        let handle = tokio::spawn(async move {
            let rep = parse(num).await.unwrap();
            if let Err(err) = tx_num.send(rep).await {
                eprintln!("receiver must be closed... {:?}", err);
            }
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.await?;
    }
    Ok(rx)
}

async fn parse(id: u32) -> Result<String, Error> {
    const BASE: &'static str = "https://jsonplaceholder.typicode.com";
    let base = Url::parse(BASE).expect("hardcoded URL is known to be valid");
    let site = base.join(&format!("/comments/{}", id))?;
    let post: Post = reqwest::get(site).await?.json().await?;

    let the_result = format!("{} {} {}", post.post_id, post.email, post.body.len());

    Ok(the_result)
}

async fn fan_in(mut rx_fan_in: mpsc::Receiver<String>) -> Result<mpsc::Receiver<String>, Error> {
    let (mut tx, rx) = mpsc::channel(10);
    tokio::spawn(async move {
        while let Some(value) = rx_fan_in.recv().await {
            let processed_value = format!("{} _ Processed", value);
            if let Err(err) = tx.send(processed_value).await {
                eprintln!("receiver must be closed... {:?}", err);
                break;
            }
        }
    });
    Ok(rx)
}

#[tokio::test]
async fn test_fan_in_fan_out() {
    let n_jobs = 8;
    let mut rx_fan_in = fan_in(
        fan_out(generator(n_jobs).await.unwrap(), n_jobs)
            .await
            .unwrap(),
    )
    .await
    .unwrap();

    while let Some(value) = rx_fan_in.recv().await {
        println!("{}", value);
    }
}
