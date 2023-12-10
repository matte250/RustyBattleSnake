use rocket::serde::{Serialize, json::Json};
use serde::Deserialize;

#[macro_use] extern crate rocket;

pub mod graph;

#[get("/")]
fn index() -> Json<ConfigResponse> {
    Json(ConfigResponse {
        api_version: "1".to_string(),
        author: "author".to_string(),
        color: "#ffffff".to_string(),
        head: "default".to_string(),
        tail: "default".to_string(),
        version: "0.0.1".to_string(),
    })
}

#[post("/start", format = "application/json", data = "<game_state>")]
fn start_handler(game_state: Json<GameState>) {
    print!("GAME {} STARTED", game_state.game.id)
}

#[post("/end", format = "application/json", data = "<game_state>")]
fn end_handler(game_state: Json<GameState>) {
    print!("GAME {} ended", game_state.game.id)
}

#[post("/move", format = "application/json", data = "<game_state>")]
fn move_handler(game_state: Json<GameState>) -> Json<TurnResponse> {
    print!("TURN {} FOR GAME {}", game_state.turn, game_state.game.id);

    Json(TurnResponse { r#move: Direction::Right, shout: None })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, start_handler, end_handler, move_handler])
}


/// Game state of any given turn.
/// 
/// API
/// - https://docs.battlesnake.com/api/requests/start
/// - https://docs.battlesnake.com/api/requests/move
/// - https://docs.battlesnake.com/api/requests/end
#[derive(Deserialize)]
struct GameState
{
    /// Game object describing the game being played.
    game: Game,

    /// Turn number of the game being played. Is 0 for new games.
    turn: u32,

    /// Board object describing the game board on this turn.
    board: Board,

    /// Battlesnake object describing your BattleSnake.
    you: BattleSnake,
}

/// https://docs.battlesnake.com/api/objects/game
#[derive(Deserialize)]
struct Game
{
    /// A unique identifier for the this game.
    id: String,

    /// Information about the rule set being used to run this game.
    #[serde(rename = "ruleset")]
    rule_set: serde_json::Map<String, serde_json::Value>,

    /// The name of the map used to populate the game board with snakes, food, and hazards.
    map: String,

    /// (milliseconds) How much time your snake has to respond to requests for this game.
    timeout: u32,

    /// The source of this game. One of:
    /// - tournament
    /// - league
    /// - arena
    /// - challenge
    /// - custom
    source: String,
}

/// https://docs.battlesnake.com/api/objects/board
#[derive(Deserialize)]
struct Board {
    /// The number of rows in the y-axis of the game board.
    height: u32,

    /// The number of columns in the x-axis of the game board.
    width: u32,

    /// Array of coordinates representing food locations on the game board. Example:
    food: Vec<Coord>,

    /// Array of coordinates representing hazardous locations on the game board. These will only appear in some [game modes](https://docs.battlesnake.com/guides/playing/modes).
    hazards: Vec<Coord>,

    /// Array of Battlesnake Objects representing all Battlesnakes remaining on the game board (including yourself if you haven't been eliminated).
    snakes: Vec<BattleSnake>
}

/// https://docs.battlesnake.com/api/objects/battlesnake
#[derive(Deserialize)]
struct BattleSnake {
    /// Unique identifier for this Battlesnake in the context of the current Game.
    id: String,

    /// Name given to this Battlesnake by its author.
    name: String,

    /// Health value of this Battlesnake, between 0 and 100 inclusively.
    health: u32,

    /// Array of coordinates representing this Battlesnake's location on the game board. This array is ordered from head to tail.
    body: Vec<Coord>,

    /// The previous response time of this Battlesnake, in milliseconds. If the Battlesnake timed out and failed to respond, the game timeout will be returned (game.timeout)
    latency: String,

    /// Coordinates for this Battlesnake's head. Equivalent to the first element of the body array.
    head: Coord,

    /// Length of this Battlesnake from head to tail. Equivalent to the length of the body array.
    length: u32, 

    /// Message shouted by this Battlesnake on the previous turn.
    shout: String,

    /// The squad that the Battlesnake belongs to. Used to identify squad members in Squad Mode games.
    squad: String,

    /// The collection of customizations that control how this Battlesnake is displayed.     
    customizations: Customizations,
}

#[derive(Deserialize)]
struct Customizations {
    color: String,
    head: String,
    tail: String,
}

#[derive(Deserialize)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(Serialize)]
struct ConfigResponse
{
    #[serde(rename = "apiversion")]
    /// Version of the Battlesnake API implemented by this Battlesnake. Currently only API version 1 is valid.
    api_version: String,

    /// Username of the author of this Battlesnake. If provided, this will be used to verify ownership.
    author: String,

    /// Hex color code used to display this Battlesnake. Must start with "#" and be 7 characters long.
    color: String,

    /// Displayed head of this Battlesnake. See [Customization Guide](https://docs.battlesnake.com/guides/customizations) for available options.
    head: String,

    /// Displayed tail of this Battlesnake. See [Customization Guide](https://docs.battlesnake.com/guides/customizations) for available options.
    tail: String,

    /// A version number or tag for your snake.
    version: String
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
struct TurnResponse {
    r#move: Direction,
    shout: Option<String>,
}