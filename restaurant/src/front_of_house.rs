pub mod hosting;

mod serving {
    fn take_order() {
      println!("Taking order")
    }

    fn serve_order() {
      println!("Serving order")
    }

    fn take_payment() {
      println!("Taking payment")
    }

    pub fn serve() {
        take_order();
        serve_order();
        take_payment();
    }
}
