use yew::prelude::*;
use rand::Rng;
use std::io;

// Cria o html
// Mas por enquanto o jogo não usa esse front
#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

/*
Função que verifica se o jogo acabou, ou seja, não existe mais nenhum espaço vazio e nenhum conjunto de blocos podem ser unidos. Só é chamada depois de verificado que não há mais nenhum espaço em branco.

true -> game over
false -> still at least one movement left
*/
fn check_end(matrix : &mut [[u128; 4]; 4]) -> bool {
    let mut movement: bool = false;
    for i in 0..4 {
        for j in 0..4 {
            if (i > 0) && (matrix[i-1][j] == matrix[i][j]) {
                movement = true;
            }
            if (i < 3) && (matrix[i+1][j] == matrix[i][j]) {
                movement = true;
            }
            if (j > 0) && (matrix[i][j-1] == matrix[i][j]) {
                movement = true;
            }
            if (j < 3) && (matrix[i][j+1] == matrix[i][j]) {
                movement = true;
            }
        }
    }
    return movement;
}

/*
Função para gerar um novo bloco em uma posição livre (=0) com
a probabilidade de 90% de gerar um bloco de valor 2 e probabilidade de 10% de gerar um bloco de valor 4. 

A posição é escolhida de modo equiprovável.

Retorna um valor booleano indicando se o jogo acabou:
    true -> acabou; 
    false -> jogo segue
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
                    
                    // remove this after front implementation
                    print_matrix(*matrix); 

                    // possible end game
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

// temporary debugging function
fn print_matrix(matrix : [[u128; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
    println!();
}

// Função de movimentação para esquerda
// Retorna se o jogo acabou
fn move_left(matrix : &mut [[u128; 4]; 4]) -> bool {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    let mut null_movement: bool = true;

    for k in 0..3 {
        for j in 1..(4-k) {
            for i in 0..4 {
                if (matrix[i][j] == matrix[i][j-1]) && (!merged[i][j-1]) {
                    matrix[i][j-1] *= 2;
                    matrix[i][j] = 0;

                    merged[i][j-1] = true;
                    merged[i][j] = false;
                    null_movement = false;
                }
                else if matrix[i][j-1] == 0 {
                    matrix[i][j-1] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i][j-1] = merged[i][j];
                    merged[i][j] = false;
                    null_movement = false;
                }
            } 
        }
        print_matrix(*matrix); // <- remove this after front
    }
    if null_movement {return false};

    return generate_tile(matrix);
}

// Função de movimentação para direita
// Retorna se o jogo acabou
fn move_right(matrix : &mut [[u128; 4]; 4]) -> bool {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    let mut null_movement: bool = true;

    for k in 0..3 {
        for j in (k..3).rev() {
            for i in 0..4 {
                if (matrix[i][j] == matrix[i][j+1]) && (!merged[i][j+1]) {
                    matrix[i][j+1] *= 2;
                    matrix[i][j] = 0;

                    merged[i][j+1] = true;
                    merged[i][j] = false;
                    null_movement = false;
                }
                else if matrix[i][j+1] == 0 {
                    matrix[i][j+1] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i][j+1] = merged[i][j];
                    merged[i][j] = false;
                    null_movement = false;
                }
            } 
        }
        print_matrix(*matrix); // <- remove this after front
    }

    if null_movement {return false};

    return generate_tile(matrix);
}

// Função de movimentação para cima
// Retorna se o jogo acabou
fn move_up(matrix : &mut [[u128; 4]; 4]) -> bool {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    let mut null_movement: bool = true;

    for k in 0..3 {
        for i in 1..(4-k) {
            for j in 0..4 {
                if (matrix[i][j] == matrix[i-1][j]) && (!merged[i-1][j]) {
                    matrix[i-1][j] *= 2;
                    matrix[i][j] = 0;

                    merged[i-1][j] = true;
                    merged[i][j] = false;
                    null_movement = false;
                }
                else if matrix[i-1][j] == 0 {
                    matrix[i-1][j] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i-1][j] = merged[i][j];
                    merged[i][j] = false;
                    null_movement = false;
                }
            } 
        }
        print_matrix(*matrix); // <- remove this after front
    }

    if null_movement {return false};

    return generate_tile(matrix);
}

// Função de movimentação para baixo
// Retorna se o jogo acabou
fn move_down(matrix : &mut [[u128; 4]; 4]) -> bool {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    let mut null_movement: bool = true;

    for k in 0..3 {
        for i in (k..3).rev() {
            for j in 0..4 {
                if (matrix[i][j] == matrix[i+1][j]) && (!merged[i+1][j]) {
                    matrix[i+1][j] *= 2;
                    matrix[i][j] = 0;

                    merged[i+1][j] = true;
                    merged[i][j] = false;
                    null_movement = false;
                }
                else if matrix[i+1][j] == 0 {
                    matrix[i+1][j] = matrix[i][j];
                    matrix[i][j] = 0;

                    merged[i+1][j] = merged[i][j];
                    merged[i][j] = false;
                    null_movement = false;
                }
            } 
        }
        print_matrix(*matrix); // <- remove this after front
    }

    if null_movement {return false};

    return generate_tile(matrix);
}

// Função de fim de jogo
fn end_game() {
    // do something here :D
    // game over screen ???
    println!("Game Over!");
}

// Função de inicialização da matriz e do jogo.
fn start_game(matrix : &mut [[u128; 4]; 4]) {
    *matrix = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    println!("Good Luck, Have Fun!");

    generate_tile(matrix);

    loop {
        let mut movement = String::new();

        io::stdin()
            .read_line(&mut movement)
            .expect("Failed to read line");
        
        let movement: u32 = match movement.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        if movement == 4 {
            if move_left(matrix) {break};
        }
        else if movement == 6 {
            if move_right(matrix) {break};
        }
        else if movement == 8 {
            if move_up(matrix) {break};
        }
        else if movement == 2 {
            if move_down(matrix) {break};
        }
        else {break};

        println!("------------");
    }

    end_game();

}


fn main() {
    //yew::Renderer::<App>::new().render();
    let mut matrix: [ [u128; 4] ; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    start_game(&mut matrix);
}   