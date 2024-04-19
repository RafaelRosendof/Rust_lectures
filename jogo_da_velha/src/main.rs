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
fn printa_tabu(tabuleiro: [[char ; 3]; 3]){
    for linha in tabuleiro.iter(){
        for &cel in linha{
            print!("|__{}__| + ",cel);
        }
        println!();
        println!();
    }
}

fn recebe_jogada(jogador: &Jogador , tabuleiro: &mut [[char ; 3] ; 3]){ //matriz
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

       if tabuleiro[linha][coluna]=='?'{
        tabuleiro[linha][coluna] = match jogador{
            Jogador::X => 'X',
            Jogador::O => 'O',
        };
        break;
       }else{
           println!("Posição ocupada, tente outra! ");
       } 

    }
}
    
fn verifica_ganhar(tabuleiro: &[[char ; 3] ; 3]) -> Option<Jogador> {

   // let linhas = [0 , 1 , 2];
   // let colunas = [0 , 1 , 2];
    //testar todas as linhas, colunas e diagonais e caso dê empate

    //testando diagonais  
    if tabuleiro[0][0] != '?' && tabuleiro[0][0] == tabuleiro[1][1] && tabuleiro[1][1] == tabuleiro[2][2] || tabuleiro[0][2] != '?' && tabuleiro[0][2] == tabuleiro[1][1] && tabuleiro[1][1] == tabuleiro[2][0]{
        
        return match tabuleiro[1][1]{
            'X' => Some(Jogador::X),
            'O' => Some(Jogador::O),
            _ => None,
        };
    }
    //linha 
//    for i in linhas{
//            if tabuleiro[0][i] == tabuleiro[1][i] && tabuleiro[1][i] == tabuleiro[2][i]{
//                if tabuleiro[i][0] == 'X' { return Some(Jogador::X);}
//                else{return Some(Jogador::O);}

//        }
//    }
    for linha in tabuleiro{
        if linha[0] != '?' && linha[0] == linha[1] && linha[1] == linha[2]{
            return match linha[0]{
                'X' => Some(Jogador::X),
                'O' => Some(Jogador::O),
                _ => None,
            };
        }
    }

    //testando colunas 
//    for i in colunas{
//        if tabuleiro[i][0] == tabuleiro[i][1] && tabuleiro[i][1] == tabuleiro[2][i]{
//            if tabuleiro[0][i] == 'X' { return Some(Jogador::X);}
//            else{return Some(Jogador::O);}
//        }
//    }
    for colunas in 0..3{
        if tabuleiro[0][colunas] != '?' && tabuleiro[0][colunas] == tabuleiro[1][colunas] && tabuleiro[1][colunas] == tabuleiro[2][colunas]{
            return match tabuleiro[0][colunas]{
                'X' => Some(Jogador::X),
                'O' => Some(Jogador::O),
                _ => None,
            };
        }
    }

    None    
    //empate retorna????? 
}

fn main(){
    println!("\n Jogo Iniciado \n");

    // o main deve ser responsável por inicializar o jogo e fazer um loop para dar continuidade no
    let mut tabuleiro = [['?' ; 3]; 3];
    let mut actual_player = Jogador::X;

   
    loop{
        printa_tabu(tabuleiro);

        recebe_jogada(&actual_player , &mut tabuleiro);

        if let Some(vencedor) = verifica_ganhar(&tabuleiro){
            match vencedor{
                Jogador::X => println!("Jogador 1 Venceu "),
                Jogador::O => println!("Jogador 2 Venceu"),
            }
            println!("\n Jogo Finalizado \n");
            break;
        }
        //should be the player O that plays now!!!!
        verifica_ganhar(&tabuleiro);
        actual_player = match actual_player {
            Jogador::X => Jogador::O,
            Jogador::O => Jogador::X,
        };

        //vendo um empate
        if tabuleiro.iter().all(|linha| linha.iter().all(|&cel| cel != '?')) {
            println!("Empate!");
            println!("\nJogo Finalizado\n");
            break; // Exit the loop if there's a tie
        }
    }
}
