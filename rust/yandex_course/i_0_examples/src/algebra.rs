#[derive(Debug)]
enum Command {
    Init,                                // вариант "как есть", без доп.данных
    WaitSeconds(usize),                  // вариант с одной переменной
    WriteDataOffsetSize(u64, u32),       // вариант с несколькими переменными
    ReadData { offset: u64, size: u32 }, // вариант с несколькими именованными переменными
}

struct S1 {
    key1: u64,
    key2: u32,
}

enum Mode {
    ReadOnly {
        guest_key: String,
    },
    Write {
        admin_key: String,
        access_role: String,
    },
    WelcomeMessage,
}
enum InputKind {
    File { filename: String },
    Http { port: u16 },
}
// В enum вы не обязаны ассоциировать разными вариантами
// разные форматы данных и имена переменных
// Да, ниже мы указываем одни и те же форматы и имена, но зато нам не нужно помнить,
// с каким режимом ассоциирован filename! Мы эконимим своё внимание - и таким образом
// допускаем меньше ошибок
enum SourceKind {
    UserFile { filename: String },
    SystemFile { filename: String },
}

enum OuterWalletTouch {
    ContractUpdate {
        change: i32,
        reason: String,
        source: String,
    }, // как структура
    P2pIncome(u32, String), // как кортеж
}

pub fn fake_main() {
    let init = Command::Init;
    let read_data = Command::ReadData {
        offset: 512,
        size: 64,
    };
    for command in [init, read_data] {
        println!("{:?}", command);
    }

    struct S2; // unit struct
    struct S3(usize); // tuple struct
    struct S4(u64, u32); // tuple struct 

    let touches = vec![
        OuterWalletTouch::P2pIncome(100, "Eisenhorn".into()),
        OuterWalletTouch::ContractUpdate {
            change: 200,
            reason: "deposit".into(),
            source: "Goldman Sachs".into(),
        },
        OuterWalletTouch::P2pIncome(0, "Ravenor".into()),
        OuterWalletTouch::ContractUpdate {
            change: -100,
            reason: "credit".into(),
            source: "Horns and Hooves".into(),
        },
    ];

    // Использование алгебраического типа данных

    let mut sources = vec![];
    for touch in touches {
        match &touch { // match по ссылке
    OuterWalletTouch::ContractUpdate{change, source, reason}
    // Здесь:
    // 1. однострочная ветка match
    //    (все ветки должны возвращать одно и то же,
    //     здесь не возвращается ничего, то есть `()`)
    // 2. сырые r#""# -строки, где не нужно экранировать
    //    (r##""##, если может встретиться `"#`)
    // 3. выравнивание строк в println! (в format! тоже работает)
    //    через :4, знак > задаёт 'направление' выравнивания
    => println!( r#"From {:>16} change: {:4} (reason: "{:7}")"#,
            source, change, reason ),
    // С помощью if можно задать специальную ветку для некоторых значений enum
    OuterWalletTouch::P2pIncome(update, source) if *update == 0
    => println!("     {:>16} checked wallet is reachable", source),
    // Кроме однострочников, можно использовать {}-блоки
    OuterWalletTouch::P2pIncome(update, source) => {
    println!("From {:>16} change: {:4}, p2p", source, update);
    },
    // Паттерн "всё остальное"
    // (в этом примере будет вызывать warn "unreachable pattern",
    //  так как мы все ветки уже покрыли)
    _ => println!("unknown"),
    }
        sources.push(match touch {
            // match по значению
            // Здесь:
            // 1. _ можно привязывать и к отдельным полям
            // 2. Поля можно 'переименовывать' (здесь: `source: my_source`)
            // 3. Паттерны можно объединять через |
            OuterWalletTouch::ContractUpdate {
                change: _,
                reason: _,
                source: my_source,
            }
            | OuterWalletTouch::P2pIncome(_, my_source) => my_source,
        });
    }
    println!("\nall sources: {:?}", sources);
}
