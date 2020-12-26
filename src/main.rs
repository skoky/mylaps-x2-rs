use std::{thread, time};
use std::ffi::{c_void, CStr, CString};
use std::process::exit;
use std::ptr::null;
use std::sync::mpsc;

use clap::{App, Arg};

use crate::mylapsx2::{availableappliance_t, CONNECTIONSTATE__csAuthenticationFailed, MDP_NOTIFY_TYPE, mdp_sdk_alloc, mdp_sdk_appliance_verify, mdp_sdk_handle_t, mdp_sdk_messagequeue_process, mdp_sdk_notify_verify_appliance, mta_connect, mta_handle_alloc, mta_handle_t, mta_notify_connect, mta_notify_connectionstate, mta_notify_systemsetup, systemsetup_t};

mod mylapsx2;

static mut stop: bool = false;

fn main() {
    let mut context = 0 as *mut c_void;
    let timeout = time::Duration::from_secs(10);
    let app_name = CString::new("mylpaps-x2-rs").unwrap();

    let matches = App::new(app_name.to_str().unwrap())
        .version("1.0.0")
        .author("skokys@gmail.com")
        .arg(Arg::with_name("hostname")
            .index(1)
            .required(true)
            .help("MyLaps X2 server hostname or ip address")
        )
        .get_matches();

    let hostname_param = match matches.value_of("hostname") {
        Some(h) if h.len() > 0 => h,
        Some(_) | None => panic!("MyLaps X2 server name missing"),
    };

    let hostname = CString::new(hostname_param).unwrap();

    unsafe {
        let sdk_handle = mdp_sdk_alloc(app_name.as_ptr(), context);

        mdp_sdk_notify_verify_appliance(sdk_handle, Some(notify_verify));

        match mdp_sdk_appliance_verify(sdk_handle, hostname.as_ptr()) {
            false => panic!("Can't create SDK handle"),
            true => {}
        }
        println!("Connecting {}...", hostname_param);
        let now = time::Instant::now();
        while !stop {
            mdp_sdk_messagequeue_process(sdk_handle, true, 1_000);
            if now.elapsed() >= timeout {
                println!("Timeout waiting for verify");
                exit(1);
            }
        }
    }

    unsafe extern "C" fn notify_verify(handle: mdp_sdk_handle_t,
                                       hostname: *const ::std::os::raw::c_char,
                                       is_verified: bool,
                                       appliance: *const availableappliance_t,
                                       context: *mut ::std::os::raw::c_void, ) {
        let h = CStr::from_ptr(hostname);
        println!("Verification result {} -> {}", h.to_str().unwrap(), is_verified);
        if is_verified {
            let appl = (*appliance);
            println!("Appliance build {}", appl.buildnumber);
        }
        stop = true;
    }
}
