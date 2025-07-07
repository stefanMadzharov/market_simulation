use core::order_book::OrderBook;

fn main() {
    let mut order_book = OrderBook::with_comodity("Gold");
    order_book.add_sell(1.0, 1.0);
    order_book.add_buy(1.0, 1.0);
    dbg!(order_book);
}
