/// Mermaid diagram: https://mermaid.live/edit#pako:eNp1kVFPgzAUhf8Kub4ZtkiBMfpgYpxZlixithmjYpYKF0ZW6FKKOpf9d8sYTjHep3tvv3NOm-4gEjEChVIxhaOMpZLlvTcSFoauiLOyHGFiSIyNJOOcnjFCzEhwIen7KlPY4VKJWBxJwlrylbNo3SG3yLl4b027aAM_n78Yvd6lMVdi02zqjlJaX6c-GKOaIYu3BjXGwXK-CO6Wo_vZ1WIS3DZ8C2jNMfAgE1rweDOdBg9dXmiyeUQbPEnmLMF_E07I74x6_zcFTEhlFgNVskITcpQ5q0fY1W4hqBXmGALVbczkOoSw2GvNhhVPQuStTIoqXQFNGC_1VG3i09d9byUWMcprURUKqDvwDyZAd_AB1PbtvmUPLddziUM81zJhqyGr79ue49nEcayB5wzdvQmfh9iL_tBz_J-1_wJ7ja8f
use adar::{async_trait, prelude::*};
use std::{process::Command, time::Duration};
use tokio::{main, time::sleep};

#[StateEnumAsync]
#[ReflectEnum] // Optional. (Used here to print the name of the state)
enum TrafficLight {
    Go,
    GetReady,
    StopIfSafe,
    Stop,
}

impl TrafficLight {
    const YELLOW_DURATION: Duration = Duration::from_secs(1);
    const GO_STOP_DURATION: Duration = Duration::from_secs(2);
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
impl MachineAsync for TrafficLight {
    async fn on_transition(&mut self, new_state: &Self::States, _context: &mut Self::Context) {
        Command::new("clear")
            .status()
            .expect("Failed to clear the screen!");

        println!("{}", new_state.name());
    }
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
impl StateAsync for Go {
    async fn on_enter(&mut self, _context: &mut Self::Context) {
        println!("⚫\n⚫\n🟢");
    }
    async fn on_update(&mut self, _context: &mut Self::Context) -> Option<Self::States> {
        std::thread::sleep(TrafficLight::GO_STOP_DURATION);
        Some(StopIfSafe.into())
    }
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
impl StateAsync for GetReady {
    async fn on_enter(&mut self, _context: &mut Self::Context) {
        println!("🔴\n🟡\n⚫");
    }
    async fn on_update(&mut self, _context: &mut Self::Context) -> Option<Self::States> {
        sleep(TrafficLight::YELLOW_DURATION).await;
        Some(Go.into())
    }
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
impl StateAsync for StopIfSafe {
    async fn on_enter(&mut self, _context: &mut Self::Context) {
        println!("⚫\n🟡\n⚫");
    }
    async fn on_update(&mut self, _context: &mut Self::Context) -> Option<Self::States> {
        sleep(TrafficLight::YELLOW_DURATION).await;
        Some(Stop.into())
    }
}

#[cfg_attr(feature="async-st", async_trait(?Send))]
#[cfg_attr(not(feature = "async-st"), async_trait)]
impl StateAsync for Stop {
    async fn on_enter(&mut self, _context: &mut Self::Context) {
        println!("🔴\n⚫\n⚫")
    }
    async fn on_update(&mut self, _context: &mut Self::Context) -> Option<Self::States> {
        sleep(TrafficLight::GO_STOP_DURATION).await;
        Some(GetReady.into())
    }
}

#[cfg_attr(feature = "async-st", main(flavor = "current_thread"))]
#[cfg_attr(not(feature = "async-st"), main(flavor = "multi_thread"))]
async fn main() {
    StateMachineAsync::new(Stop).await.run().await;
}
