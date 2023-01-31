use yew::prelude::*;
use rand::Rng;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

/*
Função para gerar um novo bloco em uma posição livre ( =0 ) com
a probabilidade de 90% de gerar um bloco de valor 2 e probabilidade
de 10% de gerar um bloco de valor 4. A posição é escolhida de modo equiprovável.
*/
fn generate_tile(matrix : &mut [[u128; 4]; 4]) {

    let mut zeros: usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if matrix[i][j] == 0 {zeros += 1};
        }
    }

    let mut position: usize = rand::thread_rng().gen_range(0..(zeros));

    // value == 9 -> tile(4); value < 9 -> tile(2)
    let value: u128 = if rand::thread_rng().gen_range(0..10) == 9 {4} else {2};

    for i in 0..4 {
        for j in 0..4 {
            if matrix[i][j] == 0 {
                if position == 0 {
                    matrix[i][j] = value;
                    return;
                }
                else {
                    position -= 1;
                }
            }
        }
    }
}

// debugging function
fn print_matrix(matrix : [[u128; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
    println!();
}

fn transverse(matrix : &mut [[u128; 4]; 4], direction : [u128; 2]) {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    for k in 0..3 {
        for j in 1..4 {
            for i in 0..(4-k) {
                if matrix[i][j] == matrix[i][j-1] {

                    // merging with left cell
                    if !merged[i][j-1] {
                        matrix[i][j-1] = 2*matrix[i][j-1];
                        matrix[i][j] = 0;

                        merged[i][j-1] = true;
                        merged[i][j] = false;
                    }
                    // if left cell already merged do nothing
                }
                else if matrix[i][j-1] == 0 {
                    matrix[i][j-1] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i][j-1] = false;
                    merged[i][j] = false;
                }
            } 
        }
        print_matrix(*matrix);
    }
}

fn main() {
    //yew::Renderer::<App>::new().render();
    let mut matrix: [ [u128; 4] ; 4] = [
        [4, 0, 4, 0],
        [0, 2, 0, 0],
        [0, 2, 0, 2],
        [2, 2, 2, 2]
    ];
    print_matrix(matrix);
    transverse(&mut matrix, [0, 0]);
    print_matrix(matrix);
}   