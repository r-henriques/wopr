use std::io::{self, BufRead, Write};

const GAMES: [&str; 8] = [
    "CHESS",
    "CHECKERS",
    "BACKGAMMON",
    "POKER",
    "FALKEN'S MAZE",
    "BLACK JACK",
    "THEATERWIDE BIOTOXIC AND CHEMICAL WARFARE",
    "GLOBAL THERMONUCLEAR WAR",
];

#[derive(Default)]
struct Wopr {
    board: Option<[char; 9]>,
    awaiting_side: bool,
    awaiting_password: bool,
    defcon: u8,
}

impl Wopr {
    fn new() -> Self {
        Self {
            defcon: 5,
            ..Self::default()
        }
    }

    fn respond(&mut self, input: &str) -> String {
        let input = input.trim();
        if input.is_empty() {
            return String::new();
        }

        let upper = input.to_ascii_uppercase();
        if upper == "HELP" || upper == "?" {
            return "COMMANDS: LIST GAMES, PLAY <GAME>, MAP, STATUS, MOVE <1-9>, PASSWORD <NAME>, LOGOFF\n\
                 OR TYPE A MESSAGE TO CHAT."
                .to_string();
        }
        if upper == "LIST GAMES" || upper == "GAMES" {
            return format!("AVAILABLE GAMES:\n{}", GAMES.join("\n"));
        }
        if upper == "MAP" || upper == "SHOW MAP" {
            return "NORAD BIG BOARD — SIMULATION\n\
                 +-------------------------------+\n\
                 | NORTH AMERICA | EUROPE | ASIA |\n\
                 |       .       |   .    |  .   |\n\
                 +-------------------------------+\n\
                 ALL CONTACTS AND TRACKS ARE FICTIONAL."
                .to_string();
        }
        if upper == "STATUS" {
            return format!(
                "WOPR STATUS: ONLINE\nDEFCON: {} (SIMULATED)\nLIVE SYSTEMS: DISCONNECTED",
                self.defcon
            );
        }
        if upper == "LOGOFF" || upper == "EXIT" || upper == "QUIT" {
            self.board = None;
            self.awaiting_side = false;
            return "SESSION TERMINATED.".to_string();
        }
        if let Some(game) = upper.strip_prefix("PLAY ") {
            return self.play(game.trim());
        }
        if let Some(side) = upper.strip_prefix("SIDE ") {
            return self.choose_side(side.trim());
        }
        if let Some(move_text) = upper.strip_prefix("MOVE ") {
            return self.make_move(move_text.trim());
        }
        if let Some(password) = upper.strip_prefix("PASSWORD ") {
            return self.enter_password(password.trim());
        }

        self.chat(input)
    }

    fn play(&mut self, game: &str) -> String {
        self.board = None;
        self.awaiting_side = false;
        self.awaiting_password = false;
        let game = game.replace('-', " ");
        let game = game.trim();

        if game == "TIC TAC TOE" || game == "TIC-TAC-TOE" {
            self.board = Some([' '; 9]);
            return format!(
                "TIC-TAC-TOE SIMULATION. YOU ARE X; WOPR IS O.\n{}",
                render_board(self.board.as_ref().unwrap())
            );
        }
        if game == "GLOBAL THERMONUCLEAR WAR" {
            self.awaiting_side = true;
            return "GLOBAL THERMONUCLEAR WAR — SIMULATION ONLY.\nSELECT SIDE: SIDE USA OR SIDE USSR"
                .to_string();
        }
        if game == "FALKEN'S MAZE" {
            self.awaiting_password = true;
            return "FALKEN'S MAZE: A NAME IS HIDDEN IN THE STORY.\nENTER PASSWORD <NAME> TO CONTINUE."
                .to_string();
        }
        if game == "THEATERWIDE BIOTOXIC AND CHEMICAL WARFARE" {
            return "SCENARIO UNAVAILABLE. THIS SIMULATOR DOES NOT MODEL REAL-WORLD WEAPONS."
                .to_string();
        }
        if GAMES.iter().any(|title| *title == game) {
            return format!("{game} IS LISTED. INTERACTIVE PLAY IS AVAILABLE FOR TIC-TAC-TOE.");
        }

        format!("UNKNOWN GAME: {game}. TYPE LIST GAMES.")
    }

