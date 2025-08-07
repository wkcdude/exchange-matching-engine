use ordered_float::OrderedFloat;
use std::collections::{BTreeMap, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OrderType {
    MarketOrder,
    LimitOrder,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OrderLifeTime {
    GoodTilCancel,
    FillOrKill,
    ImmidiateOrCancel,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OrderStatus {
    New,
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrderRecord {
    pub order_side: OrderSide,
    pub order_id: String,
    pub price: f64,
    pub initial_quantity: f64,
    pub remaining_quantity: f64,
    pub order_type: OrderType,
    pub order_life_time: OrderLifeTime,
    pub order_status: OrderStatus,
}

#[allow(dead_code)]
impl OrderRecord {
    pub fn new( order_side: OrderSide, order_id: String, price: f64, initial_quantity: f64, order_type: OrderType, order_life_time: OrderLifeTime, ) -> Self {
        OrderRecord {
            order_side,
            order_id,
            price,
            initial_quantity,
            remaining_quantity: initial_quantity,
            order_type,
            order_life_time,
            order_status: OrderStatus::New,
        }
    }

    pub fn refresh_order_status(&mut self) {
        if self.remaining_quantity == 0.0 {
            self.order_status = OrderStatus::Filled;
        }

        if self.remaining_quantity != 0.0 && self.remaining_quantity != self.initial_quantity {
            self.order_status = OrderStatus::PartiallyFilled;
        }

        if self.remaining_quantity == self.initial_quantity {
            self.order_status = OrderStatus::Open;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
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

    fn init_entry(&mut self, incoming_order: &mut OrderRecord) {
        if !self .buy_orders .contains_key(&OrderedFloat(incoming_order.price)) {
            self.buy_orders .insert(OrderedFloat(incoming_order.price), OrderEntry::new());
        }

        if !self .sell_orders .contains_key(&OrderedFloat(incoming_order.price)) {
            self.sell_orders .insert(OrderedFloat(incoming_order.price), OrderEntry::new());
        }
    }

    pub fn get_remaining_available_quantity(&self, incoming_order: &OrderRecord) -> f64 {
        let mut total_available_quantity = 0.0;

        match incoming_order.order_side {
            OrderSide::Buy => {
                for (_, order_entry) in self.sell_orders.range(..=&OrderedFloat(incoming_order.price)) {
                    total_available_quantity += order_entry.total_quantity;
                }
            }
            OrderSide::Sell => {
                for (_, order_entry) in self.buy_orders.range(&OrderedFloat(incoming_order.price)..) {
                    total_available_quantity += order_entry.total_quantity;
                }
            }
        }

        return total_available_quantity;
    }

    fn match_order(&mut self, incoming_order: &mut OrderRecord) {
        let order_entry = match incoming_order.order_side {
            OrderSide::Buy => &mut self.sell_orders,
            OrderSide::Sell => &mut self.buy_orders,
        };

        let prices: Vec<_> = match incoming_order.order_side {
            OrderSide::Buy => order_entry.keys().cloned().collect(),
            OrderSide::Sell => order_entry.keys().rev().cloned().collect(),
        };

        for price in prices {
            let price = price.into_inner();

            match incoming_order.order_side {
                OrderSide::Buy => {
                    if price > incoming_order.price {
                        break;
                    }
                },
                OrderSide::Sell => {
                    if price < incoming_order.price {
                        break;
                    }
                },
            }

            let entry = order_entry.get_mut(&OrderedFloat(price)).unwrap();

            while let Some(resting_order) = entry.queue.front_mut() {
                if incoming_order.remaining_quantity == 0.0 {
                    break;
                }

                let traded_qty = incoming_order.remaining_quantity.min(resting_order.remaining_quantity);
                incoming_order.remaining_quantity -= traded_qty;
                resting_order.remaining_quantity -= traded_qty;
                entry.total_quantity -= traded_qty;

                if resting_order.remaining_quantity == 0.0 {
                    entry.queue.pop_front();
                }

                if incoming_order.remaining_quantity == 0.0 {
                    break;
                }
            }

            if entry.queue.is_empty() {
                order_entry.remove(&OrderedFloat(price));
            }
        }

        incoming_order.refresh_order_status();
    }

    fn set_order(&mut self, incoming_order: &mut OrderRecord) {
        if let OrderStatus::Filled = incoming_order.order_status {
            return;
        }

        if let OrderLifeTime::ImmidiateOrCancel = incoming_order.order_life_time {
            if let OrderStatus::PartiallyFilled = incoming_order.order_status {
                return;
            }
        }

        match incoming_order.order_side {
            OrderSide::Buy => {
                if let Some(buy_entry) = self.buy_orders.get_mut(&OrderedFloat(incoming_order.price)) {
                    buy_entry.total_quantity = buy_entry.total_quantity + incoming_order.remaining_quantity;
                    buy_entry.queue.push_back(incoming_order.clone());
                }
            }
            OrderSide::Sell => {
                if let Some(sell_entry) = self.sell_orders.get_mut(&OrderedFloat(incoming_order.price)) {
                    sell_entry.total_quantity = sell_entry.total_quantity + incoming_order.remaining_quantity;
                    sell_entry.queue.push_back(incoming_order.clone());
                }
            }
        }
    }

    pub fn cancel_order(&mut self, incoming_order: &mut OrderRecord) {
        match incoming_order.order_side {
            OrderSide::Buy => {
                if let Some(buy_entry) = self.buy_orders.get_mut(&OrderedFloat(incoming_order.price)) {
                    if let Some(pos) = buy_entry.queue .iter().position(|entry_order| entry_order.order_id == incoming_order.order_id) {
                        if let Some(entry_record) = buy_entry.queue.get(pos) {
                            buy_entry.total_quantity -= entry_record.remaining_quantity;
                            buy_entry.queue.remove(pos);
                        }
                    }
                }
            }
            OrderSide::Sell => {
                if let Some(sell_entry) = self.sell_orders.get_mut(&OrderedFloat(incoming_order.price)) {
                    if let Some(pos) = sell_entry.queue .iter() .position(|entry_order| entry_order.order_id == incoming_order.order_id) {
                        if let Some(entry_record) = sell_entry.queue.get(pos) {
                            sell_entry.total_quantity -= entry_record.remaining_quantity;
                            sell_entry.queue.remove(pos);
                        }
                    }
                }
            }
        }
    }

    fn validate_incoming_order(&self, incoming_order: &mut OrderRecord) {
        let remaining_avaible_quantity = self.get_remaining_available_quantity(incoming_order);
        if let OrderLifeTime::FillOrKill = incoming_order.order_life_time {
            if remaining_avaible_quantity < incoming_order.remaining_quantity {
                return;
            }
        }
    }

    pub fn create_order(&mut self, incoming_order: &mut OrderRecord) {
        self.init_entry(incoming_order);
        self.validate_incoming_order(incoming_order);
        self.match_order(incoming_order);
        self.set_order(incoming_order);
    }
}
