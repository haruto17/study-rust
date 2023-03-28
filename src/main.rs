use futures::executor::block_on;
use rand::Rng;
use std::thread;
use std::time::Duration;

// foo.rsをモジュールとして使用する
mod foo;

// useによりモジュールのパスを省略
use foo::bar; // foo::barをbarとして参照できる

// asによりモジュールに別名をつけられる
use foo::bar as bbaarr;

// *を指定すると子要素をすべて使用できる
use foo::bar::*;

///////////////////////////////////////////////////////////////////////////
// マクロ
macro_rules! log {
    ($x:expr) => {
        println!("{}", $x);
    };
}
///////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////
// インプリメンテーション
struct Rect {
    width: u32,
    height: u32,
}

// implによって構造体にメソッドを加えることができる。selfは自オブジェクトを示す
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
///////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////
// トレイト

///////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////
// イテレータ
struct Counter {
    max: u32,
    count: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { max: max, count: 0 }
    }
}

// イテレータはforで利用することができる
// next()により次のオブジェクトを返却し、最後に達するとNoneを返却する
// Selfはimplにおける自分自身の型を示す
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.max {
            Some(self.count)
        } else {
            None
        }
    }
}
///////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////
// 非同期関数
struct Song {
    lyric: String,
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn learn_song() -> Song {
    let song = Song {
        lyric: String::from("La la la..."),
    };
    println!("Learned song");
    return song;
}

async fn sing_song(song: Song) {
    println!("{}", song.lyric);
}

async fn dance() {
    println!("Dance");
}

async fn async_main() {
    let f1 = learn_and_sing(); // 歌を習って歌う
    let f2 = dance(); // ダンスする
    futures::join!(f1, f2);
}
///////////////////////////////////////////////////////////////////////////
fn main() {
    println!("Hello, world!");

    // 変数宣言（イミュータブル）
    let n = 0;

    // 変数宣言（ミュータブル）
    let mut m = 0;

    // 定数宣言
    const MAX_POINTS: u32 = 100;

    // 型変換
    let x: i32 = 123;
    let y: i64 = x as i64;

    // タプル
    let tup = (10, "20", 30);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // 配列（要素数は固定）
    let arr = [10, 20, 30];
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    for v in &arr {
        println!("{}", v);
    }

    // ベクタ（要素数は可変）
    let mut vec = vec![10, 20, 30];
    vec.push(40);
    println!("{} {} {} {}", vec[0], vec[1], vec[2], vec[3]);
    for v in &vec {
        println!("{}", v);
    }

    // 文字列（&str）
    let mut name = "TANAKA";
    name = "SASAKI";

    // 文字列（String） 文字列の連結などが可能
    let mut name = String::from("TANAKA");
    // 別の文字列設定
    name = "SASAKI".to_string();
    // 文字列の追加
    name.push_str("TARO");

    // ヒープ領域のメモリ確保
    // スタック領域（i32,&str）=> 関数が呼ばれると積み重なっていき、関数が終わると解放
    // ヒープ領域（String,Vec）=> 関数が終わっても存在。メモリは所有者が居なくなった時点で解放
    let p: Box<Point> = Box::new(Point { x: 100, y: 200 });
    println!("{} {}", p.x, p.y);

    // スライス
    let s = String::from("ABCDEFGHIJKLMN");
    let s1 = &s[0..3]; // 0番目から3番目の手前までのスライス（"ABC"）
    let s2 = &s[3..6]; // 3番目から6番目の手前までのスライス（"DEF"）
    println!("{} {}", s1, s2);

    let a = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let a1 = &a[0..3]; // 0番目から3番目の手前までのスライス[10,20,30]
    let a2 = &a[3..6]; // 3番目から6番目の手前までのスライス[40,50,60]
    println!("{:?} {:?}", a1, a2);

    // クロージャー
    let square = |x: i32| x * x;
    println!("{}", square(9));

    // moveはクロージャー内で参照するクロージャー外変数が存在する場合、その所有権をクロージャーに移動させることを宣言する
    let msg = String::from("Hello"); // クロージャー外変数msg
    let func = move || {
        // 所有権をクロージャーに移動することを宣言
        println!("{}", msg); // 参照したクロージャー外変数の所有権はクロージャーに移動
    }; // クロージャー終了時に所有者が不在となり解放される
    func(); // クロージャー呼び出し
            // println!("{}",msg) // 解放領域を参照しようとするのでエラーとなる

    // マクロ名!でマクロの呼び出し
    log!("ABC...");

    // if文
    if n == 1 {
        println!("One");
    } else if n == 2 {
        println!("Two");
    } else {
        println!("Other");
    }
    // if文も式なので、次のように書ける
    let q = if n == 1 { "OK!" } else { "NG!" };

    // while文
    let mut w = 0;
    while w < 10 {
        w += 1;
    }

    // for文
    for i in 0..10 {
        println!("{}", i);
    }

    // ループ
    let mut l = 0;
    loop {
        l += 1;
        if l == 2 {
            continue;
        }
        if l == 10 {
            break;
        }
        println!("{}", l);
    }

    // マッチ
    let m = 2;
    match m {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("More"),
    }

    let r = Rect {
        width: 200,
        height: 300,
    };
    println!("{}", r.area());

    let counter = Counter::new(10);
    for c in counter {
        println!("{}", c);
    }

    // マルチスレッド
    // スレッドの起動
    // 引数にクロージャーを指定
    let th = thread::spawn(|| {
        for i in 1..10 {
            println!("A");
            thread::sleep(Duration::from_millis(100));
        }
    });
    th.join().unwrap();
    println!("Finished");

    // スレッドからスレッド外の変数を参照するには、moveによって変数の所有権をスレッドに引き渡すことを明示する必要がある
    let str = String::from("ABC");
    let th2 = thread::spawn(move || {
        // 所有権を引き渡すことを明示
        for i in 1..10 {
            println!("{}", str); // strの所有権を得る
            thread::sleep(Duration::from_millis(100));
        }
    });
    th2.join().unwrap();
    println!("Finished");
    // println!("{}",str); // 所有権が移動済みのためエラー

    block_on(async_main());

    // randクレートを使って、1～101の間で乱数生成
    let mut rng = rand::thread_rng();
    for i in 1..10 {
        println!("{}", rng.gen_range(1..101));
    }

    // モジュールfoo.rs内のfoo_func()の実行
    foo::foo_func();

    // foo.rsでモジュールとしてインポートしたbar.rs内のbar_func()の実行
    foo::bar::bar_func();

    bar::bar_func();

    bbaarr::bar_func();

    bar_func();
}

///////////////////////////////////////////////////////////////////////////
// 関数
// returnは戻り値を呼び出し元に返す
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

// returnされない場合は、最後の式が戻り値として返される（セミコロンはなし）
fn divide(x: i32, y: i32) -> i32 {
    x / y
}
///////////////////////////////////////////////////////////////////////////

// Dropトレイトで、メモリ解放時に処理を実行（デストラクタ）
impl Drop for Point {
    fn drop(&mut self) {
        println!("Bye!");
    }
}

// 構造体
struct Point {
    x: i32,
    y: i32,
}

// 共用体
// f1とf2は同じメモリを共用する
union MyUnion {
    f1: u32,
    f2: u32,
}

// 列挙型
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}