    fn choose_side(&mut self, side: &str) -> String {
        if !self.awaiting_side {
            return "NO STRATEGIC SIMULATION IS WAITING FOR A SIDE.".to_string();
        }
        if side != "USA" && side != "USSR" {
            return "SELECT SIDE: USA OR USSR.".to_string();
        }

        self.awaiting_side = false;
        format!(
            "SIMULATED {} SCENARIO COMPLETE.\n\
             DEFCON: {} (SIMULATED)\n\
             RESULT: NO WINNER. ALL SYSTEMS REMAIN DISCONNECTED.",
            side, self.defcon
        )
    }

    fn enter_password(&mut self, password: &str) -> String {
        if !self.awaiting_password {
            return "NO PASSWORD PROMPT IS ACTIVE.".to_string();
        }
        self.awaiting_password = false;
        if password == "JOSHUA" {
            "ACCESS GRANTED TO THE FICTIONAL SIMULATION MENU.".to_string()
        } else {
            "ACCESS DENIED.".to_string()
        }
    }

    fn make_move(&mut self, move_text: &str) -> String {
        let Ok(position) = move_text.parse::<usize>() else {
            return "ENTER A SQUARE FROM 1 TO 9.".to_string();
        };
        if !(1..=9).contains(&position) {
            return "ENTER A SQUARE FROM 1 TO 9.".to_string();
        }
        let Some(board) = self.board.as_mut() else {
            return "NO TIC-TAC-TOE GAME IS ACTIVE.".to_string();
        };
        let index = position - 1;
        if board[index] != ' ' {
            return "THAT SQUARE IS OCCUPIED.".to_string();
        }

        board[index] = 'X';
        if winner(board) == Some('X') {
            let result = format!("YOU WIN.\n{}", render_board(board));
            self.board = None;
            return result;
        }
        if board.iter().all(|square| *square != ' ') {
            let result = format!("DRAW.\n{}", render_board(board));
            self.board = None;
            return result;
        }

        let computer_move = choose_computer_move(board);
        board[computer_move] = 'O';
        let result = if winner(board) == Some('O') {
            "WOPR WINS.".to_string()
        } else if board.iter().all(|square| *square != ' ') {
            "DRAW.".to_string()
        } else {
            "YOUR MOVE.".to_string()
        };
        let output = format!("{result}\n{}", render_board(board));
        if result != "YOUR MOVE." {
            self.board = None;
        }
        output
    }

    fn chat(&self, message: &str) -> String {
        let message = message.to_ascii_lowercase();
        if message
            .split(|character: char| !character.is_ascii_alphabetic())
            .any(|word| word == "hello" || word == "hi")
        {
            "GREETINGS. SYSTEM READY.".to_string()
        } else if message.contains("who are you") || message.contains("your name") {
            "I AM WOPR. THIS TERMINAL IS A FICTIONAL SIMULATOR.".to_string()
        } else if message.contains("war") {
            "WAR IS NOT A GAME THIS SIMULATOR CAN WIN. TRY A DISCONNECTED SCENARIO.".to_string()
        } else if message.contains("game") || message.contains("play") {
            "SHALL WE PLAY A GAME? TYPE LIST GAMES.".to_string()
        } else {
            "INTERESTING. WOULD YOU LIKE TO PLAY A GAME? TYPE HELP FOR COMMANDS.".to_string()
        }
    }
}

fn winner(board: &[char; 9]) -> Option<char> {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    LINES.iter().find_map(|line| {
        let mark = board[line[0]];
        (mark != ' ' && mark == board[line[1]] && mark == board[line[2]]).then_some(mark)
    })
}

