fn main(){
    println!("Fala figas \n");

    let A: i32 = 30; //aqui definimos a variável A como sendo um inteiro, o rust por decreto tem
                     //que um inteiro é do tipo i32 mas pode ser escrito como let A = 30 que dá a
                     //mesma coisa pois o rust tem sua inferência, mas as vezes é bom especificar o
                     //tipo da variável pois pode confundir o interpretador do rust ou ficar mais
                     //legivél para o programador 
    assert_eq!(A, 30 );// Aqui o assert_eq!(, ); funciona como uma expectativa, meio que uma // comparação entre o A e o 30  

    println!("Deu bom \n");

    let b = 250.9;

 //   assert_eq!(b, 30 ); //essa informação deu errado pois Comparou um FLOAT com INT dando erro
  //  println!("Deu merda");
    assert_eq!(b,250.9 );
    println!("Deu bom denovo, puta merda!\n");

    println!("#####################\n\n");


    let mut n: i32 = 2;
    n+=3;
    assert_eq!(n,5 );
    println!("Usando o mut \n");

    //aqui usamos o mut para deixar essa variável mutavel, coloca o mut antes e ela pode assumir
    //valores diferentes ao longo do código.
    

    //variáveis podem ser inicializadas dentro de pedaços de {} 
    //ex:
    {
        println!("Printando n dentro de um escopo de {}",n);

    }

    println!("Chamando a minha primeira função !");
    chama();

    println!("####################\n\n");

    //exemple of SHADOWING
    let fig: i32 = 100;
    println!("{}",fig);
    let fig: &str = "Eu sou o pica das galáxias \n";
    println!("{}",fig);
    println!("SUCESSO");

    //A mesma variável pode assumir diferentes valores conforme eu dou a tipagem, porém caso eu
    //coloque let a =10 e depois eu coloque a+=10 ai eu não posso utilizar isso mas sim eu posso
    //colocar inteiros floats e strings com a mesma assinatura desde que eu utilize a tipagem.

    //tuplas!!!
    let (mut x,mut y) = (1,5);
    x+=2;
    assert_eq!(x , 3);
    assert_eq!(y,5 );
    println!("Deu certo com as tuplas!!! {} {}",x,y);

    // ou podemos usar assim
    let(B , N);
    (B,..) = (3,4);
    [.., N] = [1,2];
}

fn chama(){
    let x: &str = "Tudo bom Figas";
    println!("{}",x);
}
//declaramos uma função assim. basicamente como as outras, e chama no main 

