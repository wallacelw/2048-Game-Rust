use yew::prelude::*;
use rand::Rng;
use std::io;
use std:: mem;

// Cria o html
// Mas por enquanto o jogo não usa esse front
#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

// temporary debugging function
fn print_matrix(matrix: [[u128; 4]; 4]) {
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
    println!();
}

/*
Função que verifica se o jogo acabou, ou seja, não existe mais nenhum espaço vazio e nenhum conjunto de blocos podem ser unidos. Só é chamada depois de verificado que não há mais nenhum espaço em branco.

true -> game over
false -> still at least one movement left
*/
fn check_end(matrix : &mut [[u128; 4]; 4]) -> bool {
    let mut movement: bool = true;
    for i in 0..4 {
        for j in 0..4 {
            if (i > 0) && (matrix[i-1][j] == matrix[i][j]) {
                movement = false;
            }
            else if (i < 3) && (matrix[i+1][j] == matrix[i][j]) {
                movement = false;
            }
            else if (j > 0) && (matrix[i][j-1] == matrix[i][j]) {
                movement = false;
            }
            else if (j < 3) && (matrix[i][j+1] == matrix[i][j]) {
                movement = false;
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
                    if zeros == 1 {
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

/* Função de movimentação
Retorna se o jogo acabou:
    true -> acabou
    false -> não acabou ainda
Direction[] define a orientação da movimentação:
    [0,  1] -> para a direita
    [0, -1] -> para a esquerda
    [ 1, 0] -> para baixo
    [-1, 0] -> para cima
*/
fn shift_aux(matrix: &mut [[u128; 4]; 4], merged: &mut [ [bool; 4]; 4], direction: [i32; 2], i: i32, j:i32, null_movement: &mut bool) {
    let mut row = i as usize;
    let mut col = j as usize;

    // change horizontal movement to vertical movement
    if direction[0] != 0 {
        mem::swap(&mut row, &mut col);
    };

    let adj_row = (row as i32 + direction[0]) as usize;
    let adj_col = (col as i32 + direction[1]) as usize;

    if matrix[row][col] == 0 {return};

    if (matrix[row][col] == matrix[adj_row][adj_col]) && 
    (!merged[adj_row][adj_col]) && (!merged[row][col]) {
        matrix[adj_row][adj_col] *= 2;
        matrix[row][col] = 0;
        
        merged[adj_row][adj_col] = true;
        merged[row][col] = false;
        *null_movement = false;
    }

    else if matrix[adj_row][adj_col] == 0 {
        matrix[adj_row][adj_col] = matrix[row][col];
        matrix[row][col] = 0;

        merged[adj_row][adj_col] = merged[row][col];
        merged[row][col] = false;
        *null_movement = false;
    }
}

fn shift(matrix : &mut [[u128; 4]; 4], direction : [i32; 2]) -> bool {

    let mut merged: [ [bool; 4]; 4] = [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false]
    ];

    let mut null_movement: bool = true;

    for k in 0..3 {
        // left, up
        if (direction[0] + direction[1]) == -1 {
            for j in 1..(4-k) {
                for i in 0..4 {
                    shift_aux(matrix, &mut merged, direction, i, j, &mut null_movement);
                } 
            }
        }

        // right, down
        else {
            for j in (k..3).rev() {
                for i in 0..4 {
                    shift_aux(matrix, &mut merged, direction, i, j, &mut null_movement);
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
    
        if movement == 4 { // Left
            if shift(matrix, [0, -1]) {break};
        }
        else if movement == 6 { // Right
            if shift(matrix, [0,  1]) {break};
        }
        else if movement == 8 { // Up
            if shift(matrix, [-1, 0]) {break};
        }
        else if movement == 2 { // Down
            if shift(matrix, [ 1, 0]) {break};
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