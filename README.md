
This is a **sample code** showing how to use [MyLaps X2 SDK](https://help.mylaps.com/s/article/Where-can-I-find-the-X2-SDK?language=en_US)
with [Rust language](https://www.rust-lang.org/). The example generates native SDK binder into Rust lang,
connects to a MyLaps X2 server and validates connection with excellent performance.

Features
* generating Rust wrapper from C based SDK in `build.rs`
* command line app verifying hostname X2 sever
* error handling sample from the SDK; wrapping *unsafe* methods call
* macro for converting `*void` from / to SDK
* state management with native API

The [Rust language](https://www.rust-lang.org/) is modern system language suitable for realtime and high-performance
applications *without garbage collector*.

**MYLAPS is not responsible for this code and cannot provide support or warranty on it** If you ned any help, contact author of the repo.

This sample connects to MyLaps X2 server, not decoders. A direct connection to decoder is not supported by MyLaps anymore.
Also the app does not support connection to legacy decoders like AMBrc Decoder, MYLAPS RC4 Decoder, AMBmx3 Decoder,
TranX3 Decoder, ChipX/ProChip Decoder, BIB Decoder, AMBrc, AMB20, AMB130, TranX-2 / TranX2, TranXPro, Activ or PegaSys.
If you need help with those decoders, see https://ammconverter.eu.

# Supported platforms

The project runs inside docker container on Linux, Windows, Mac OSX platforms or other supported platforms. There
is an additional platform dependency on Intel X64.

Note: MyLaps provides X2 SDK binaries compiled for Intel 64bit and 32 it HW platforms only.
Anyway, this example app support the 64 bit platform as this is commonly used today.

# Pre-installation

- Contact [MyLaps](https://www.mylaps.com/x2-timing-data-system/x2-sdk) to download X2 SDK. Copy it to the root directory `sdk-master`, version min 4.1+
- install Rust language that includes *cargo* build tool
- make sure you have X2 appliance available on network, have it hostname or ip address
- install latest Docker and Docker-compose

# Compiling example

Make sure Docker install and run building process inside the docker container (using docker-compose)
See all details inside the `docker-compose.yml`:

    docker-compose run app-build

# Running example app

    docker-compose run x2 <x2hostname>

# Example output on console

    docker-compose run x2 myx2.local
    Creating mylaps-x2-rs_x2_run ... done
    Connecting myx2.local...
    Verification result myx2.local -> true
    Appliance build 67215430

# MyLaps support

Thank you MyLaps for supporting this project!

# Future work TBD

- covering more events from X2 appliance, like manual event, resend example, time sync etc
- unit testing, if multiplatform plugin will support it
