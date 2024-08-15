
template<typename... T>
union Sandwich;

template<typename T, typename... Tail>
union Sandwich<T, Tail...> final {

    T head;
    Sandwich<Tail...> tail;
};