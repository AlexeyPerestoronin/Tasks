[← back to library](../../librarium.md)

В Rust трейты `Sync` и `Send` используются для обеспечения безопасного доступа к данным в многопоточных программах. Трейт `Sync` указывает, что тип может быть безопасно передан между потоками, если к нему обращаются через ссылку. Это означает, что если тип `T` реализует `Sync`, то `&T` может быть передан между потоками.

Основные аспекты трейта `Sync`:
1. **Автоматическая реализация**:  
    Rust автоматически реализует Sync для типов, которые безопасно использовать из нескольких потоков одновременно. Например, примитивные типы, такие как `i32` и `f64`, являются Sync.  
    Типы, которые содержат только Sync поля, также автоматически становятся Sync.
0. **Не Sync типы**:  
    Типы, которые не являются Sync, обычно содержат внутреннее состояние, которое может изменяться, например, RefCell или Rc. Эти типы не могут быть безопасно переданы между потоками без дополнительной синхронизации.
0. **Использование Sync**:  
    В большинстве случаев вам не нужно явно реализовывать Sync. Вместо этого вы используете типы, которые уже реализуют Sync, такие как `Arc<Mutex<T>>`, для безопасного доступа к данным из нескольких потоков.

Пример использования:  
В следующем примере показано, как использовать Arc и Mutex для безопасного доступа к данным из нескольких потоков:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *data.lock().unwrap());
}
```

Объяснение:
* Arc (Atomic Reference Counted): используется для обеспечения безопасного доступа к данным из нескольких потоков, увеличивая счетчик ссылок атомарно.
* Mutex: используется для обеспечения взаимного исключения, позволяя только одному потоку изменять данные в данный момент времени.
* lock(): блокирует Mutex и возвращает изменяемую ссылку на данные, защищенные Mutex.
В этом примере Arc<Mutex<i32>> реализует Sync, что позволяет безопасно передавать его между потоками.