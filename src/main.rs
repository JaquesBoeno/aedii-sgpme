use aedii_sgpme::data_structs::hashmap::MyHashMap;

fn main() {
    let mut map = MyHashMap::new();
    let key = "chave";
    map.put(key, 2);

    let r = map.get(&key);

    match r {
        Some(x) => {
            println!("Valor encontrado {}, e a len é {}", x, map.len())
        }
        None => println!("valor não encontrado"),
    }
}
