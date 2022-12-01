fn main() {
    
}

fn _变量解构() {
    let (a, b) = (1, 2);

    assert_eq!((a, b), (1, 2))
}

fn _解构赋值() {
    // struct Struct {
    //     e: i32,
    //     f: i32,
    // }

    // let (a, b, c, d, e, f);

    // (a, b) = (1, 2);
    // [c, .., d, e, _] = [1, 2, 3, 4, 5]; //‘_’代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了‘_’

    // Struct { f, .. } = Struct { e: 15, f: 18 };

    // println!("e:{},f:{}", e, f)
}

fn _常量和变量之间的差异() {
    const X: i32 = 1; //常量不能标注为 mut；常量必须显式标注其类型；命名规范为全部大写，且单词间用‘_’隔开；
    let _y = 2;
}

fn _变量遮蔽() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 3;
        println!("in scop x is:{}", x)
    }

    println!("x is:{}", x)
}

fn _变量遮蔽2() {
    let spance = "   ";
    let spance = spance.len();
    println!("{}", spance)
}

fn _数字运算() {
    let twenty = 20;
    let twenty_one = 21i32;
    let twenty_two: i32 = 32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{}+{}+{}={}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(1));

    let forty_tows = [42.0, 42f32, 42.5_f32];

    println!("{:.1}", forty_tows[2]);
}
