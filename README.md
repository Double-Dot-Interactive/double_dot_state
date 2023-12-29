# Double_Dot_State is a usefull crate for maintaining state management in rust

It is primarily used for state-management in Bevy but will soon include a feature to make it work for any rust project, ie. won't implement any Bevy features and just check if state transitions are valid.

## What Double_Dot_State accomplishes

Properly managing State can be a cumbersome task. Take this state diagram as an example

![alt text](https://github.com/Double-Dot-Interactive/double_dot_state/blob/main/doc/example%20diagram.png?raw=true)

In this example there is a mix of linear and arbitrary state transitions.

Managing this state is made easier thanks to the `DoubleState` macro.

`DoubleState` will implement `States` from Bevy automatically.

All you need to do is define linear or arbitrary state transitions for your state enum.

```rust
#[derive(Debug, Clone, Default, DoubleState)]
pub enum AppState {
    #[default]
    #[linear(MainMenu)]
    Loading,
    #[linear(Playing)]
    #[arbitrary(Exit)]
    MainMenu,
    #[linear(Paused)]
    Playing,
    #[linear(Playing)]
    #[arbitrary(MainMenu, Exit)]
    Paused,
    Exit
} 
```

***note*** you'll get a compiler error if no default state is defined via the [`Default`](https://doc.rust-lang.org/std/default/trait.Default.html) derive macro.

The `DoubleState` derive macro will take each attribute placed on each state and check if they are valid enum fields. So if OptionMenu (which doesn't exist) is defined as a linear transition from MainMenu you'd get a compiler error telling you `"OptionMenu" doesn't exist as a state in "AppState"`. Knowing this cuts down on runtime errors.

Only 1 linear transition is allowed per state, however you can define an unlimited amount of arbitrary transitions.

## Usage

After defining your state enum, add it to Bevy via the [`add_double_state()`](https://docs.rs/double_dot_state/latest/double_dot_state/trait.AppExt.html#tymethod.add_double_state) method from [`AppExt`](https://docs.rs/double_dot_state/latest/double_dot_state/trait.AppExt.html) which is added onto [`App`](https://docs.rs/bevy/latest/bevy/app/struct.App.html) from double_dot_state

```rust
use bevy::prelude::*;
fn main() {
    App::new()
        // your implementation here
        .add_double_state::<AppState>()
    ;
}
```

### Linear Transitions

After your AppState is in the system you could for example trantition to the MainMenu state after finished loading assets.

```rust
fn load_assets(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: EventWriter<DoubleStateEvent<AppState>>
) {
    // load your assets

    // transition into the next linear state defined in the enum type
    next_state.send(DoubleStateEvent(DoubleStateTransition::Linear));
}
```

[`DoubleStateTransition::Linear`](https://docs.rs/double_dot_state/latest/double_dot_state/enum.DoubleStateTransition.html) tells double_dot_state the you want to attempt a linear state transition from the current state.

***note*** This will panic with the message `No linear transition found for "AppState::Loading"` if no linear transition is defined for Loading which is the current AppState in this example. Panicing here is important, as a bug like this will render your program useless anyways. Knowing this can help with debugging your code.

### Arbitrary Transitions

If you have a state that can transition to multiple arbitrary ones you can transition to whichever state you want.

So if you want to transition to MainMenu from Paused you can do so like this

```rust
fn pause_menu(
    mut state: EventWriter<DoubleStateEvent<AppState>>
) {
    // your pause menu implementation here
    
    // if the user clicks Main Menu transition to MainMenu state
    next_state.send(DoubleStateEvent(DoubleStateTransition::Arbitrary(AppState::MainMenu)));
}
```

[`DoubleStateTransition::Arbitrary`](https://docs.rs/double_dot_state/latest/double_dot_state/enum.DoubleStateTransition.html) tells double_dot_state that you want to attempt an arbitrary transition from the current AppState to the new AppState.

***note*** This will panic with the message `Arbitrary transition "MainMenu" not found for "AppState::Paused"` if the specified new transition doesn't exist for Paused which is the current AppState in this example. Panicing here is important, as a bug like this will render your program useless anyways. Knowing this can help with debugging your code.

License
Dual-licensed under either of

Apache License, Version 2.0, (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
MIT license (LICENSE-MIT or <https://opensource.org/licenses/MIT>)
at your option.

Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted via [GitHub Repo](https://github.com/Double-Dot-Interactive/double_dot_state) for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
