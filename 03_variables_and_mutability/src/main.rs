fn main() {
    let number = 5; // Immutable variable.

    println!("Immutable | {}.", number);

    let mut mut_number = 0; // The <mut> keyword defines a mutable variable.

    println!("Mutable   | {}.", mut_number);

    mut_number = 6;

    println!("Mutable   | {}.", mut_number);

    // Constants are always immutable. But unlike immutable vars, constants can be declared on global scope.
    const NUMBER: u32 = 123;

    println!("Constant  | {}.", NUMBER);

    let shadow = 2;

    println!("Value before shadowing: {}.", shadow);

    {
        // Unlike mut let, shadowing creates another variable and can be assigned by another data type
        let shadow = shadow * 2;

        println!("Shadow value: {}.", shadow);
    }

    println!("Value after shadowing: {}.", shadow);
}
