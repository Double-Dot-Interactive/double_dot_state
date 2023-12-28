# Double_Dot_State is a usefull crate for maintaining state management in rust
It is primarily used for state-management in Bevy

## What Double_Dot_State accomplishes
State management is easy thanks to the `DoubleState` macro.

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

The `DoubleState` derive macro will take each attribute placed on each state and check if they are valid enum fields. So if OptionMenu is defined as a linear transition from MainMenu you'd get a compiler error telling you `OptionMenu doesn't exist as a state in AppState`. This gives the user some
peace of mind defining their states.

Only 1 linear transition is allowed per state along with an unlimited amount of arbitrary transitions.

## Usage
After defining your state enum, add it to Bevy via the [`add_state()`](https://docs.rs/bevy/0.12.1/bevy/app/struct.App.html#method.add_state) method from [`App`](https://docs.rs/bevy/0.12.1/bevy/app/struct.App.html)

```rust
use bevy::prelude::*;
fn main() {
    App::new()
        // your implementation here
        .add_state::<AppState>()
    ;
}
```
### Linear Transitions

After your AppState is in Bevy's system you could for example trantition to the MainMenu state after finished loading assets.
```rust
fn load_assets(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut state: ResMut<NextState<AppState>>
) {
    // load your assets

    // transition into the next linear state defined in the enum type
    state.set(state.0.unwrap().linear_transition());
}
```

[`linear_transition()`]() will return the linear transition defined as an attribute on the current state. It will **panic** if no linear transitions exist for the current state.

### Arbitrary Transitions

If you have a state that can transition to multiple arbitrary ones you can transition to whichever state you want.

So if you want to transition to MainMenu from Paused you can do so like this
```rust
fn pause_menu(
    mut state: ResMut<NextState<AppState>>
) {
    // your pause menu implementation here
    
    // if the user clicks Main Menu transition to MainMenu state
    state.set(state.0.unwrap().arbitrary_transition(AppState::MainMenu));
}
```

[`arbitray_transition()`]() will return the state specified if it is deemed as a valid transition via the attributes defined on the current state.

