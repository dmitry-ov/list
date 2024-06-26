Каждый элемент такого списка содержит объект и указатель на следующий элемент. Таким образом, элементы списка объединины в цепь, в которой каждый элемент знает о следующем.
Подробности на вики: https://clck.ru/332iN9
Список должен уметь:

- Возвращать итератор по всем элементам, 
- добавлять элемент в конец,
- добавлять элемент в начало,
- добавлять элемент после N-го,
- Разделяться на два списка: от начального элемента до (N-1)-го и от (N-1)-го до последнего.
- Предоставлять возможность изменять элементы списка.

Так как каждый элемент списка содержит ссылку на следующий, Rust не даст нам менять элементы списка (правило заимствования о одной мутабельной ссылке). Для преодоления этого ограничения можно использовать обёртку Rc<RefCell>. Она даст возможность модифицировать элемент списка несмотря на то, что на него существует ссылка (у предыдущего элемента).

Требования:
- Все перечисленные методы реализованы.
- Все методы протестированы.
- Написан пример кода, демонстрирующий функционал списка.
- "cargo clippy" и "cargo fmt --check" не выдают предупреждений и ошибок.