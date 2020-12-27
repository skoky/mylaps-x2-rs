use std::{thread, time};
use std::ffi::{c_void, CStr, CString};
use std::process::exit;
use std::ptr::null;
use std::sync::mpsc;
use std::time::Duration;

use clap::{App, Arg};

use crate::mylapsx2::availableappliance_t;
use crate::mylapsx2::mdp_sdk_alloc;
use crate::mylapsx2::mdp_sdk_appliance_verify;
use crate::mylapsx2::mdp_sdk_handle_dummystruct;
use crate::mylapsx2::mdp_sdk_handle_t;
use crate::mylapsx2::mdp_sdk_messagequeue_process;
use crate::mylapsx2::mdp_sdk_notify_verify_appliance;

mod mylapsx2;

struct State {
    should_stop: bool
}

const APP_NAME: &str = "mylaps-x2-rs";
const VERSION: &str = "1.0.0";
const AUTHOR: &str = "skokys@gmail.com";

const TIMEOUT: Duration = time::Duration::from_secs(10);
const HOSTNAME_PARAM: &str = "hostname";

#[cfg(debug_assertions)]
macro_rules! toVoid { ($e:expr) => (&mut $e as *mut _ as *mut c_void)  }

#[cfg(debug_assertions)]
macro_rules! fromVoid { ($e:expr) => {unsafe { &mut *($e as *mut State) } } }

fn main() {
    let mut state = State { should_stop: false };
    let app_name = CString::new(APP_NAME).unwrap();

    let matches = App::new(APP_NAME)
        .version(VERSION)
        .author(AUTHOR)
        .arg(Arg::with_name(HOSTNAME_PARAM)
            .index(1)
            .required(true)
            .help("MyLaps X2 server hostname or ip address")
        )
        .get_matches();

    let hostname_param = match matches.value_of(HOSTNAME_PARAM) {
        Some(h) if h.len() > 0 => h,
        Some(_) | None => panic!("MyLaps X2 server name missing"),
    };

    let sdk_handle = sdk_handle_safe(toVoid!(state), app_name);

    setup_notifier(sdk_handle);

    verify_appliance(sdk_handle, hostname_param);

    println!("Connecting {}...", hostname_param);
    let now = time::Instant::now();
    while !state.should_stop {
        wait_for_message(sdk_handle);

        if now.elapsed() >= TIMEOUT {
            println!("Timeout waiting for verify");
            exit(1);
        }
    }
}

unsafe extern "C" fn notify_verify(_handle: mdp_sdk_handle_t,
                                   hostname: *const ::std::os::raw::c_char,
                                   is_verified: bool,
                                   appliance: *const availableappliance_t,
                                   context: *mut ::std::os::raw::c_void, ) {
    assert!(!hostname.is_null(), "Hostname is null in notify_verify handler");

    let h = CStr::from_ptr(hostname);
    match h.to_str() {
        Ok(hostname_str) => println!("Verification result {} -> {}", hostname_str, is_verified),
        Err(e) => panic!("Unable to unwrap hostname")
    }

    if is_verified && !appliance.is_null() {
        let appl = (*appliance);
        println!("Appliance build {}", appl.buildnumber);
    }

    assert!(!context.is_null(), "Context is null in notify_verify handler");
    let data = fromVoid!(context);
    data.should_stop = true;
}

fn wait_for_message(sdk_handle: mdp_sdk_handle_t) {
    unsafe { mdp_sdk_messagequeue_process(sdk_handle, true, 1_000); }
}

fn setup_notifier(sdk_handle: mdp_sdk_handle_t) {
    unsafe { mdp_sdk_notify_verify_appliance(sdk_handle, Some(notify_verify)); }
}

fn verify_appliance(sdk_handle: mdp_sdk_handle_t, hostname_param: &str) {
    match CString::new(hostname_param) {
        Ok(hostname) => {
            match unsafe { mdp_sdk_appliance_verify(sdk_handle, hostname.as_ptr()) } {
                false => panic!("verify appliance failed"),
                true => {}
            }
        }
        Err(e) => panic!("Unable to convert hostname {}", e)
    }
}

fn sdk_handle_safe(context: *mut c_void, app_name: CString) -> *mut mdp_sdk_handle_dummystruct {
    let sdk_handle = unsafe { mdp_sdk_alloc(app_name.as_ptr(), context) };

    assert!(!sdk_handle.is_null(), "Unable to get sdk handle");
    return sdk_handle;
}
