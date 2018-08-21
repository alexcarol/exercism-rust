#[macro_export]
macro_rules! hashmap {
    () => {
        std::collections::HashMap::new()
    };
    ( $( $key:expr => $value:expr, )* ) => {
        {

            let mut map = hashmap!();
            $(
                map.insert($key, $value);
            )*

            map
        }
    };
    ( $( $key:expr => $value:expr ),* ) => {
        hashmap!($($key => $value,)*)
    };
}
