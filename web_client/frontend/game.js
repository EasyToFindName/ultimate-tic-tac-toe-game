var game_view;
var el_type = 'X';
var game_socket;

function get_field() {
    return document.getElementById("game_field");
}

function init() {
    console.log("Init called");

    let game_field = get_field();
    let ctx = game_field.getContext("2d");

    game_socket = new WebSocket("ws://localhost:3000/ws");

    game_socket.addEventListener('open', function (event) {
        console.log("Socket is opened");
        socket.send('Hello Server!');
    });

    console.log("Web socket should be created");

    game_view = new GameView(10, 10, game_field.width, game_field.height);
    game_view.draw(ctx);

    game_field.addEventListener("click", function (ev) {
        mouse_clicked(this, ev);
    });
}

function mouse_clicked(canvas, ev) {
    console.log("Mouse clicked");

    let rect = canvas.getBoundingClientRect();

    let view = {
        x: ev.clientX - rect.left,
        y: ev.clientY - rect.top
    };

    let logic = game_view.view_to_logic_coords(view.x, view.y);

    set_random_element(logic.x, logic.y);
}

function set_random_element(logic_x, logic_y) {

    let canvas = get_field();
    let ctx = canvas.getContext("2d");

    game_view.draw_elem(ctx, new GameElem(el_type, logic_x, logic_y));

    if (el_type == 'X') {
        el_type = 'O';
    }
    else {
        el_type = 'X';
    }
}
