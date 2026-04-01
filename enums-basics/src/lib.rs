// Library module for enum examples that can be tested

#[derive(Debug, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
    
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    pub fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

pub enum Shape {
    Circle(f64),        // radius
    Rectangle(f64, f64), // width, height
    Square(f64),        // side
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    }
    
    pub fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle(width, height) => 2.0 * (width + height),
            Shape::Square(side) => 4.0 * side,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }
    
    #[test]
    fn test_traffic_light_next() {
        assert_eq!(TrafficLight::Red.next(), TrafficLight::Green);
        assert_eq!(TrafficLight::Green.next(), TrafficLight::Yellow);
        assert_eq!(TrafficLight::Yellow.next(), TrafficLight::Red);
    }
    
    #[test]
    fn test_coin_values() {
        assert_eq!(Coin::Penny.value_in_cents(), 1);
        assert_eq!(Coin::Nickel.value_in_cents(), 5);
        assert_eq!(Coin::Dime.value_in_cents(), 10);
        assert_eq!(Coin::Quarter.value_in_cents(), 25);
    }
    
    #[test]
    fn test_shape_area() {
        let circle = Shape::Circle(2.0);
        let rectangle = Shape::Rectangle(3.0, 4.0);
        let square = Shape::Square(5.0);
        
        // Using approximate equality for floating point
        assert!((circle.area() - 12.56637).abs() < 0.0001);
        assert_eq!(rectangle.area(), 12.0);
        assert_eq!(square.area(), 25.0);
    }
    
    #[test]
    fn test_shape_perimeter() {
        let circle = Shape::Circle(2.0);
        let rectangle = Shape::Rectangle(3.0, 4.0);
        let square = Shape::Square(5.0);
        
        assert!((circle.perimeter() - 12.56637).abs() < 0.0001);
        assert_eq!(rectangle.perimeter(), 14.0);
        assert_eq!(square.perimeter(), 20.0);
    }
    
    #[test]
    fn test_option_enum() {
        let some_value: Option<i32> = Some(42);
        let no_value: Option<i32> = None;
        
        assert_eq!(some_value.unwrap(), 42);
        assert_eq!(no_value.unwrap_or(100), 100);
        
        // Test pattern matching with Option
        match some_value {
            Some(value) => assert_eq!(value, 42),
            None => panic!("Expected Some(42)"),
        }
        
        match no_value {
            Some(_) => panic!("Expected None"),
            None => assert!(true), // Expected
        }
    }
    
    #[test]
    fn test_result_enum() {
        let ok_result: Result<i32, String> = Ok(42);
        let err_result: Result<i32, String> = Err("Something went wrong".to_string());
        
        assert_eq!(ok_result.unwrap(), 42);
        assert_eq!(err_result.unwrap_or(100), 100);
        
        // Test pattern matching with Result
        match ok_result {
            Ok(value) => assert_eq!(value, 42),
            Err(_) => panic!("Expected Ok(42)"),
        }
        
        match err_result {
            Ok(_) => panic!("Expected Err"),
            Err(e) => assert_eq!(e, "Something went wrong"),
        }
    }
}