use std::io;

fn main() {

    loop {
        println!("Bienvenue dans votre calculatrice simplifiée !");

        println!("Veuillez choisir une opération : +, -, *, /, ou bien 'quit' pour quitter");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line"); // avec le "&" j'utilise operation en tant que réference, il n'y aura donc pas de changement de propriété

        let operation = operation.trim(); // je prend le résultat de opération.trim(), je fais du SHADOWING

        if operation == "quit"{
            break;
        }

        let valid_operations = ["+", "-", "*", "/"];

        let mut is_valid_operation: bool = false;

        for &valid_op in &valid_operations  { // je fais "&" car je ne veux pas modifier ce qu'il y a dans mon tableau "valid_operations", j'utilise donc une réference
            if operation == valid_op {
                is_valid_operation = true;
                break;
            }
        }

        if !is_valid_operation {
            println!("L'opération n'est pas possible");
            continue;
        }
      
        println!("Entrez le premier nombre :");       
        let num1 = read_number();

        println!("Entrez le second nombre :");
        let num2 = read_number();
    

        let result: f64;

        if operation == "+" {
            result = num1 + num2
        } else if operation == "-" {
            result = num1 - num2
        } else if operation == "*" {
            result = num1 * num2
        } else  {
            result = num1 / num2
        }

        println!("Résultat : {num1} {operation} {num2} = {result} ");
    }
    

}

fn read_number() -> f64 {
    let mut res = 0.0;

    while res == 0.0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
       match input.trim().parse::<f64>() { // Permet de gérer le result du retour d'une méthode
        Ok(value) => {
            res = value;
        },
        Err(_) => {
            println!("Ce n'est pas un nombre valide")
        }
       }
    }
       res // je return cette valeur sous forme d'expression car je n'ai pas de ";"

}