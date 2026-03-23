use std::sync::mpsc::{self, Receiver, Sender};

// -----------------------------
// 1) Builder Pattern
// -----------------------------
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    use_tls: bool,
    max_connections: usize,
}

#[derive(Debug)]
struct ServerConfigBuilder {
    host: String,
    port: u16,
    use_tls: bool,
    max_connections: usize,
}

impl ServerConfigBuilder {
    fn new() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            use_tls: false,
            max_connections: 100,
        }
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn tls(mut self, enabled: bool) -> Self {
        self.use_tls = enabled;
        self
    }

    fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    fn build(self) -> ServerConfig {
        ServerConfig {
            host: self.host,
            port: self.port,
            use_tls: self.use_tls,
            max_connections: self.max_connections,
        }
    }
}

// -----------------------------
// 2) Strategy Pattern
// -----------------------------
trait DiscountStrategy {
    fn apply(&self, price: f64) -> f64;
}

struct NoDiscount;
impl DiscountStrategy for NoDiscount {
    fn apply(&self, price: f64) -> f64 {
        price
    }
}

struct PercentageDiscount {
    percent: f64,
}
impl DiscountStrategy for PercentageDiscount {
    fn apply(&self, price: f64) -> f64 {
        price * (1.0 - self.percent / 100.0)
    }
}

struct Checkout {
    strategy: Box<dyn DiscountStrategy>,
}
impl Checkout {
    fn new(strategy: Box<dyn DiscountStrategy>) -> Self {
        Self { strategy }
    }

    fn total(&self, items_total: f64) -> f64 {
        self.strategy.apply(items_total)
    }
}

// -----------------------------
// 3) Command Pattern
// -----------------------------
trait Command {
    fn execute(&self, text: &mut String);
}

struct AppendCommand {
    suffix: String,
}
impl Command for AppendCommand {
    fn execute(&self, text: &mut String) {
        text.push_str(&self.suffix);
    }
}

struct UppercaseCommand;
impl Command for UppercaseCommand {
    fn execute(&self, text: &mut String) {
        *text = text.to_uppercase();
    }
}

// -----------------------------
// 4) Observer Pattern (channel-based)
// -----------------------------
struct EventBus {
    subscribers: Vec<Sender<String>>,
}

impl EventBus {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn subscribe(&mut self) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();
        self.subscribers.push(tx);
        rx
    }

    fn publish(&self, message: &str) {
        for sub in &self.subscribers {
            let _ = sub.send(message.to_string());
        }
    }
}

fn main() {
    // Builder
    let config = ServerConfigBuilder::new()
        .host("0.0.0.0")
        .port(443)
        .tls(true)
        .max_connections(5_000)
        .build();
    println!("builder -> {:?}", config);
    println!(
        "server runs on {}:{} tls={} max={}",
        config.host, config.port, config.use_tls, config.max_connections
    );

    // Strategy
    let regular = Checkout::new(Box::new(NoDiscount));
    let promo = Checkout::new(Box::new(PercentageDiscount { percent: 15.0 }));
    println!("strategy regular total: {}", regular.total(200.0));
    println!("strategy promo total: {}", promo.total(200.0));

    // Command
    let mut text = String::from("hello");
    let commands: Vec<Box<dyn Command>> = vec![
        Box::new(AppendCommand {
            suffix: " rust".to_string(),
        }),
        Box::new(UppercaseCommand),
    ];
    for cmd in commands {
        cmd.execute(&mut text);
    }
    println!("command result: {}", text);

    // Observer (publisher/subscribers)
    let mut bus = EventBus::new();
    let sub_a = bus.subscribe();
    let sub_b = bus.subscribe();
    bus.publish("Build started");
    bus.publish("Build finished");

    if let Ok(msg) = sub_a.recv() {
        println!("observer A got: {}", msg);
    }
    if let Ok(msg) = sub_b.recv() {
        println!("observer B got: {}", msg);
    }
}
