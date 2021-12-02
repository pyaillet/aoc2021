use std::fs;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Position {
    depth: i32,
    forward: i32,
}

impl Position {
    fn take_order(&mut self, order: Order) -> () {
        match order {
            Order::Forward(forward) => self.forward += forward,
            Order::Down(down) => self.depth += down,
            Order::Up(up) => self.depth -= up,
        };
    }
}

enum Order {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Order {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut order = s.split(" ");
        let order_text = order.next().unwrap();
        let order_value = order.next().unwrap().parse::<i32>().unwrap();
        match order_text {
            "forward" => Ok(Order::Forward(order_value)),
            "up" => Ok(Order::Up(order_value)),
            "down" => Ok(Order::Down(order_value)),
            _ => Err(()),
        }
    }
}

fn parse_orders(input: &str) -> Vec<Order> {
    input
        .lines()
        .map(|line| line.parse::<Order>().unwrap())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let orders = parse_orders(&contents);
    let mut position = Position::default();
    for order in orders {
        position.take_order(order);
    }
    println!("{}", position.depth * position.forward);
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let orders = super::parse_orders(input);
        let mut position = super::Position::default();
        for order in orders {
            position.take_order(order);
        }
        assert_eq!(position.depth * position.forward, 150);
    }
}
