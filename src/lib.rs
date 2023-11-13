#[macro_export]
macro_rules! define_graph {
    (
        $(#[$outer:meta])*
        $vis:vis graph $MyGraph:ident {

        }
    ) => {
        $(#[$outer])*
        $vis struct $MyGraph {

        }

        impl $MyGraph {
            pub fn new() -> Self {
                Self { }
            }
        }
    };
}
