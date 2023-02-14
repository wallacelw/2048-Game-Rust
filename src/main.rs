//! Implementação do jogo 2048 utilizando a linguagem Rust
//!
//! Foi utilizado o framework Yew para criar o aplicativo web

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

/** Função que verifica se o jogo acabou, ou seja, não existe mais nenhum espaço vazio e nenhum conjunto de blocos podem ser unidos. Só é chamada depois de verificado que não há mais nenhum espaço em branco. Return {true} indica que o jogo acabou e return {false} indica que o jogo persiste.
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

/**
Função para gerar um novo bloco em uma posição livre (=0) com
a probabilidade de 90% de gerar um bloco de valor 2 e probabilidade de 10% de gerar um bloco de valor 4. 

A posição é escolhida de modo equiprovável.
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

/** Função auxiliar de movimentação
Verifica se dois blocos adjacentes podem ser unidos ou se apenas deve se locomover um bloco para um espaço vazio.
*/
fn shift_aux(matrix: &mut [[u128; 4]; 4], merged: &mut [ [bool; 4]; 4], direction: [i32; 2], i: i32, j:i32, null_movement: &mut bool, score: &mut u128) {
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
        *score += matrix[adj_row][adj_col];
        
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

/** Função de movimentação
Direction[] define a orientação da movimentação:
    [0,  1] -> para a direita,
    [0, -1] -> para a esquerda,
    [ 1, 0] -> para baixo,
    [-1, 0] -> para cima
*/
fn shift(matrix : &mut [[u128; 4]; 4], direction : [i32; 2], score: &mut u128) -> bool {

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
                    shift_aux(matrix, &mut merged, direction, i, j, &mut null_movement, score);
                } 
            }
        }

        // right, down
        else {
            for j in (k..3).rev() {
                for i in 0..4 {
                    shift_aux(matrix, &mut merged, direction, i, j, &mut null_movement, score);
                }
            }
        }

        print_matrix(*matrix); // <- remove this after front
    }

    if null_movement {return false};

    return generate_tile(matrix);
}

/**
Função de fim de jogo
É chamada após ser detectada que não há mais movimentos possíveis para o jogo.
*/ 
fn end_game(score: &mut u128) {
    // do something here :D
    // game over screen ???
    println!("Game Over!");
    println!("Score: {}", *score);
}

/**
Função de inicialização do jogo.
Inicializa também a matriz com elementos nulos e reinicia a pontuação.
*/ 
fn start_game(matrix : &mut [[u128; 4]; 4]) {
    *matrix = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    println!("Good Luck, Have Fun!");

    generate_tile(matrix);

    let mut score: u128 = 0;

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
            if shift(matrix, [0, -1], &mut score) {break};
        }
        else if movement == 6 { // Right
            if shift(matrix, [0,  1], &mut score) {break};
        }
        else if movement == 8 { // Up
            if shift(matrix, [-1, 0], &mut score) {break};
        }
        else if movement == 2 { // Down
            if shift(matrix, [ 1, 0], &mut score) {break};
        }
        else {break};

        println!("------------");
    }

    end_game(&mut score);

}


/**
Função main, que inicializa a págica e chama a função para inicializar o jogo.
*/ 
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



// Módulo de testes
#[cfg(test)]
mod tests {
    use crate::shift;

    struct TestCase {
        state: [ [u128; 4] ; 4],
        moves: [i32; 2],
    }

    #[test]
    fn move_up(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [0, 2, 2, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[-1,0]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);
       
        assert!(teste.state[0][1] == 2 && teste.state[0][2] == 2);
    }
    
    #[test]
    fn move_down(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [0, 2, 2, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[1,0]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);

        assert!(teste.state[3][1] == 2 && teste.state[3][2] == 2);
    
    }

    #[test]
    fn move_rigth(){
        let mut teste = 
            TestCase {
                    
                state: [
                [0, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 0, 0]
                ],
                    
                moves:[0,1]
                };
            
        shift(&mut teste.state,teste.moves, &mut 1);

        assert!(teste.state[1][3] == 2 && teste.state[2][3] == 2);
    
    }
        
    #[test]
    fn move_left(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [0, 0, 2, 0],
                [0, 0, 2, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[0,-1]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);

        assert!(teste.state[1][0] == 2 && teste.state[2][0] == 2);

        }
    
    #[test]
    fn combine(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [2, 2, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[0,-1]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);

        assert!(teste.state[1][0] == 4);

        }
    
    #[test]
    fn not_combine(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [2, 4, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[0,-1]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);

        assert!(teste.state[1][0] == 2 && teste.state[1][1] == 4);
        }
   
    #[test]
    fn generating_tale(){
        let mut teste = 
            TestCase {
                
                state: [
                [0, 0, 0, 0],
                [0, 2, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
                ],
                
                moves:[-1,0]
            };
        
        shift(&mut teste.state,teste.moves, &mut 1);

        let mut non_zeros = 0;
        
        for i in 0..4 {
            for j in 0..4 {
                if teste.state[i][j] == 2 || teste.state[i][j] == 4 {

                    non_zeros +=1

                }
            }
        }

        assert_eq!(non_zeros,2);

        }


}
       