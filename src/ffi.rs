use std::ffi::{CStr, CString};
// use crate::futures::websockets::*;
use std::os::raw::c_char;
use serde_json::Value;
use crate::account::Account;
use crate::api::Binance;
use crate::account::OrderSide;
use crate::account::OrderType;
use crate::account::TimeInForce;
use crate::general::General;
use crate::market::Market;

extern fn dummy(_: *const c_char) -> *mut c_char {
    std::ptr::null_mut()
}

static mut FUNC_CPP_FROM_RUST: extern fn(s: *const c_char) -> *mut c_char = dummy;
static mut API_KEY: Option<String> = None;
static mut SECRET_KEY: Option<String> = None;
static mut ACCOUNT: Option<Account> = None;
static mut GENERAL: Option<General> = None;
static mut MARKET: Option<Market> = None;

///
/// Must be called at beginning
/// 
pub fn init() {
    unsafe {
        API_KEY.get_or_insert_with(|| "YOUR_API_KEY".to_owned());
        SECRET_KEY.get_or_insert_with(|| "YOUR_SECRET_KEY".to_owned());
        ACCOUNT.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        GENERAL.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
        MARKET.get_or_insert(Binance::new(API_KEY.clone(), SECRET_KEY.clone()));
    }
}


#[no_mangle]
pub extern "C" fn initFromCpp(callback: extern fn(_: *const c_char) -> *mut c_char) -> i32 {
    unsafe {
        FUNC_CPP_FROM_RUST = callback;
    }
    0
}

#[no_mangle]
//pub fn limit_buy<S, F>(&self, symbol: S, qty: F, price: f64) -> Result<Transaction>
pub extern "C" fn limit_buy_rs(symbol: *const c_char, qty: *const c_char, price: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_qty = CStr::from_ptr(qty).to_str().unwrap();
        let rs_price = CStr::from_ptr(price).to_str().unwrap();
        
        let res = format!("{:?}", ACCOUNT.clone().unwrap().limit_buy(rs_symbol, rs_qty.parse::<f64>().unwrap(), rs_price.parse::<f64>().unwrap()));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
pub extern "C" fn rustFromCpp(s: *const c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(s);
        let rust_str = c_str.to_str().unwrap();
        let rust_args: Value = serde_json::from_str(rust_str).unwrap();
    
        FUNC_CPP_FROM_RUST(s);
        std::ptr::null_mut()
    }
}

#[no_mangle]
//pub fn cancel_order_with_client_id<S>(&self, symbol: S, orig_client_order_id: String) 
pub extern "C" fn cancel_order_with_client_id_rs(symbol: *const c_char, orig_client_order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let cstr = CStr::from_ptr(orig_client_order_id);
        let str_slice = cstr.to_str().expect("Invalid UTF-8 string");
        let rs_orig_client_order_id: String = String::from(str_slice);
        
        let res = format!("{:?}", ACCOUNT.clone().unwrap().cancel_order_with_client_id(rs_symbol, rs_orig_client_order_id));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn cancel_order<S>(&self, symbol: S, order_id: u64) -> Result<OrderCanceled>
pub extern "C" fn cancel_order_rs(symbol: *const c_char, order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_order_id: &str = CStr::from_ptr(order_id).to_str().unwrap();
        
        let res = format!("{:?}", ACCOUNT.clone().unwrap().cancel_order(rs_symbol, rs_order_id.parse::<u64>().unwrap()));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn custom_order<S, F>(&self, symbol: S, qty: F, price: f64, stop_price: Option<f64>, order_side: OrderSide,
//    order_type: OrderType, time_in_force: TimeInForce, new_client_order_id: Option<String>, ) -> Result<Transaction>
pub extern "C" fn custom_order_rs(
                    symbol: *const c_char, qty: *const c_char, price: *const c_char,
                    stop_price: *const c_char, order_side: *const c_char, order_type: *const c_char, 
                    time_in_force: *const c_char, new_client_order_id: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_qty = CStr::from_ptr(qty).to_str().unwrap();
        let rs_price = CStr::from_ptr(price).to_str().unwrap();
        let rs_stop_price = CStr::from_ptr(stop_price).to_str().unwrap();
        let rs_order_side = CStr::from_ptr(order_side).to_str().unwrap();
        let rs_order_type = CStr::from_ptr(order_type).to_str().unwrap();
        let rs_time_in_force: &str = CStr::from_ptr(time_in_force).to_str().unwrap();
        // let rs_new_client_order_id: &str = CStr::from_ptr(new_client_order_id).to_str().unwrap();
        
        let cstr = { CStr::from_ptr(new_client_order_id) };
        let str_slice = cstr.to_str().ok().unwrap();
        let rs_new_client_order_id = str_slice.to_owned();
        
        let res = format!("{:?}", ACCOUNT.clone().unwrap().custom_order(
            rs_symbol, 
            rs_qty.parse::<f64>().unwrap(),
            rs_price.parse::<f64>().unwrap(),
            Some(rs_stop_price.parse::<f64>().unwrap()),
            OrderSide::from_int(rs_order_side.parse::<i32>().unwrap()).unwrap(),
            OrderType::from_int(rs_order_type.parse::<i32>().unwrap()).unwrap(),
            TimeInForce::from_int(rs_time_in_force.parse::<i32>().unwrap()).unwrap(),
            Some(rs_new_client_order_id)
        ));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}


#[no_mangle]
// pub fn exchange_info(&self) -> Result<ExchangeInformation>
pub extern "C" fn exchange_info_rs() -> *mut c_char {
    unsafe {
        let res = format!("{:?}", GENERAL.clone().unwrap().exchange_info());
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderBook>
pub extern "C" fn get_custom_depth_rs(symbol: *const c_char, depth: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();
        let rs_depth = CStr::from_ptr(depth).to_str().unwrap();

        let res = format!("{:?}", MARKET.clone().unwrap().get_custom_depth(rs_symbol, rs_depth.parse::<u64>().unwrap()));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_price<S>(&self, symbol: S) -> Result<SymbolPrice>
pub extern "C" fn get_price_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = format!("{:?}", MARKET.clone().unwrap().get_price(rs_symbol));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}

#[no_mangle]
// pub fn get_book_ticker<S>(&self, symbol: S) -> Result<Tickers>
pub extern "C" fn get_book_ticker_rs(symbol: *const c_char) -> *mut c_char {
    unsafe {
        let rs_symbol = CStr::from_ptr(symbol).to_str().unwrap();

        let res = format!("{:?}", MARKET.clone().unwrap().get_book_ticker(rs_symbol));
        CString::new(res).unwrap().into_raw() as *mut c_char
    }
}