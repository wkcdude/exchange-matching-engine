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
#[derive(Clone)]
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

impl OrderRecord {
    pub fn new(
        order_side: OrderSide,
        order_id: String,
        price: f64,
        initial_quantity: f64,
        order_type: OrderType,
        order_life_time: OrderLifeTime
    ) -> Self {
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

    fn init_entry(&mut self, incoming_order: &mut OrderRecord) {
        if !self.buy_orders.contains_key(&OrderedFloat(incoming_order.price)) {
            self.buy_orders.insert(OrderedFloat(incoming_order.price), OrderEntry::new());
        }

        if !self.sell_orders.contains_key(&OrderedFloat(incoming_order.price)) {
            self.sell_orders.insert(OrderedFloat(incoming_order.price), OrderEntry::new());
        }
    }

    fn get_buy_order_entry(&mut self, price: f64) -> Option<&mut OrderEntry> {
        self.buy_orders.get_mut(&OrderedFloat(price))
    }

    fn get_sell_order_entry(&mut self, price: f64) -> Option<&mut OrderEntry> {
        self.sell_orders.get_mut(&OrderedFloat(price))
    }

    pub fn match_order(&mut self, incoming_order: &mut OrderRecord) {
        match incoming_order.order_side {
            OrderSide::Buy => {
                if let Some(sell_entry) = self.get_sell_order_entry(incoming_order.price) {
                    while
                        sell_entry.total_quantity != 0.0 &&
                        incoming_order.remaining_quantity != 0.0
                    {
                        if let Some(mut sell_order_record) = sell_entry.queue.pop_front() {
                            sell_entry.total_quantity -= sell_order_record.remaining_quantity;
                            incoming_order.remaining_quantity -=
                                sell_order_record.remaining_quantity;

                            if incoming_order.remaining_quantity < 0.0 {
                                sell_entry.total_quantity -= incoming_order.remaining_quantity;
                                sell_order_record.remaining_quantity =
                                    -incoming_order.remaining_quantity;
                                sell_entry.queue.push_front(sell_order_record);
                                incoming_order.remaining_quantity = 0.0;
                            }
                        }
                    }
                }
            }
            OrderSide::Sell => {
                if let Some(buy_entry) = self.get_buy_order_entry(incoming_order.price) {
                    while
                        buy_entry.total_quantity != 0.0 &&
                        incoming_order.remaining_quantity != 0.0
                    {
                        if let Some(mut buy_order_record) = buy_entry.queue.pop_front() {
                            buy_entry.total_quantity -= buy_order_record.remaining_quantity;
                            incoming_order.remaining_quantity -=
                                buy_order_record.remaining_quantity;

                            if incoming_order.remaining_quantity < 0.0 {
                                buy_entry.total_quantity -= incoming_order.remaining_quantity;
                                buy_order_record.remaining_quantity =
                                    -incoming_order.remaining_quantity;
                                buy_entry.queue.push_front(buy_order_record);
                                incoming_order.remaining_quantity = 0.0;
                            }
                        }
                    }
                }
            }
        }

        incoming_order.refresh_order_status();
    }

    fn set_order(&mut self, incoming_order: &mut OrderRecord) {
        if let OrderStatus::Filled = incoming_order.order_status {
            return;
        }

        match incoming_order.order_side {
            OrderSide::Buy => {
                if let Some(buy_entry) = self.get_buy_order_entry(incoming_order.price) {
                    buy_entry.total_quantity =
                        buy_entry.total_quantity + incoming_order.remaining_quantity;
                    buy_entry.queue.push_back(incoming_order.clone());
                }
            }
            OrderSide::Sell => {
                if let Some(sell_entry) = self.get_sell_order_entry(incoming_order.price) {
                    sell_entry.total_quantity =
                        sell_entry.total_quantity + incoming_order.remaining_quantity;
                    sell_entry.queue.push_back(incoming_order.clone());
                }
            }
        }
    }

    pub fn cancel_order(&mut self, incoming_order: &mut OrderRecord) {
        match incoming_order.order_side {
            OrderSide::Buy => {
                if let Some(buy_entry) = self.get_buy_order_entry(incoming_order.price) {
                    if
                        let Some(pos) = buy_entry.queue
                            .iter()
                            .position(|entry_order| entry_order.order_id == incoming_order.order_id)
                    {
                        if let Some(entry_record) = buy_entry.queue.get(pos) {
                            buy_entry.total_quantity -= entry_record.remaining_quantity;
                            buy_entry.queue.remove(pos);
                        }
                    }
                }
            }
            OrderSide::Sell => {
                if let Some(sell_entry) = self.get_sell_order_entry(incoming_order.price) {
                    if
                        let Some(pos) = sell_entry.queue
                            .iter()
                            .position(|entry_order| entry_order.order_id == incoming_order.order_id)
                    {
                        if let Some(entry_record) = sell_entry.queue.get(pos) {
                            sell_entry.total_quantity -= entry_record.remaining_quantity;
                            sell_entry.queue.remove(pos);
                        }
                    }
                }
            }
        }
    }

    pub fn create_order(&mut self, ordered: &mut OrderRecord) {
        self.init_entry(ordered);
        self.match_order(ordered);
        self.set_order(ordered);
    }
}
