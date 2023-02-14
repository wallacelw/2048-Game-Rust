//! Implementação do jogo 2048 utilizando a linguagem Rust
//!
//! Foi utilizado o framework Yew para criar o aplicativo web

use yew::prelude::*;
use rand::Rng;
use std::cell::RefCell;
use std::{ mem };

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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub matrix: RefCell<[ [u128; 4] ; 4]>,
}

/**
Função de inicialização do jogo.
Inicializa também a matriz com elementos nulos e reinicia a pontuação.
*/ 
struct Game {
    matrix: [[u128; 4]; 4],
    score: u128,
    onkeypress: Callback<KeyboardEvent>,
    ended: bool
}
enum Msg {
    KeyUP,
    KeyDOWN,
    KeyLEFT,
    KeyRIGHT,
    Nothing
}


impl Component for Game {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut matrix: [ [u128; 4] ; 4] = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0]
        ];
        generate_tile(&mut matrix);

        let onkeypress = _ctx.link().callback(|event: KeyboardEvent| {
            match event.key().as_str() {
                "ArrowUp" => {Msg::KeyUP},
                "ArrowDown" => {Msg::KeyDOWN},
                "ArrowLeft" => {Msg::KeyLEFT},
                "ArrowRight" => {Msg::KeyRIGHT}
                _ => {Msg::Nothing}
            }
        });
        
        Self { 
            matrix: matrix,
            score: 0,
            onkeypress: onkeypress,
            ended: false
        } 
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::KeyUP => {if shift(&mut self.matrix, [-1, 0], &mut self.score) {self.ended = true}},
            Msg::KeyDOWN => {if shift(&mut self.matrix, [1, 0], &mut self.score) {self.ended = true}},
            Msg::KeyLEFT => {if shift(&mut self.matrix, [0, -1], &mut self.score) {self.ended = true}},
            Msg::KeyRIGHT => {if shift(&mut self.matrix, [0, 1], &mut self.score) {self.ended = true}},
            _ => return false,
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
            <main>
            <div class="container">
                <div class="game">
                    <div class="game__header">
                        <div class="game__header__2048">{"2048"}</div>
                        <div class="content">
                            <div>
                                <p>{"pontuação"}</p>
                                <span>{self.score}</span>
                            </div>
                            <div>
                                <p>{"melhor"}</p>
                                <span>{"0"}</span>
                            </div>
                            <p>{"junte as peças até formar 2048!"}</p>
                        </div>
                    </div>
                <div class="board">
                    <div class={format!("piece piece-{}", self.matrix[0][0].to_string())}>{self.matrix[0][0].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[0][1].to_string())}>{self.matrix[0][1].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[0][2].to_string())}>{self.matrix[0][2].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[0][3].to_string())}>{self.matrix[0][3].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[1][0].to_string())}>{self.matrix[1][0].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[1][1].to_string())}>{self.matrix[1][1].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[1][2].to_string())}>{self.matrix[1][2].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[1][3].to_string())}>{self.matrix[1][3].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[2][0].to_string())}>{self.matrix[2][0].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[2][1].to_string())}>{self.matrix[2][1].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[2][2].to_string())}>{self.matrix[2][2].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[2][3].to_string())}>{self.matrix[2][3].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[3][0].to_string())}>{self.matrix[3][0].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[3][1].to_string())}>{self.matrix[3][1].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[3][2].to_string())}>{self.matrix[3][2].to_string()}</div>
                    <div class={format!("piece piece-{}", self.matrix[3][3].to_string())}>{self.matrix[3][3].to_string()}</div>
                </div>

                <h1 style={format!("display: {}", if self.ended {""} else {"none"})}> {"GAME OVER"} </h1>
                <button onkeydown={self.onkeypress.clone()}>{"iniciar"} <i class="fa fa-play" style="font-size:36px; color: white;"></i></button>
                </div>
                </div>
            </main>
            </>
        )
    }

}

// Cria o html
// Mas por enquanto o jogo não usa esse front
#[function_component(App)]
fn app() -> Html {
    html! {<Game/>}
}

/**
Função main, que inicializa a págica e chama a função para inicializar o jogo.
*/ 
fn main() {
    yew::Renderer::<App>::new().render();
}   