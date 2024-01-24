fn substituir_letras(mut palavra: String, posicoes: Vec<usize>, substituicao: char) -> String {
    for &indice in &posicoes {
        if let Some(caractere) = palavra.chars().nth(indice) {
            palavra = palavra
                .chars()
                .enumerate()
                .map(|(i, c)| if i == indice { substituicao } else { c })
                .collect();
        }
    }
    palavra
}

fn main() {
    let palavra_original = String::from("rust");
    let posicoes_a_substituir = vec![1, 3];  // Substituir o segundo e o quarto caractere
    let substituicao = '*';

    let resultado = substituir_letras(palavra_original, posicoes_a_substituir, substituicao);

    println!("{}", resultado);
}