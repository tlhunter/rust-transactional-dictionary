mod dict;
use dict::Dict;

fn main() {
    let mut d = Dict::new();

    println!("{}", d.has("potato".to_string()));

    d.set("potato".to_string(), "foo".to_string());

    println!("{}", d.has("potato".to_string()));
    println!("{:?}", d.get("potato".to_string()));

    d.delete("potato".to_string());
    println!("{}", d.has("potato".to_string()));

    d.set("potato".to_string(), "foo".to_string());

    d.begin();
    d.set("forget me".to_string(), "such forget".to_string());
    d.cancel();

    d.begin();
    d.set("spudz".to_string(), "is potat".to_string());
    d.commit();

    println!("{:#?}", d.data); // should contain [potato, spudz]
}
