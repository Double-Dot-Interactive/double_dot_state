pub use double_dot_macro::*;
pub use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum DoubleStateTransition<S>
where
    S: DoubleStates
{
    Linear,
    Arbitrary(S)
}

#[derive(Debug, Clone)]
pub struct DoubleStateEvent<S: DoubleStates>(pub DoubleStateTransition<S>);


fn watch_state_event<S: DoubleStates + States>(
    mut state_event: EventReader<DoubleStateEvent<S>>,
    mut next_state: ResMut<NextState<S>>,
    state: Res<State<S>>
) {
    for event in state_event.iter() {
        match &event.0 {
            DoubleStateTransition::Linear => {
                // info!("{:?}", state.0.linear_transition());
                next_state.set(state.0.linear_transition())
            },
            DoubleStateTransition::Arbitrary(arb_state) => {
                // info!("{:?}", state.0.arbitrary_transition(&arb_state));
                next_state.set(state.0.arbitrary_transition(&arb_state))
            },
        }
    }
}

pub trait AppExt {
    fn add_double_state<S: DoubleStates + States>(&mut self) -> &mut Self;
}

impl AppExt for App {
    fn add_double_state<S: DoubleStates + States>(&mut self) -> &mut Self {
        self
            .add_state::<S>()
            .add_event::<DoubleStateEvent<S>>()
            .add_system(watch_state_event::<S>)
    }
}