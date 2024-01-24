extern crate rand;
use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn encontrar_posicoes_letra(palavra: &str, letra: char) -> Vec<usize> {
    palavra
        .char_indices()
        .filter(|&(_, c)| c == letra)
        .map(|(indice, _)| indice)
        .collect()
}

fn cria_palavra_escondida(palavra: &str) -> String{
    let underscore = "_";
    underscore.repeat(palavra.len())    
}

fn revela_letras_escondidas(letra: char, palavra_escondida: &str, ocorrencias: &Vec<usize>) -> String{

    let mut palavra = palavra_escondida.to_string();
    for &indice in ocorrencias {
        palavra = palavra
                .chars()
                .enumerate()
                .map(|(i, c)| if i == indice && c == '_' {letra} else {c})
                //.map(|(i, c)| if i == indice { letra } else { c })
                .collect();
    }
    palavra
}

fn main(){
    
    let rnd_value : u32 = rand::thread_rng().gen_range(0, 9);
    let palavras: [&str; 10] = ["um", "dois", "tres","quatro","cinco","seis","sete","oito","nove","dez"];
    
    println!("Tente adivinhar a palavra aleatória");
    let usize_value: usize = rnd_value as usize;
    let palavra_escolhida = palavras[usize_value];
    
    let mut hidden = cria_palavra_escondida(palavra_escolhida);
    println!("A palavra selecionada: {}", hidden);

    loop {
        if hidden != palavra_escolhida {
            
            let mut data_input = String::new();

            println!("Digite uma letra : ");            
            io::stdin()
                .read_line(&mut data_input)
                .expect("Erro ao ler");
    
            // Obter a primeira letra da entrada (ignorar possíveis caracteres adicionais)
            let letra = data_input.trim().chars().next();
           
            
            match letra {
                Some(data_input) => {
                    // Contar ocorrências da letra na palavra
                    let ocorrencias = encontrar_posicoes_letra(&palavras[usize_value], data_input);
                    hidden = revela_letras_escondidas(data_input,&hidden, &ocorrencias);
                    println!("Palavra:  {}", hidden);
                    // Exibir resultado
                    if ocorrencias.len() > 0 {
                        println!(
                            "A letra '{}' ocorre {:?} nas posicoes na palavra.",
                            data_input, ocorrencias
                        );
                    } else {
                        println!("A palavra a ser adivinhada não possui a letra {}", data_input);
                    }
                }
                None => {
                    println!("Nenhuma letra digitada.");
                }
    
            }
        } else {
            println!("Voce acertou a palavra!!! Palavra : {}", palavra_escolhida);
            break;
        }
        

    }



}