# Интересные способы решения:
1. Если предстоит работать со строками, то рассмотреть:
    - целесообразность обхода строки в обратном направлении;
    - формирование списка слов (`std::list<std::string>`) из строки;
0. Если предстоит работать с массивами, то рассмотреть:
    - целесообразность предварительной сортировки;
        - целесообразность преобразования в карту (`std::map<ValueType, size_t>`) после сортировки;
0. Если предстоит работать с матрицами, то рассмотреть:
    - возможность целевого образования значений элементов матрицы в процессе первичного обхода (так, например, можно сохранять минимальный путь)
    - целесообразность формирования списка задач по факту первичного обхода элементов матрицы:
        ```cpp
        std::list<void()> tasks{};
        for (size_t i = 0; i < m; ++i) {
            for (size_t j = 0; j < n; ++j) {
                if (/* target condition */) {
                    tasks.emplace_back([&]() { /* task's operations... */ });
                }
            }
        }
        for (auto& task: tasks) {
            task();
        }
        ```