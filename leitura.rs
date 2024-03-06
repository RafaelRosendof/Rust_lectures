use std::io; //é um módulo que tras facilidades para trabalhar com input e output de dados é uma
             //parte da biblioteca padrão do rust std que é uma coleção de facilidades 


//vamos entender o que é isso tudo 


fn main(){
    println!("Coloque o primeiro número: ");

    //lendo o primeiro número
    let mut int1 = String::new(); //lendo o primeiro número
    io::stdin().read_line(&mut int1).expect("Falhou em ler a linha \n");
//Guardamos o int1 em uma String para ser lida e após isso o metodo 'expect' trata qualquer erro
//que possa ocorrer durante a leitura, não precisa mas é bom ter
    

// Analisa (parse()) a String input1 para um número inteiro (i32). O match é usado para lidar com o resultado da análise. Se for bem-sucedido (Ok)
//num1 recebe o valor. Se houver um erro (Err), imprime uma mensagem de erro e encerra o programa.
//
    let num1: i32 = match int1.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Input inválido, entre com um número válido");
            return;
        }
    };

    println!("Digite o segundo número: ");

    let mut int2 = String::new();
    io::stdin().read_line(&mut int2).expect("Falhou em ler a linha \n");

    let num2: i32 = match int2.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Deu merda, coloca um inteiro válido");
            return;
        }
    };

    //vou explicar tudo isso detalhado dps, fica calmo 
    let sum = num1 + num2;
    println!("A soma deu {}",sum);
}
