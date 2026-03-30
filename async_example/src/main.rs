use tokio::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use futures::future::join_all;

/// A simple async function that demonstrates non-blocking I/O
pub async fn greet(name: &str) -> String {
    // Simulate async network call
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Hello, {}!", name)
}

pub async fn concurrent_greetings(names: &[&str]) -> Vec<String> {
    let handles: Vec<_> = names.iter().map(|&name| {
        let name_owned = name.to_string(); // make an owned String
        tokio::spawn(async move { greet(&name_owned).await })
    }).collect();

    futures::future::join_all(handles)
        .await
        .into_iter()
        .filter_map(|res| match res {
            Ok(value) => Some(value),
            Err(e) => {
                eprintln!("Task failed: {}", e);
                None
            }
        })
        .collect()
}

/// Demonstrates shared mutable state with async
pub struct SharedCounter {
    lock: Mutex<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        Self {
            lock: Mutex::new(0),
        }
    }
    
    pub async fn increment(&self) {
        let mut locked = self.lock.lock().await;
        *locked += 1;
    }
    
    pub async fn get_value(&self) -> i32 {
        *self.lock.lock().await
    }
}

/// Demonstrates error handling in async context
pub async fn process_with_retry<F>(
    attempts: u32,
    mut operation: F,
) -> Result<String, String>
where
    F: FnMut() -> Result<String, String> + Send,
{
    for attempt in 1..=attempts {
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) => {
                println!("Attempt {} failed: {}", attempt, e);
                if attempt < attempts {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }
    Err("All attempts failed".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex as StdMutex;

    #[tokio::test]
    async fn test_greet() {
        let result = greet("Rust").await;
        assert_eq!(result, "Hello, Rust!");
    }

    #[tokio::test]
    async fn test_concurrent_greetings() {
        let names = &["Alice", "Bob", "Charlie"];
        let results = concurrent_greetings(names).await;
        assert_eq!(results.len(), 3);
        assert!(results.contains(&"Hello, Alice!".to_string()));
    }

    #[tokio::test]
    async fn test_shared_counter() {
        let counter = Arc::new(SharedCounter::new());
        let mut handles = Vec::new();
        
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            handles.push(tokio::spawn(async move {
                counter_clone.increment().await;
            }));
        }

        for handle in handles {
            handle.await.unwrap();
        }

        assert_eq!(counter.get_value().await, 10);
    }

    #[tokio::test]
    async fn test_retry_success() {
        let attempts_counter = Arc::new(StdMutex::new(0));

        let counter_clone = Arc::clone(&attempts_counter);
        let mut operation = move || {
            let mut attempts = counter_clone.lock().unwrap();
            *attempts += 1;
            if *attempts < 3 {
                Err("Temporary failure".to_string())
            } else {
                Ok("Success!".to_string())
            }
        };

        let result = process_with_retry(5, &mut operation).await;
        assert_eq!(result, Ok("Success!".to_string()));
        assert_eq!(*attempts_counter.lock().unwrap(), 3);
    }
}

#[tokio::main]
async fn main() {
    println!("Async Rust Example");

    let names = &["Alice", "Bob", "Charlie"];
    let results = concurrent_greetings(names).await;

    for greeting in results {
        println!("{}", greeting);
    }
}