
#export LD_DEBUG=all
export LD_LIBRARY_PATH=/opt/prj/sdk-master/lib/linux/x86-64
export LIBRARY_PATH=/opt/prj/sdk-master/lib/linux/x86-64
export PATH=$HOME/.cargo/bin:$PATH
rustc --version

export RUSTFLAGS=-Awarnings
cargo build
