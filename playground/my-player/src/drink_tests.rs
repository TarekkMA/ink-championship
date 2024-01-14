//! The last paradigm can be called: 'quasi-E2E` testing. It is based on the drink! library
//! (https://github.com/Cardinal-Cryptography/drink/). It is a peculiar trade-off between unit
//! and E2E testing.
//!
//! The idea is that we do not have a running node at all. We keep a wrapped runtime in the memory
//! and interact with it directly. While it is not the perfectly realistic environment, it is still
//! a very good approximation of the real world. Thanks to the architecture, everything is kept
//! synchronous and instant. Also, we have more power over the chain itself, and thus we can easily
//! manipulate the whole state, for example block building.
//!
//! Since the library is pretty fresh, sometimes it comes with onerous API or some not obvious
//! behavior. Please be patient and do not hesitate to raise an issue or ask for help :)
//!
//! For a quick-start tutorial with drink, consult: https://github.com/inkdevhub/drink/tree/main/examples/quick-start-with-drink.

use drink::{
    runtime::MinimalRuntime,
    session::{Session, NO_ARGS, NO_ENDOWMENT, NO_SALT},
};
use squink_splash::State;

use crate::drink_tests::game_parameters::*;

/// Just a type alias for the result type of quasi-E2E testcases.
type TestResult<T> = Result<T, Box<dyn std::error::Error>>;

/// We gather all game parameters in this module, so that we can easily change and access them.
mod game_parameters {
    pub const DIMENSION: u32 = 4;
    pub const START: u32 = 1;
    pub const FORMING_ROUNDS: u32 = 0;
    pub const ROUNDS: u32 = 10;
    pub const BUY_IN: u128 = 0;
}

/// We declare a contract bundle provider. It will take care of building all contract dependencies in the compilation
/// phase and gather all contract bundles into a single registry.
#[drink::contract_bundle_provider]
enum BundleProvider {}

/// As in the unit tests and e2e tests, we can verify, that the contract instantiation works well.
#[drink::test]
fn instantiation_works() -> TestResult<()> {
    // We create a new session object. While `drink!` exposes also a low-level API (`Sandbox`), it
    // is way more convenient to use the `Session` API. It keeps the context, caller, and other
    // information for us. Also, it allows to work with weak types instead of raw bytes.
    //
    // `drink!` allows to work with an arbitrary runtime. In this case, we use the minimal one,
    // which provides only these pallets that enable working with the ink! contracts.
    let mut session = Session::<MinimalRuntime>::new()?;
    session.deploy_bundle(
        // We pass data about `my_player` contract.
        BundleProvider::MyPlayer.bundle()?,
        // We pass the constructor name.
        "new",
        // We pass argument values. The transcoding object will take care of encoding them.
        &[format!("({DIMENSION},{DIMENSION})"), START.to_string()],
        // We don't need any salt for account generation.
        NO_SALT,
        // We don't need any endowment.
        NO_ENDOWMENT,
    )?;
    Ok(())
}

/// As always, we can write a test verifying the contract behavior. In this case, we want to verify
/// that the contract returns the correct coordinates.
#[drink::test]
fn uses_dummy_strategy_correctly() -> TestResult<()> {
    let session = Session::<MinimalRuntime>::new()?;
    // Similarly to the e2e tests, we move the familiar boilerplate logic to a helper function.
    let coordinates: Option<(u32, u32)> =
        instantiate_my_player(session).call("my_turn", NO_ARGS, NO_ENDOWMENT)??;
    assert_eq!(coordinates, Some((1, 0)));
    Ok(())
}

/// We can easily test multiple contracts. In this case, we want to verify that the game contract
/// works well with many players.
#[drink::test]
fn we_can_simulate_game_with_many_players() -> TestResult<()> {
    // Prepare contract constructor arguments.
    let dim_arg = format!("({DIMENSION},{DIMENSION})");
    let my_player_args = [dim_arg.clone(), START.to_string()];
    let game_args = [
        format!("{{x:{DIMENSION},y:{DIMENSION}}}"),
        BUY_IN.to_string(),
        FORMING_ROUNDS.to_string(),
        ROUNDS.to_string(),
    ];

    // Deploy all contracts. Remember to use appropriate transcoder for every contract.
    let session = Session::<MinimalRuntime>::new()?;

    todo!("Deploy all player contracts and the game contract. Use `BundleProvider` to get the contract bundles.");
    todo!("Register players");
    todo!("Play the game");
    todo!("Check the game state after it has finished");

    Ok(())
}
