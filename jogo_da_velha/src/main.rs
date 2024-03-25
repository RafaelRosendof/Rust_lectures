//Jogo da velha 
//
//


//Primeiro precisa fazer a função que imprime o tabuleiro 3x3 que recebe uma matriz de char e
//retorna o tabuleiro preenchido e precisa estar atualizado 
//
enum Jogador{
    X,
    O,
}
fn printa_tabu(tabuleiro: [[i32 ; 3]; 3]){
    for linha in tabuleiro.iter(){
        for &cel in linha{
            print!("|__{}__| + ",cel);
        }
        println!();
        println!();
    }
}

fn recebe_jogada(jogador: Jogador , tabuleiro: &mut [[i32 ; 3] ; 3]){ //matriz
    //recebe a posição e verifica se a posição é válida 
       println!("Digite o valor na posição que deseja do 1 ao 9"); 
       loop{
          let mut jogada = String::new();
           std::io::stdin().read_line(&mut jogada).expect("Falha ao ler a entrada");
           
            let jogada: usize = match jogada.trim().parse(){
                Ok(num) => num,
                Err(_) =>{ println!("Entrada inválida, entre novamente ");
                continue;}
            };

            //colocar linha e coluna indo de 1 até 9
            let linha = (jogada - 1)/3;
            let coluna = (jogada -1) %3;

            //testar condicionais para ver se entrada é válida.

       if tabuleiro[linha][coluna]==0{
        tabuleiro[linha][coluna] = match jogador{
            Jogador::X => 1,
            Jogador::O => 2,
        };
        break;
       }else{
           println!("Posição ocupada, tente outra! ");
       } 

    }
}
    
fn verifica_ganhar(tabuleiro: &mut [[i32 ; 3] ; 3]) -> Option<Jogador> {

    let linhas = [0 , 1 , 2];
    let colunas = [0 , 1 , 2];
    //testar todas as linhas, colunas e diagonais e caso dê empate
    if(tabuleiro[0][0] == tabuleiro[1][1] && tabuleiro[2][2] || tabuleiro[0][2] && tabuleiro[1][1] && tabuleiro[2][0]){
        if tabuleiro[1][1] == "X"{
            return Jogador::X;
        }
        else{return Jogador::O;}
    }
    //linha 
    for i in linhas{
            if tabuleiro[0][i] == tabuleiro[1][i] && tabuleiro[1][i] == tabuleiro[2][i]{
                if tabuleiro[i][0] == "X" { return Jogador::X;}
                else{return Jogador::O;}

        }
    }

    for i in colunas{
        if tabuleiro[i][0] == tabuleiro[i][1] && tabuleiro[i][1] == tabuleiro[2][i]{
            if tabuleiro[0][i] == 1 { return Jogador::X;}
            else{return Jogador::O;}
        }
    }

    //empate retorna????? não sei implementar essa lógica
}

fn main(){
    println!("\n Jogo Iniciado \n");

    // o main deve ser responsável por inicializar o jogo e fazer um loop para dar continuidade no
    let mut tabuleiro = [[0 ; 3]; 3];
    let mut jogador_atual = Jogador::X;

   
    loop{
        printa_tabu(tabuleiro);

        if let Some(vencedor) = verifica_ganhar(&tabuleiro){
            match vencedor{
                Jogador::X => println!("Jogador 1 Venceu "),
                Jogador::O => println!("Jogador 2 Venceu");
            }
        }
    }
  
    println!("\nJogo Finalizado\n");
}
