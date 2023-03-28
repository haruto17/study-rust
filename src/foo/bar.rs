pub fn bar_func() {
    println!("Bar!");
    crate::foo::bar::bar_hello(); // ルートモジュールのfooのbarのbar_hello()
    super::bar::bar_hello(); // 親モジュールのbarのbar_hello()
    self::bar_hello(); // 自モジュールのbar_hello()
}

pub fn bar_hello() {
    println!("Hello!");
}
