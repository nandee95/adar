use adar::prelude::*;

struct TupleBuilder<T>(T);

impl TupleBuilder<()> {
    fn new() -> Self {
        Self(())
    }
}

impl<T> TupleBuilder<T> {
    fn add<J>(self, value: J) -> TupleBuilder<T::Output>
    where
        T: TupleConcat<(J,)>,
    {
        TupleBuilder(self.0.concat((value,)))
    }

    fn build(self) -> T {
        self.0
    }
}

fn main() {
    let tuple = TupleBuilder::new()
        .add(22u32)
        .add("hello")
        .add(true)
        .build();

    println!("Tuple: {:?}", tuple);
}
