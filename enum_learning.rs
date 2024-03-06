//definindo um enum chamado status
enum Status{
    Ok,
    Error(String),
}

//função que processa um Status
fn processa_status(s: Status){
    match s{
        Status::Ok => println!("Tudo correto! "),
        Status::Error(msg) => println!("Error {}",msg),
    }
}
//Definindo um enum chamado shape
enum Shape{
    circle(f64),
    retangulo(f64,f64),
}
//Implementando um método para o enum shape 
impl Shape{
    //definindo uma função para calcular a área
    fn area(&self) -> f64{
        match *self{
            Shape::circle(radius) => 3.14 * radius *radius,
            Shape::retangulo(altura , largura) => altura * largura,
        }
    }
}

fn main(){
    //aplicação do enum Status
    let status_ok = Status::Ok;
    let status_error = Status::Error(String::from("Erro crítico"));

    processa_status(status_ok);
    processa_status(status_error);

    //Exemplo de uso do enum shape
    let circle = Shape::circle(30.35);
    let retang = Shape::retangulo(25.4 , 21.3);

    println!("Área do circulo {}" , circle.area());
    println!("Área do retangulo {}", retang.area());
}
