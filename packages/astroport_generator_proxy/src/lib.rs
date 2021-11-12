pub mod anc_staking;
pub mod generator;
pub mod generator_proxy;
pub mod mars_staking;

#[allow(clippy::all)]
mod uints {
    use uint::construct_uint;
    construct_uint! {
        pub struct U256(4);
    }
}

pub use uints::U256;
