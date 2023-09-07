#[macro_export]
macro_rules! vec_comprehension{
    ($exp:expr, $var:ident in $cont:expr, $(if $cond:expr)?) => {
        {
            let mut aux = Vec::new();
            for $var in $cont {
                $(if $cond)? {
                    aux.push($exp);
                }
            }
            aux
        }
    };
}

pub use vec_comprehension;

#[macro_export]
macro_rules! map {
    ($key:ty, $val:ty) => {
        {
            let aux: HashMap::<$key,$val> = HashMap::new();
            aux
        }
    };
    ($($key:expr => $val:expr), *) => {
        {
            let mut aux = HashMap::new();
            $(aux.insert($key, $val);)*
            aux
        }
    };
}
pub use map;