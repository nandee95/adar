use crate::{
    state_machine::{EndState, HasEndState},
    utils::MaybeSend,
};
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use async_trait::async_trait;
use core::marker::PhantomData;

pub trait StateTypesAsync<P1 = (), P2 = (), P3 = (), P4 = (), P5 = (), P6 = (), P7 = (), P8 = ()>
where
    Self: MaybeSend,
{
    type States;
    type Context;
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
pub trait StateAsync<P1 = (), P2 = (), P3 = (), P4 = (), P5 = (), P6 = (), P7 = (), P8 = ()>
where
    Self: StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8>,
{
    #[allow(unused_variables)]
    #[inline(always)]
    async fn on_enter(&mut self, context: &mut Self::Context) {}

    #[allow(unused_variables)]
    #[inline(always)]
    async fn on_update(&mut self, context: &mut Self::Context) -> Option<Self::States> {
        None
    }

    #[allow(unused_variables)]
    #[inline(always)]
    async fn on_leave(&mut self, context: &mut Self::Context) {}
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
pub trait MachineAsync<P1 = (), P2 = (), P3 = (), P4 = (), P5 = (), P6 = (), P7 = (), P8 = ()>
where
    Self: StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8>,
{
    #[allow(unused_variables)]
    #[inline(always)]
    async fn on_transition(&mut self, new_state: &Self::States, context: &mut Self::Context) {}

    #[allow(unused_variables)]
    #[inline(always)]
    async fn on_update(&mut self, context: &mut Self::Context) {}
}

pub struct StateMachineAsync<
    S,
    P1 = (),
    P2 = (),
    P3 = (),
    P4 = (),
    P5 = (),
    P6 = (),
    P7 = (),
    P8 = (),
> where
    S: StateAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + MachineAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S>,
{
    state: S::States,
    context: S::Context,
    phantom: PhantomData<(P1, P2, P3, P4, P5, P6, P7, P8)>,
}

impl<S, P1, P2, P3, P4, P5, P6, P7, P8> StateMachineAsync<S, P1, P2, P3, P4, P5, P6, P7, P8>
where
    S: StateAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + MachineAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S>,
{
    pub async fn new_context<S2>(
        state: S2,
        mut context: S::Context,
    ) -> StateMachineAsync<S2::States, P1, P2, P3, P4, P5, P6, P7, P8>
    where
        S2: StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S> + Into<S::States>,
    {
        let mut state = state.into() as S::States;
        state.on_enter(&mut context).await;
        StateMachineAsync::<S2::States, P1, P2, P3, P4, P5, P6, P7, P8> {
            state,
            context,
            phantom: PhantomData,
        }
    }

    pub async fn new<S2>(state: S2) -> Self
    where
        S2: StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S> + Into<S::States>,
        S::Context: Default,
    {
        Self::new_context(state, S::Context::default()).await
    }

    pub async fn run(&mut self) {
        while let Some(new_state) = StateAsync::on_update(&mut self.state, &mut self.context).await
        {
            self.transition(new_state).await;
        }
    }

    pub async fn update(&mut self) {
        if let Some(new_state) = StateAsync::on_update(&mut self.state, &mut self.context).await {
            self.transition(new_state).await;
        }
    }

    pub async fn transition(&mut self, new_state: impl Into<S>) {
        self.state.on_leave(&mut self.context).await;
        let new_state = new_state.into();
        self.state
            .on_transition(&new_state, &mut self.context)
            .await;
        self.state = new_state;
        self.state.on_enter(&mut self.context).await;
    }

    pub fn context(&self) -> &S::Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut S::Context {
        &mut self.context
    }

    pub fn state(&self) -> &S::States {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut S::States {
        &mut self.state
    }

    pub async fn end(mut self) {
        self.state.on_leave(&mut self.context).await
    }
}

impl<S, P1, P2, P3, P4, P5, P6, P7, P8> HasEndState
    for StateMachineAsync<S, P1, P2, P3, P4, P5, P6, P7, P8>
where
    S: StateAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + MachineAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S>
        + HasEndState,
{
    fn is_finished(&self) -> bool {
        self.state.is_finished()
    }
}

impl<S, P1, P2, P3, P4, P5, P6, P7, P8> core::fmt::Debug
    for StateMachineAsync<S, P1, P2, P3, P4, P5, P6, P7, P8>
where
    S: StateAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + MachineAsync<P1, P2, P3, P4, P5, P6, P7, P8>
        + StateTypesAsync<P1, P2, P3, P4, P5, P6, P7, P8, States = S>,
    S::States: core::fmt::Debug,
    S::Context: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StateMachine")
            .field("state", &self.state)
            .field("context", &self.context)
            .finish()
    }
}

impl StateTypesAsync for EndState {
    type States = ();
    type Context = ();
}

impl StateAsync for EndState {}

#[cfg(test)]
mod test {
    use crate::{self as adar, prelude::*, utils::MaybeSend};
    use async_trait::async_trait;
    use once_cell::sync::Lazy;
    use serial_test::serial;
    use std::sync::Arc;
    use tokio::{sync::Mutex, test};

    #[derive(Eq, PartialEq, Debug)]
    enum MockState {
        A,
        B,
        C,
    }

    type MockContext = u32;
    #[derive(Eq, PartialEq, Debug)]
    enum MockCall {
        OnEnter(MockContext),
        OnUpdate(MockContext),
        OnLeave(MockContext),
    }

    #[derive(Default, Clone)]
    struct Mock(Arc<Mutex<MockInner>>);

    static MOCK: Lazy<Mock> = Lazy::new(Mock::default);

    #[derive(Default)]
    struct MockInner {
        calls: Vec<(MockState, MockCall)>,
        b_transition: Option<Test>,
    }

    impl Mock {
        pub async fn push(&self, state: MockState, call: MockCall) {
            self.0.lock().await.calls.push((state, call));
        }

        pub async fn take(&self) -> Vec<(MockState, MockCall)> {
            core::mem::take(&mut self.0.lock().await.calls)
        }

        pub async fn b_transition(&self, state: Test) {
            self.0.lock().await.b_transition = Some(state);
        }
    }

    #[StateEnumAsync(context=MockContext)]
    enum Test {
        A,
        B,
        C,
    }

    impl MachineAsync for Test {}

    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl StateAsync for A {
        async fn on_enter(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::A, MockCall::OnEnter(*context)).await;
        }

        async fn on_update(&mut self, context: &mut Self::Context) -> Option<Self::States> {
            MOCK.push(MockState::A, MockCall::OnUpdate(*context)).await;
            None
        }

        async fn on_leave(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::A, MockCall::OnLeave(*context)).await;
        }
    }

    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl StateAsync for B {
        async fn on_enter(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::B, MockCall::OnEnter(*context)).await;
        }

        async fn on_update(&mut self, context: &mut Self::Context) -> Option<Self::States> {
            MOCK.push(MockState::B, MockCall::OnUpdate(*context)).await;
            MOCK.0.lock().await.b_transition.take()
        }

        async fn on_leave(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::B, MockCall::OnLeave(*context)).await;
        }
    }

    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl StateAsync for C {
        async fn on_enter(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::C, MockCall::OnEnter(*context)).await;
        }

        async fn on_update(&mut self, context: &mut Self::Context) -> Option<Self::States> {
            MOCK.push(MockState::C, MockCall::OnUpdate(*context)).await;
            None
        }

        async fn on_leave(&mut self, context: &mut Self::Context) {
            MOCK.push(MockState::C, MockCall::OnLeave(*context)).await;
        }
    }

    #[StateEnumAsync]
    #[derive(Debug)]
    enum TestDerive {
        A2,
    }
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl MachineAsync for TestDerive {}
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl StateAsync for A2 {}

    #[StateEnumAsync(context = Arc<Mutex<MockInner>>)]
    enum TestWithComplexContext {
        A3,
    }
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl MachineAsync for TestWithComplexContext {}
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl StateAsync for A3 {}

    #[StateEnumAsync(context = for<T> Option<T> where T: std::fmt::Debug + MaybeSend)]
    enum TestWithGenericWithContext {
        A4,
    }
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl MachineAsync for TestWithGenericWithContext {}
    #[cfg_attr(feature="async-st", async_trait(?Send))]
    #[cfg_attr(not(feature = "async-st"), async_trait)]
    impl<T> StateAsync<T> for A4 where T: std::fmt::Debug + MaybeSend {}

    #[cfg(feature = "async-st")]
    #[StateEnumAsync]
    enum TestSingleThreaded {
        A5(std::marker::PhantomData<*const ()>),
    }
    #[cfg(feature = "async-st")]
    #[async_trait(?Send)]
    impl MachineAsync for TestSingleThreaded {}
    #[cfg(feature = "async-st")]
    #[async_trait(?Send)]
    impl StateAsync for A5 {}

    #[cfg_attr(feature = "async-st", test(flavor = "current_thread"))]
    #[cfg_attr(not(feature = "async-st"), test(flavor = "multi_thread"))]
    async fn test_macro_edge_cases() {
        // Note: Just to make sure they can be constructed
        let sm = StateMachineAsync::new(A2).await;
        println!("{:?}", sm);
        StateMachineAsync::new_context(A3, Arc::new(Mutex::new(MockInner::default()))).await;
        StateMachineAsync::new_context(A4, Some(())).await;
        #[cfg(feature = "async-st")]
        StateMachineAsync::new(A5(std::marker::PhantomData)).await;
    }

    #[cfg_attr(feature = "async-st", test(flavor = "current_thread"))]
    #[cfg_attr(not(feature = "async-st"), test(flavor = "multi_thread"))]
    #[serial]
    async fn test_external_transition_and_update() {
        let mut sm = StateMachineAsync::new_context(A, 0).await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::A, MockCall::OnEnter(0))]
        );
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::A, MockCall::OnUpdate(0))]
        );
        sm.transition(B).await;
        assert_eq!(
            MOCK.take().await,
            vec![
                (MockState::A, MockCall::OnLeave(0)),
                (MockState::B, MockCall::OnEnter(0))
            ]
        );
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::B, MockCall::OnUpdate(0))]
        );
        sm.transition(C).await;
        assert_eq!(
            MOCK.take().await,
            vec![
                (MockState::B, MockCall::OnLeave(0)),
                (MockState::C, MockCall::OnEnter(0))
            ]
        );
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::C, MockCall::OnUpdate(0))]
        );
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::C, MockCall::OnUpdate(0))]
        );
        sm.end().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::C, MockCall::OnLeave(0))]
        );
    }

    #[cfg_attr(feature = "async-st", test(flavor = "current_thread"))]
    #[cfg_attr(not(feature = "async-st"), test(flavor = "multi_thread"))]
    #[serial]
    async fn test_internal_transition_and_update() {
        let mut sm = StateMachineAsync::new_context(B, 0).await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::B, MockCall::OnEnter(0))]
        );
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::B, MockCall::OnUpdate(0))]
        );
        MOCK.b_transition(C.into()).await;
        sm.update().await;
        assert_eq!(
            MOCK.take().await,
            vec![
                (MockState::B, MockCall::OnUpdate(0)),
                (MockState::B, MockCall::OnLeave(0)),
                (MockState::C, MockCall::OnEnter(0))
            ]
        );
        sm.end().await;
        assert_eq!(
            MOCK.take().await,
            vec![(MockState::C, MockCall::OnLeave(0))]
        );
    }
}
