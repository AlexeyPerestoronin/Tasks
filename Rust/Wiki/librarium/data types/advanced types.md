[← back to library](../../librarium.md)

Далее перечислены некоторые особенности работы с системой типов в Rust:  
1. **Rust позволяет создавать псевдонимы для типов**:  
    ```rust
    type Kilometers = i32;
    ```
    Псевдонимы типов не являются отдельными типами, а только дополнительным именем, ассоциированным с типом.

0. **Тип Never - `!`**:  
    Never - это тип, который никогда не возвращает значение.  
    1. Функции, которые никогда ничего не возвращают:  
        Функции, которые всегда паникуют или зацикливаются, могут иметь тип Never.
        ```rust
        fn panic_function() -> ! {
            panic!("This function never returns!");
        }
        ```
    0. Бесконечные циклы:  
        Бесконечные циклы, которые никогда не завершаются, также могут быть выражены с использованием типа Never.
        ```rust
        fn infinite_loop() -> ! {
            loop {
                // Do something forever
            }
        }
        ```
    0. Использование в match выражениях:  
        Тип Never может быть полезен в match выражениях, когда вы хотите указать, что определенная ветка никогда не будет достигнута.
        ```rust
        fn process_value(value: Option<i32>) {
            match value {
                Some(v) => println!("Value: {}", v),
                None => unreachable!("This should never happen!"),
            }
        }
        ```
    0. Прерывание программы:  
        Использование `std::process::exit` для завершения программы также возвращает `!`, так как программа завершает выполнение и не возвращает управление.
        ```rust
        fn terminate_program() -> ! {
            std::process::exit(1);
        }
        ```

0. **Типы функций**:  
    Типы функций определяются их сигнатурой, например, `fn(i32) -> i32`.  

0. **Тип unit - `()`**:  
    unit - эти тип, аналог `void` в других языках.  
    Используется для обозначения отсутствия значения.  

0. **Динамические типы**:  
    Динамические типы - это все типы, размер которых становится известным на этапе компиляции.  
    Технически данные типы прячутся за указателем на структуру, которая содержит указатель на область памяти данных типа и выделенный для него размер, который вычисляется на этапе компиляции программы.  

    Для использования динамических типов, необходимо экранировать доступ к ним через ссылку или какой-либо smart pointers (`Box<str>`).

    Вот список динамических типов в Rust:  
    * `[T]`, где `T` - это любой тип
    * `str` (по сути, это тип `[u8]`)

0. **Тип указателя на функцию:**:  
    Указатель на функцию, имеет специальный синтаксис: `let <var_name> : fn(<function parameters>) -> (<function return type>) = pointer to function`:  
    ```rust
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    fn main() {
        let one : fn(i32) -> i32  = add_one;
        let two : fn(fn(i32) -> i32, i32) -> i32 = do_twice;

        println!("The answer is: {}", two(one, 1));
    }
    ```
