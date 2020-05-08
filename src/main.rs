use gpio_cdev::{Chip, LineRequestFlags};
use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry::*;
use std::sync::{Arc, RwLock};
use warp::{Filter, http::StatusCode};

#[tokio::main]
async fn main() {
    let active_pins = BTreeMap::<GpioPath, Chip>::new();
    let shared_pins_state = Arc::new(RwLock::new(active_pins));
    let with_pins_state = warp::any().map(move || shared_pins_state.clone());

    let gpio_hello = warp::path!("gpio")
        .map(|| "This is the GPIO API");

    let gpio_modify = warp::post()
        .and(warp::path!("gpio" / String / u32))
        .and(with_pins_state)
        /* 1KB should be enough for anyone */
        .and(warp::body::content_length_limit(1024))
        .and(warp::body::json())
        .map(gpio_modify)
        .map(as_reply);

    let routes = gpio_hello.or(gpio_modify);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Serialize,Deserialize,Debug)]
enum GpioCmd {
    In,
    Out {
        value: bool,
    },
}

type GpioPath = (String, u32);
type GpioModifyResult = Result<(), gpio_cdev::errors::Error>;

fn gpio_modify(chip: String, pin: u32,
               pins: Arc<RwLock<BTreeMap<GpioPath, Chip>>>,
               body: GpioCmd)
    -> GpioModifyResult
{
    // Lock the global map of pins so we can have exclusive access
    // to the mut methods on it and its Chips.
    let mut shared_pins = pins.write().unwrap();
    let mut our_pin_entry = shared_pins.entry((chip.clone(), pin));
    let chipdev = match our_pin_entry {
        Occupied(ref mut entry) => entry.get_mut(),
        Vacant(entry) => entry.insert(Chip::new(format!("/dev/{}", chip))?)
    };

    let line = chipdev.get_line(pin)?;
    match body {
        GpioCmd::Out { value } => {
            line.request(LineRequestFlags::OUTPUT, 0, "http-gpio")?
                .set_value(value as u8)
        }
        GpioCmd::In => {
            line.request(LineRequestFlags::INPUT, 0, "http-gpio")?;
            Ok(())
        }
    }
}

fn as_reply(value: GpioModifyResult) -> Box<dyn warp::Reply> {
    // Return if success, or stringify the error if not
    match value {
        Ok(_) => Box::new("Success"),
        Err(err) => Box::new(
            warp::reply::with_status(err.to_string(),
                                     StatusCode::INTERNAL_SERVER_ERROR))
    }
}
