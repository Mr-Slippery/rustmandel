// Discrete Dynamical System
pub trait DDS<State> {
    fn cont(&self, z: State) -> bool;
    fn next(&self, z: State, c: State) -> State;
}

// Iterated Function System
pub trait IFS<State, Output> {
    fn iter(&self, start: State, param: State) -> Output;
}
