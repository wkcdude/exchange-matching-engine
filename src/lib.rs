mod orderbook;

mod orderbook_test {
    use crate::orderbook::*;
    use ordered_float::OrderedFloat;
    use uuid::Uuid as uuid;

    #[test]
    fn limit_order_test() {
        let mut orderbook = OrderBook::new();

        let test_price = 9999.0;

        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Buy,
            uuid::new_v4().to_string(),
            test_price,
            130.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        
        let binding = OrderEntry::default();

        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price)).unwrap_or(&binding);
        assert_eq!(buy_entry.queue.len(), 1);
        assert_eq!(buy_entry.total_quantity, 130.0);
       
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price)).unwrap_or(&binding);
        assert_eq!(sell_entry.queue.len(), 0);
        assert_eq!(sell_entry.total_quantity, 0.0);
    
        let cancel_order_id = uuid::new_v4().to_string();
        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Buy,
            cancel_order_id.clone(),
            test_price,
            130.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        
        let binding = OrderEntry::default();

        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price)).unwrap_or(&binding);
        assert_eq!(buy_entry.queue.len(), 2);
        assert_eq!(buy_entry.total_quantity, 260.0);
       
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price)).unwrap_or(&binding);
        assert_eq!(sell_entry.queue.len(), 0);
        assert_eq!(sell_entry.total_quantity, 0.0);

        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Sell,
            uuid::new_v4().to_string(),
            test_price,
            50.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        if let Some(buy_entry) = orderbook.buy_orders.get(&OrderedFloat(test_price)) {
            assert_eq!(buy_entry.queue.len(), 2);
            assert_eq!(buy_entry.total_quantity, 210.0);
        }
        if let Some(sell_entry) = orderbook.sell_orders.get(&OrderedFloat(test_price)) {
            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Sell,
            uuid::new_v4().to_string(),
            test_price,
            80.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 130.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.cancel_order(&mut OrderRecord::new(
            OrderSide::Buy,
            cancel_order_id.clone(),
            test_price,
            0.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 0);
            assert_eq!(buy_entry.total_quantity, 0.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Buy,
            uuid::new_v4().to_string(),
            test_price,
            100.0,
            OrderType::LimitOrder,
            OrderLifeTime::GoodTilCancel,
        ));
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 100.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }

        orderbook.create_order(&mut OrderRecord::new(
            OrderSide::Sell,
            uuid::new_v4().to_string(),
            test_price,
            120.0,
            OrderType::LimitOrder,
            OrderLifeTime::FillOrKill,
        ));
        let buy_entry = orderbook.buy_orders.get(&OrderedFloat(test_price));
        let sell_entry = orderbook.sell_orders.get(&OrderedFloat(test_price));
        if let (Some(buy_entry), Some(sell_entry)) = (buy_entry, sell_entry) {
            assert_eq!(buy_entry.queue.len(), 1);
            assert_eq!(buy_entry.total_quantity, 100.0);

            assert_eq!(sell_entry.queue.len(), 0);
            assert_eq!(sell_entry.total_quantity, 0.0);
        }
    }
}
