// Disciplina : Linguagem e Lógica de Programação
// Professor : Alan Furukawa
// Descrição : Aqui você descreve o que o programa faz! (função)
// Autor(a) : Gabriel Aguiar Rocha
// Data atual : 04/10/2021

use std::io;

fn ler()->i32{
    let mut input=String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().unwrap()
}

fn main() {
    let mut i=1;
    println!("digite um o {}º numero ",i);
    let mut num1=ler();
    println!("digite um o {}º numero ",i+1);
    let mut num2=ler();
    println!("digite um o {}º numero ",i+2);
    let mut num3=ler();
    println!("digite um o {}º numero ",i+3);
    let mut num4=ler();
    println!("digite um o {}º numero ",i+4);
    let mut num5=ler();

    let mut maior= num1;
    let mut menor =num1;

    if num2 > maior {
        maior = num2;
    }
    if num2 < menor {
        menor = num2;
    }
    if num3 > maior {
        maior = num3; }
    if num3 < menor {
        menor = num3;
    }
    if num4 > maior {
        maior = num4;
    }
    if num4 < menor {
        menor = num4;
    }
    if num5 > maior {
        maior = num5;
    }
    if num5 < menor {
        menor = num5;
    }
    println!("o maior numero é {} e o menor é {}",maior,menor);


}
