mod orderbook;

mod orderbook_test {
    use crate::orderbook::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn bound_order_test() {
        let mut orderbook = OrderBook::new();

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Sell,
                "1".to_string(),
                130.0,
                100.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Sell,
                "2".to_string(),
                120.0,
                100.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Sell,
                "3".to_string(),
                110.0,
                100.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );

        let _ = orderbook.get_remaining_available_quantity(
            &(OrderRecord {
                order_side: OrderSide::Buy,
                order_id: "4".to_string(),
                price: 120.0,
                initial_quantity: 300.0,
                remaining_quantity: 300.0,
                order_type: OrderType::LimitOrder,
                order_life_time: OrderLifeTime::GoodTilCancel,
                order_status: OrderStatus::New,
            })
        );
    }

    #[test]
    fn limit_order_test() {
        let mut orderbook = OrderBook::new();

        let test_price = 9999.0;

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Buy,
                "1".to_string(),
                test_price,
                130.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 130.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Buy,
                "2".to_string(),
                test_price,
                130.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 2);
            assert_eq!(buy_entry.total_quantity, 260.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Sell,
                "3".to_string(),
                test_price,
                50.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 2);
            assert_eq!(buy_entry.total_quantity, 210.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(
            &mut OrderRecord::new(
                OrderSide::Sell,
                "4".to_string(),
                test_price,
                80.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 130.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.cancel_order(
            &mut OrderRecord::new(
                OrderSide::Buy,
                "2".to_string(),
                test_price,
                0.0,
                OrderType::LimitOrder,
                OrderLifeTime::GoodTilCancel
            )
        );
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 0);
            assert_eq!(buy_entry.total_quantity, 0.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }
    }
}
