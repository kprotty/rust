macro_rules! count {
    ( $( $e:stmt ),* ) => {
        ${ count(e) }
        //~^ ERROR meta-variable expressions are unstable
    };
}

macro_rules! index {
    ( $( $e:stmt ),* ) => {
        $( ${ignore(e)} ${index()} )*
        //~^ ERROR meta-variable expressions are unstable
        //~| ERROR meta-variable expressions are unstable
    };
}

macro_rules! ignore {
    ( $( $i:stmt ),* ) => {{
        0 $( + 1 ${ignore(i)} )*
        //~^ ERROR meta-variable expressions are unstable
    }};
}

macro_rules! length {
    ( $( $e:stmt ),* ) => {
        $( ${ignore(e)} ${length()} )*
        //~^ ERROR meta-variable expressions are unstable
        //~| ERROR meta-variable expressions are unstable
    };
}

fn main() {
}
