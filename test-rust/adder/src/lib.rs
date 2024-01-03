pub struct Rectangle {
    width: i32,
    length: i32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool { 
        self.width > other.width && self.length > other.length
    }
}
pub fn greeting(name:String) -> String {
    format!("hello {}!", name)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 1,
            length: 1,
        };
        let smaller = Rectangle { 
            width: 0,
            length: 0,
        };
        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width: 1,
            length: 1,
        };
        let smaller = Rectangle { 
            width: 0,
            length: 0,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test] 
    fn greeting_contains_name() {
        let name = String::from("bogymen");
        let result = greeting(name);
        assert!(result.contains("bogymen"));
    }
}
 