#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    bar: String,
}

impl FooBuilder {

    pub fn name(mut self, bar: String) -> FooBuilder {
        self.bar = bar;
        self
    }

    pub fn build(self) -> Foo {
        Foo { bar: self.bar }
    }
}

mod tests {
	use super::*;

	#[test]
	fn builder_test() {
	    let foo = Foo {
	        bar: String::from("Y"),
	    };
	    let foo_from_builder: Foo = Foo::builder().name(String::from("Y")).build();
	    assert_eq!(foo, foo_from_builder);
	}
}

