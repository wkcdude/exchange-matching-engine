mod order_book {
    use ordered_float::OrderedFloat;
    use std::collections::BTreeMap;

    #[allow(dead_code)]
    pub enum OrderType {
        LimitOrder,
    }

    #[allow(dead_code)]
    pub enum OrderSide {
        Buy,
        Sell,
    }

    #[allow(dead_code)]
    pub enum OrderLifeTime {
        GoodTilCancelOrder,
    }

    #[allow(dead_code)]
    pub struct OrderRecord {
        pub order_side: OrderSide,
        pub order_id: String,
        pub price: f64,
        pub initial_quantity: f64,
        pub remaining_quantity: f64,
        pub order_type: OrderType,
        pub order_life_time: OrderLifeTime,
    }

    #[allow(dead_code)]
    pub struct OrderEntry {
        pub queue: Vec<OrderRecord>,
        pub total_quantity: f64,
    }

    #[allow(dead_code)]
    impl OrderEntry {
        fn new() -> Self {
            OrderEntry {
                queue: Vec::new(),
                total_quantity: 0.0,
            }
        }
    }

    #[allow(dead_code)]
    pub struct OrderBook {
        pub buy_orders: BTreeMap<OrderedFloat<f64>, OrderEntry>,
        pub sell_orders: BTreeMap<OrderedFloat<f64>, OrderEntry>,
    }

    #[allow(dead_code)]
    impl OrderBook {
        pub fn new() -> Self {
            OrderBook {
                buy_orders: BTreeMap::new(),
                sell_orders: BTreeMap::new(),
            }
        }

        pub fn set_order(&mut self, mut order_record: OrderRecord) {
            let buy_orders = &mut self.buy_orders;
            if !buy_orders.contains_key(&OrderedFloat(order_record.price)) {
                buy_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
            }
            let buy_order_entry = buy_orders.get_mut(&OrderedFloat(order_record.price));

            let sell_orders = &mut self.sell_orders;
            if !sell_orders.contains_key(&OrderedFloat(order_record.price)) {
                sell_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
            }
            let sell_order_entry = sell_orders.get_mut(&OrderedFloat(order_record.price));

            if let (Some(buy), Some(sell)) = (buy_order_entry, sell_order_entry) {
                match order_record.order_side {
                    OrderSide::Buy => {
                            if sell.total_quantity < order_record.remaining_quantity {
                                order_record.remaining_quantity = order_record.remaining_quantity - sell.total_quantity;
                                sell.total_quantity = 0.00;
                            } 
                            
                            if sell.total_quantity > order_record.remaining_quantity {
                                sell.total_quantity = sell.total_quantity - order_record.remaining_quantity;
                                order_record.remaining_quantity = 0.0;
                            }

                            if order_record.remaining_quantity != 0.0 {
                                buy.total_quantity = buy.total_quantity + order_record.remaining_quantity;
                                buy.queue.push(order_record);                 
                            }
                    }
                    OrderSide::Sell => {
                            let quantity = order_record.remaining_quantity;
                            sell.queue.push(order_record);
                            sell.total_quantity += quantity;
                    }
                }
            }
        }
    }

    #[test]
    fn orderbook_test() {
        let mut orderbook = OrderBook::new();

        orderbook.set_order(OrderRecord {
            order_side: OrderSide::Sell,
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
            initial_quantity: 80.00,
            remaining_quantity: 80.00,
            order_type: OrderType::LimitOrder,
            order_life_time: OrderLifeTime::GoodTilCancelOrder,
        });

        let buy_order = orderbook.buy_orders.get(&OrderedFloat(100.0));
        match buy_order {
            Some(n) => {
                assert_eq!(n.total_quantity, 0.0);
                assert_eq!(n.queue.len(), 0);
            }
            _ => {}
        }

        let sell_order = orderbook.sell_orders.get(&OrderedFloat(100.0));
        match sell_order {
            Some(n) => {
                assert_eq!(n.total_quantity, 20.0);
                assert_eq!(n.queue.len(), 1);
            }
            _ => {}
        }


    }
}