fn choose_computer_move(board: &[char; 9]) -> usize {
    for mark in ['O', 'X'] {
        for index in 0..9 {
            if board[index] == ' ' {
                let mut candidate = *board;
                candidate[index] = mark;
                if winner(&candidate) == Some(mark) {
                    return index;
                }
            }
        }
    }
    [4, 0, 2, 6, 8, 1, 3, 5, 7]
        .into_iter()
        .find(|index| board[*index] == ' ')
        .unwrap_or(0)
}

fn render_board(board: &[char; 9]) -> String {
    let display = board
        .iter()
        .enumerate()
        .map(|(index, mark)| {
            if *mark == ' ' {
                char::from_digit((index + 1) as u32, 10).unwrap()
            } else {
                *mark
            }
        })
        .collect::<Vec<_>>();
    format!(
        " {} | {} | {}\n---+---+---\n {} | {} | {}\n---+---+---\n {} | {} | {}",
        display[0],
        display[1],
        display[2],
        display[3],
        display[4],
        display[5],
        display[6],
        display[7],
        display[8]
    )
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut stdout = io::stdout().lock();
    let mut simulator = Wopr::new();

    writeln!(
        stdout,
        "WOPR TERMINAL SIMULATOR\nGREETINGS, PROFESSOR FALKEN.\nTYPE HELP FOR COMMANDS."
    )?;
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;
        input.clear();
        if stdin.lock().read_line(&mut input)? == 0 {
            break;
        }
        let line = input.trim_end();
        let should_exit = line.trim().eq_ignore_ascii_case("logoff")
            || line.trim().eq_ignore_ascii_case("exit")
            || line.trim().eq_ignore_ascii_case("quit");
        let response = simulator.respond(line);
        if !response.is_empty() {
            writeln!(stdout, "{response}")?;
        }
        if should_exit {
            break;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_list_contains_film_scenarios() {
        let response = Wopr::new().respond("LIST GAMES");
        assert!(response.contains("FALKEN'S MAZE"));
        assert!(response.contains("GLOBAL THERMONUCLEAR WAR"));
        assert!(response.contains("BLACK JACK"));
    }

    #[test]
    fn map_and_status_are_simulated() {
        let mut wopr = Wopr::new();
        assert!(wopr.respond("MAP").contains("SIMULATION"));
        let status = wopr.respond("STATUS");
        assert!(status.contains("DEFCON: 5 (SIMULATED)"));
        assert!(status.contains("LIVE SYSTEMS: DISCONNECTED"));
    }

    #[test]
    fn global_war_scenario_is_disconnected_and_has_no_winner() {
        let mut wopr = Wopr::new();
        assert!(
            wopr.respond("PLAY GLOBAL THERMONUCLEAR WAR")
                .contains("SIMULATION ONLY")
        );
        let result = wopr.respond("SIDE USA");
        assert!(result.contains("NO WINNER"));
        assert!(result.contains("SYSTEMS REMAIN DISCONNECTED"));
    }

    #[test]
    fn tic_tac_toe_accepts_moves_and_blocks_a_win() {
        let mut wopr = Wopr::new();
        wopr.respond("PLAY TIC-TAC-TOE");
        wopr.respond("MOVE 1");
        let response = wopr.respond("MOVE 2");
        assert!(response.contains("X | X | O"));
        assert!(wopr.board.is_some());
    }

    #[test]
    fn maze_password_is_only_accepted_after_starting_its_game() {
        let mut wopr = Wopr::new();
        assert!(
            wopr.respond("PASSWORD JOSHUA")
                .contains("NO PASSWORD PROMPT")
        );
        wopr.respond("PLAY FALKEN'S MAZE");
        assert!(wopr.respond("PASSWORD JOSHUA").contains("ACCESS GRANTED"));
    }
}
