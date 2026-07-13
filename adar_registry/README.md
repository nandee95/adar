# Advanced Architecture (ADAR) - Registry

[![Crates.io](https://img.shields.io/crates/v/adar_registry.svg)](https://crates.io/crates/adar_registry)
[![Downloads](https://img.shields.io/crates/d/adar_registry.svg)](https://crates.io/crates/adar_registry)
[![Docs](https://docs.rs/adar_registry/badge.svg)](https://docs.rs/adar_registry/latest/adar_registry/)

Adar registry is a collection of containers for managing resources in dynamically loaded extensions/plugins.

> Disclaimer: This crate uses some `unsafe` code. Please refer to the `SAFETY` comments in the source code for details.

## [Registry](`prelude::Registry`)

[Registry](`prelude::Registry`) is a container that lets you control the lifetime of elements through an [Entry](`prelude::Entry`) struct returned after calling [Registry::register()](`prelude::Registry::register`). [Entry](`prelude::Entry`) cannot be cloned, but it can be cast to a generic type using [Entry::as_generic()](`prelude::Entry::as_generic`), which makes it possible to store entries from multiple registries in a single container. [Registry](`prelude::Registry`) can be cloned and behaves like an [Arc](`std::sync::Arc`). Whenever the data is mutated, an internal [RwLock](`std::sync::RwLock`) is locked. You can also run code when an element is removed by using the [set_remove_callback()](`prelude::Registry::set_remove_callback`) callback.

### Example

```rust
use adar_registry::prelude::*;

struct MenuItem(pub &'static str);
struct StyleSheet(pub &'static str);

fn main() {
    // Original website setup
    let menu = Registry::<MenuItem>::new();
    let styles = Registry::<StyleSheet>::new();
    let mut website_store = vec![];

    website_store.push(menu.register(MenuItem("Home")).as_generic());
    website_store.push(menu.register(MenuItem("About")).as_generic());
    website_store.push(styles.register(StyleSheet("website.css")).as_generic());

    print_state("Original website", &menu, &styles);

    // Loading an extension which registers new resources
    let mut extension_store = vec![];
    extension_store.push(menu.register(MenuItem("Weather")).as_generic());
    extension_store.push(menu.register(MenuItem("News")).as_generic());
    extension_store.push(styles.register(StyleSheet("extension.css")).as_generic());

    print_state("After extension is loaded", &menu, &styles);

    // Unloading the extension
    drop(extension_store);

    print_state("After extension is unloaded", &menu, &styles);
}

fn print_state(step: &'static str, menu: &Registry<MenuItem>, styles: &Registry<StyleSheet>) {
    println!("{}", step);
    let menu: Vec<_> = menu.read().iter().map(|v| v.1 .0).collect();
    println!(" - Menu: {}", menu.join(", "));
    let styles: Vec<_> = styles.read().iter().map(|v| v.1 .0).collect();
    println!(" - StyleSheets: {}", styles.join(", "));
}
```

<details>
<summary>Click to see the output</summary>
<code><b>>> cargo run --example registry_extension</b></code>

```ignore
Original website
 - Menu: Home, About
 - StyleSheets: website.css
After extension is loaded
 - Menu: Home, About, Weather, News
 - StyleSheets: website.css, extension.css
After extension is unloaded
 - Menu: Home, About
 - StyleSheets: website.css
```

</details>

## [RegistryMap](`prelude::RegistryMap`)

[RegistryMap](`prelude::RegistryMap`) is similar to [Registry](`prelude::Registry`). But you need to identify each element in the registry with a key. The key need to be provided during [register()](`prelude::RegistryMap::register`) and you can later get the elements using [get()](`prelude::RegistryMapReadGuard::get`) or [get()](`prelude::RegistryMapWriteGuard::get`). [RegistryMap](`prelude::RegistryMap`) uses a [BTreeMap](`std::collections::BTreeMap`) internally.

### Example

```rust
use adar_registry::prelude::*;

trait EndPoint {
    fn execute(&self);
}

struct GetUser;

impl EndPoint for GetUser {
    fn execute(&self) {
        println!("Getting user");
    }
}

fn main() {
    let registry = RegistryMap::<&'static str, Box<dyn EndPoint + Send + Sync + 'static>>::new();
    let _entry = registry.register("get_user", Box::new(GetUser));

    registry.read().get(&"get_user").unwrap().execute();
}
```

<details>
<summary>Click to see the output</summary>
<code><b>>> cargo run --example registry_map</b></code>

```ignore
Getting user
```

</details>

## [Event](`prelude::Event`)

[Event](`prelude::Event`) is a lightweight wrapper around [Registry](`prelude::Registry`). It provides an implementation of an event/observer architecture. \
Please note that during event dispatch the [Registry](`prelude::Registry`) remains locked. This means that you cannot add elements to the registry from the callbacks. Also keep your observers lightweight!

### Example

```rust
use adar_registry::prelude::*;

fn main() {
    // Create an event and register two observers.
    let event: Event<(u32, String)> = Event::new();
    let _entry1 = event.register_observer(|data: &(u32, String)| {
        println!("Observer #1 called: {:?}", data);
    });
    let entry2 = event.register_observer(|data: &(u32, String)| {
        println!("Observer #2 called: {:?}", data);
    });

    // Since all entries are still in scope all the observers will be called.
    event.dispatch((1, "First event".into()));

    // After dropping entry2. The associated observer will be dropped.
    drop(entry2);
    event.dispatch((2, "Second event".into()));
}
```

<details>
<summary>Click to see the output</summary>
<code><b>>> cargo run --example registry_extension</b></code>

```ignore
Observer #1 called: (1, "First event")
Observer #2 called: (1, "First event")
Observer #1 called: (2, "Second event")
```

</details>

## [TracedRegistry](`prelude::TracedRegistry`)

[TracedRegistry](`prelude::TracedRegistry`) is an extension of [Registry](`prelude::Registry`). It enables you to register multiple observers that handle registering or unregistering elements.

### Example

```rust
use adar_registry::prelude::*;

fn main() {
    let registry = TracedRegistry::<&'static str>::new();
    let _observer = registry.register_observer(|(event, entry, value): &_| {
        println!("{:?}, {:?}, {}", event, entry, value)
    });

    let foo = registry.register("foo");
    let bar = registry.register("bar");
    drop(foo);
    let baz = registry.register("baz");
    drop(bar);
    drop(baz);
}
```

<details>
<summary>Click to see the output</summary>
<code><b>>> cargo run --example registry_extension</b></code>

```ignore
Register, 0, foo
Register, 1, bar
UnRegister, 0, foo
Register, 2, baz
UnRegister, 1, bar
UnRegister, 2, baz
```

</details>
