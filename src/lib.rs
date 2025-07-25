mod order_book {
    use ordered_float::OrderedFloat;
    use std::collections::{ BTreeMap, VecDeque };

    #[allow(dead_code)]
    #[derive(Clone)]
    pub enum OrderType {
        MarketOrder,
        LimitOrder,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub enum OrderSide {
        Buy,
        Sell,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
    pub enum OrderLifeTime {
        GoodTilCancel,
        FillOrKill,
        ImmidiateOrCancel,
    }

    #[allow(dead_code)]
    #[derive(Clone)]
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
    #[derive(Clone)]
    pub struct OrderEntry {
        pub queue: VecDeque<OrderRecord>,
        pub total_quantity: f64,
    }

    #[allow(dead_code)]
    impl OrderEntry {
        fn new() -> Self {
            OrderEntry {
                queue: VecDeque::new(),
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

        fn init_entry(&mut self, order_record: &mut OrderRecord) {
            if !self.buy_orders.contains_key(&OrderedFloat(order_record.price)) {
                self.buy_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
            }

            if !self.sell_orders.contains_key(&OrderedFloat(order_record.price)) {
                self.sell_orders.insert(OrderedFloat(order_record.price), OrderEntry::new());
            }
        }

        pub fn match_order(&mut self, ordered: &mut OrderRecord) {
            let buy_entry = self.buy_orders.get_mut(&OrderedFloat(ordered.price));
            let sell_entry = self.sell_orders.get_mut(&OrderedFloat(ordered.price));
            if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
                match ordered.order_side {
                    OrderSide::Buy => {
                        while sell_entry.total_quantity <= 0.0 || ordered.remaining_quantity <= 0.0 {
                            if let Some(mut sell_order_record) = sell_entry.queue.pop_front() {
                                sell_entry.total_quantity -= sell_order_record.remaining_quantity;
                                ordered.remaining_quantity -= sell_order_record.remaining_quantity;

                                if ordered.remaining_quantity < 0.0 {
                                    sell_entry.total_quantity -= ordered.remaining_quantity;

                                    sell_order_record.remaining_quantity -=
                                        ordered.remaining_quantity;
                                    sell_entry.queue.push_front(sell_order_record);
                                    ordered.remaining_quantity = 0.0;
                                }
                                continue;
                            }
                            
                            break;
                        }
                    }
                    OrderSide::Sell => {
                        while buy_entry.total_quantity <= 0.0 || ordered.remaining_quantity <= 0.0 {                            
                            if let Some(mut buy_order_record) = buy_entry.queue.pop_front() {
                                buy_entry.total_quantity -= buy_order_record.remaining_quantity;
                                ordered.remaining_quantity -= buy_order_record.remaining_quantity;

                                if ordered.remaining_quantity < 0.0 {
                                    buy_entry.total_quantity -= ordered.remaining_quantity;
                                    println!("{:?}", ordered.remaining_quantity);
                                    println!("{:?}", buy_entry.total_quantity);

                                    buy_order_record.remaining_quantity -=
                                        ordered.remaining_quantity;
                                    buy_entry.queue.push_front(buy_order_record);
                                    ordered.remaining_quantity = 0.0;
                                }
                                continue;
                            }

                            break;
                        }
                    }
                }
            }
        }

        pub fn create_order(&mut self, ordered: &mut OrderRecord) {
            self.init_entry(ordered);

            self.match_order(ordered);

            let buy_entry = self.buy_orders.get_mut(&OrderedFloat(ordered.price));
            let sell_entry = self.sell_orders.get_mut(&OrderedFloat(ordered.price));

            if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
                match ordered.order_side {
                    OrderSide::Buy => {
                        if ordered.remaining_quantity != 0.0 {
                            buy_entry.total_quantity =
                                buy_entry.total_quantity + ordered.remaining_quantity;
                            buy_entry.queue.push_back(ordered.clone());
                        }
                    }
                    OrderSide::Sell => {
                        if ordered.remaining_quantity != 0.0 {
                            sell_entry.total_quantity =
                                sell_entry.total_quantity + ordered.remaining_quantity;
                            sell_entry.queue.push_back(ordered.clone());
                        }
                    }
                }
            }
        }

        pub fn cancel_order(&mut self, order_record: OrderRecord) {}
    }

    #[test]
    fn limit_order_test() {
        let mut orderbook = OrderBook::new();

        orderbook.create_order(
            &mut (OrderRecord {
                order_side: OrderSide::Buy,
                order_id: "".to_string(),
                price: 9990.0,
                initial_quantity: 130.0,
                remaining_quantity: 130.0,
                order_type: OrderType::LimitOrder,
                order_life_time: OrderLifeTime::GoodTilCancel,
            })
        );

        orderbook.create_order(
            &mut (OrderRecord {
                order_side: OrderSide::Sell,
                order_id: "".to_string(),
                price: 9990.0,
                initial_quantity: 100.0,
                remaining_quantity: 100.0,
                order_type: OrderType::LimitOrder,
                order_life_time: OrderLifeTime::GoodTilCancel,
            })
        );


        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(9990.0));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(9990.0));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 30.0);
            
            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
            

            // assert_eq!(sell_entry.total_quantity, 100.0);
        }
    }
}
