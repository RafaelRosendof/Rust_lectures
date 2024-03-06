use std::io;
use rand::Rng;

fn tratamento(mut entrada: String) -> i32{
    loop{
        entrada.clear();
        io::stdin().read_line(&mut entrada).expect("Deu algum problema");

        match entrada.trim().parse(){
            Ok(figas) => return figas,
            Err(_) => println!("Digite um n´mero v´lido ")
        }
    }
}

fn random(a: i32 , b: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(a..=b);

    //define números aleatórios entre A e B 
}

fn main() {
    println!("Começando o nosso jogo de advinha \n");

    println!("Escolha a dificuldade 1 para fácil 2 para Difício! ");

    let  escolha = tratamento(String::new()); // iniciamos a escolha


    let num_aleatorio=random(1 , 100);


    let mut vidas: i32;

    if escolha == 1{
        vidas = 10;
    }else{
        vidas = 5;
    }

    println!("Começando o loop");

    while vidas > 0{
    print!("Escolha um número entre 0 e 100\n");
    
    let num = tratamento(String::new());

    //Agora checamos se o nosso número bateu com o número random da função random

    if num == num_aleatorio {
        println!("Voce ganhou o jogo");
        std::process::exit(1);
    }

    else if num > num_aleatorio{ //podemos notar que no rust não precisa de () 
        println!("Número é maior que o esperado \n");
        vidas -= 1;
        println!("Restam {} vidas",vidas);
    }

    else{
        println!("Número é menor que o esperado \n");
        vidas -= 1;
        println!("Restam {} vidas",vidas);
    }

    }    

    println!("O número era {}",num_aleatorio);

}

