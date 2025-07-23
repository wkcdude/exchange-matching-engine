mod order_book {
    use ordered_float::OrderedFloat;
    use std::collections::BTreeMap;

    pub enum OrderType {
        LimitOrder,
    }

    pub enum OrderSide {
        Buy,
        Sell,
    }

    pub enum OrderLifeTime {
        GoodTilCancelOrder,
    }

    pub struct OrderRecord {
        pub order_side: OrderSide,
        pub order_id: String,
        pub price: f64,
        pub initial_quantity: f64,
        pub remaining_quantity: f64,
        pub order_type: OrderType,
        pub order_life_time: OrderLifeTime,
    }

    pub struct OrderEntry {
        pub queue: Vec<OrderRecord>,
        pub total_quantity: f64,
    }

    impl OrderEntry {
        fn new() -> Self {
            OrderEntry {
                queue: Vec::new(),
                total_quantity: 0.0,
            }
        }
    }

    pub struct OrderBook {
        pub buy_orders: BTreeMap<OrderedFloat<f64>, OrderEntry>,
        pub sell_orders: BTreeMap<OrderedFloat<f64>, OrderEntry>,
    }

    impl OrderBook {
        pub fn new() -> Self {
            OrderBook {
                buy_orders: BTreeMap::new(),
                sell_orders: BTreeMap::new(),
            }
        }

        pub fn set_order(&mut self, order_record: OrderRecord) {
            match order_record.order_side {
                OrderSide::Buy => {
                    let buy_orders = &mut self.buy_orders;
                    if !buy_orders.contains_key(&OrderedFloat(order_record.price)) {
                        buy_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
                    }

                    let order_entry = buy_orders.get_mut(&OrderedFloat(order_record.price));
                    match order_entry {
                        Some(oe) => {
                            let quantity = order_record.remaining_quantity;
                            oe.queue.push(order_record);
                            oe.total_quantity += quantity;
                        }
                        _ => {}
                    }
                }
                OrderSide::Sell => {
                    let sell_orders = &mut self.sell_orders;
                    if !sell_orders.contains_key(&OrderedFloat(order_record.price)) {
                        sell_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
                    }

                    let order_entry = sell_orders.get_mut(&OrderedFloat(order_record.price));
                    match order_entry {
                        Some(oe) => {
                            let quantity = order_record.remaining_quantity;
                            oe.queue.push(order_record);
                            oe.total_quantity += quantity;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn orderbook_test() {
        let mut orderbook = OrderBook::new();

        orderbook.set_order(OrderRecord {
            order_side: OrderSide::Buy,
            order_id: "1".to_string(),
            price: 100.00,
            initial_quantity: 100.00,
            remaining_quantity: 100.00,
            order_type: OrderType::LimitOrder,
            order_life_time: OrderLifeTime::GoodTilCancelOrder,
        });

        orderbook.set_order(OrderRecord {
            order_side: OrderSide::Buy,
            order_id: "2".to_string(),
            price: 100.00,
            initial_quantity: 100.00,
            remaining_quantity: 100.00,
            order_type: OrderType::LimitOrder,
            order_life_time: OrderLifeTime::GoodTilCancelOrder,
        });

        orderbook.set_order(OrderRecord {
            order_side: OrderSide::Buy,
            order_id: "3".to_string(),
            price: 100.00,
            initial_quantity: 100.00,
            remaining_quantity: 100.00,
            order_type: OrderType::LimitOrder,
            order_life_time: OrderLifeTime::GoodTilCancelOrder,
        });

        let buy_order = orderbook.buy_orders.get(&OrderedFloat(100.0));
        match buy_order {
            Some(n) => {
                assert_eq!(n.total_quantity, 300.0);
            }
            _ => {}
        }
    }
}
