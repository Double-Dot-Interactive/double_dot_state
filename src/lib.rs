pub use double_dot_macro::*;
pub use bevy::prelude::*;
use prelude::{DoubleStateEvent, DoubleStateTransition};

pub mod prelude {
    pub use double_dot_macro::DoubleStates;
    pub use bevy::prelude::*;

    use crate::watch_state_event;
    
    #[derive(Debug, Clone)]
    pub enum DoubleStateTransition<S>
    where
        S: DoubleStates
    {
        Linear,
        Arbitrary(S)
    }

    pub enum DoubleStateSchedule<S> 
    where
        S: DoubleStates + States
    {
        OnEnter(OnEnter<S>),
        OnExit(OnExit<S>),
    }

    pub struct DoubleStateSystemConfig<S> 
    where
        S: DoubleStates + States
    {
        pub schedule: DoubleStateSchedule<S>,
        pub transition: DoubleStateTransition<S>
    }
    
    #[derive(Debug, Clone, Event)]
    pub struct DoubleStateEvent<S: DoubleStates>(pub DoubleStateTransition<S>);
    pub trait AppExt {
        fn add_double_state<S: DoubleStates + States>(&mut self) -> &mut Self;
    }
    
    impl AppExt for App {
        fn add_double_state<S: DoubleStates + States>(&mut self) -> &mut Self {
            self
                .init_state::<S>()
                .add_event::<DoubleStateEvent<S>>()
                .add_systems(Update,watch_state_event::<S>)
        }
    }
}


fn watch_state_event<S: DoubleStates + States>(
    mut state_event: EventReader<DoubleStateEvent<S>>,
    mut next_state: ResMut<NextState<S>>,
    state: Res<State<S>>
) {
    for event in state_event.read() {
        match &event.0 {
            DoubleStateTransition::Linear => {
                // info!("{:?}", state.0.linear_transition());
                next_state.set(state.linear_transition())
            },
            DoubleStateTransition::Arbitrary(arb_state) => {
                // info!("{:?}", state.0.arbitrary_transition(&arb_state));
                next_state.set(state.arbitrary_transition(&arb_state))
            },
        }
    }
}

/// Enum for Testing purposes
#[derive(Clone, Eq, PartialEq, Debug, Hash, DoubleStates, Default)]
enum TestState {
    #[linear(MainMenu)]
    Loading,
    #[arbitrary(Playing, Exit)]
    MainMenu,
    #[default]
    #[linear(Paused)]
    Playing,
    #[arbitrary(MainMenu, Exit)]
    Paused,
    Exit
}
