var game_view;
var el_type = 'X';
var game_socket;

function get_canvas() {
    return document.getElementById("game_field");
}


function handle_message(obj) {
    let ctx = get_canvas().getContext("2d");

    if (obj.Cross) {
        let elem = new GameElem('X', obj.Cross.x, obj.Cross.y);
        game_view.draw_elem(ctx, elem);
    }
    else if(obj.Circle) {
        let elem = new GameElem('O', obj.Circle.x, obj.Circle.y);
        game_view.draw_elem(ctx, elem);
    }
    else if(obj.Line) {
        console.log("unimplemented!");
    }
    else if(obj.Info) {
        alert(obj.Info);
    }
    else {
        console.log("Invalid object: ", obj);
    }


}


function init() {
    console.log("Init called");

    let game_field = get_canvas();
    let ctx = game_field.getContext("2d");

    game_socket = new WebSocket("ws://localhost:3000/ws");

    game_socket.addEventListener('open', function (event) {
        console.log("Socket is opened");
        game_socket.send('Hello Server!');
    });

    game_socket.addEventListener('message', function(event) {
        console.log("Message received: ", event.data);
        handle_message(JSON.parse(event.data));
    })

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
    game_socket.send(JSON.stringify(logic));
}

function set_random_element(logic_x, logic_y) {

    let canvas = get_canvas();
    let ctx = canvas.getContext("2d");

    game_view.draw_elem(ctx, new GameElem(el_type, logic_x, logic_y));

    if (el_type == 'X') {
        el_type = 'O';
    }
    else {
        el_type = 'X';
    }
}
