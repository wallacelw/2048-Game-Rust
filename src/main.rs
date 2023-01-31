use yew::prelude::*;
use rand::Rng;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

/*
Função que verifica se o jogo acabou, ou seja, não existe mais nenhum espaço vazio e nenhum conjunto de blocos podem ser unidos.
*/
fn check_end(matrix : &mut [[u128; 4]; 4]) -> bool {
    return true;
}

/*
Função para gerar um novo bloco em uma posição livre (=0) com
a probabilidade de 90% de gerar um bloco de valor 2 e probabilidade
de 10% de gerar um bloco de valor 4. A posição é escolhida de modo equiprovável.
Retorna um valor booleano indicando se o jogo acabou.
true -> acabou; false -> jogo segue
*/
fn generate_tile(matrix : &mut [[u128; 4]; 4]) -> bool {

    let mut zeros: usize = 0;
    for i in 0..4 {
        for j in 0..4 {
            if matrix[i][j] == 0 {zeros += 1};
        }
    }

    // Asserts if there is at least 1 empty space for generation
    assert!(!(zeros == 0), "Impossible game: no more zeros");
    if zeros == 0 {return false};

    let mut position: usize = rand::thread_rng().gen_range(0..(zeros));

    // value == 9 -> tile(4); 
    // value < 9 -> tile(2)
    let value: u128 = if rand::thread_rng().gen_range(0..10) == 9 {4} else {2};

    for i in 0..4 {
        for j in 0..4 {
            if matrix[i][j] == 0 {
                if position == 0 {
                    matrix[i][j] = value;
                    if zeros == 15 {
                        return check_end(matrix);
                    }
                    return false;
                }
                else {
                    position -= 1;
                }
            }
        }
    }

    return false;
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

// Movement of the game for the left arrow
fn move_left(matrix : &mut [[u128; 4]; 4]) {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    for k in 0..3 {
        for j in 1..(4-k) {
            for i in 0..4 {
                if (matrix[i][j] == matrix[i][j-1]) && (!merged[i][j-1]) {
                    matrix[i][j-1] *= 2;
                    matrix[i][j] = 0;

                    merged[i][j-1] = true;
                    merged[i][j] = false;
                }
                else if matrix[i][j-1] == 0 {
                    matrix[i][j-1] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i][j-1] = merged[i][j];
                    merged[i][j] = false;
                }
            } 
        }
        print_matrix(*matrix);
    }

    generate_tile(matrix);
}

// Movement of the game for the right arrow
fn move_right(matrix : &mut [[u128; 4]; 4]) {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    for k in 0..3 {
        for j in (k..3).rev() {
            for i in 0..4 {
                if (matrix[i][j] == matrix[i][j+1]) && (!merged[i][j+1]) {
                    matrix[i][j+1] *= 2;
                    matrix[i][j] = 0;

                    merged[i][j+1] = true;
                    merged[i][j] = false;
                }
                else if matrix[i][j+1] == 0 {
                    matrix[i][j+1] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i][j+1] = merged[i][j];
                    merged[i][j] = false;
                }
            } 
        }
        print_matrix(*matrix);
    }

    generate_tile(matrix);
}

// Movement of the game for the up arrow
fn move_up(matrix : &mut [[u128; 4]; 4]) {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    for k in 0..3 {
        for i in 1..(4-k) {
            for j in 0..4 {
                if (matrix[i][j] == matrix[i-1][j]) && (!merged[i-1][j]) {
                    matrix[i-1][j] *= 2;
                    matrix[i][j] = 0;

                    merged[i-1][j] = true;
                    merged[i][j] = false;
                }
                else if matrix[i-1][j] == 0 {
                    matrix[i-1][j] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i-1][j] = merged[i][j];
                    merged[i][j] = false;
                }
            } 
        }
        print_matrix(*matrix);
    }

    generate_tile(matrix);
}

fn move_down(matrix : &mut [[u128; 4]; 4]) {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    for k in 0..3 {
        for i in (k..3).rev() {
            for j in 0..4 {
                if (matrix[i][j] == matrix[i+1][j]) && (!merged[i+1][j]) {
                    matrix[i+1][j] *= 2;
                    matrix[i][j] = 0;

                    merged[i+1][j] = true;
                    merged[i][j] = false;
                }
                else if matrix[i+1][j] == 0 {
                    matrix[i+1][j] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i+1][j] = merged[i][j];
                    merged[i][j] = false;
                }
            } 
        }
        print_matrix(*matrix);
    }

    generate_tile(matrix);
}

fn main() {
    //yew::Renderer::<App>::new().render();
    let mut matrix: [ [u128; 4] ; 4] = [
        [4, 8, 4, 8],
        [8, 2, 8, 8],
        [8, 2, 8, 2],
        [2, 2, 8, 2]
    ];
    generate_tile(&mut matrix);

    // TODO create function to check if a movement is valid, aka, do something, so that it can be moved and generate a new block.

    // todo check function
}   