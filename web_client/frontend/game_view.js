console.log("HEllo from draw!");

function draw_cross(ctx, x, y, width, height) {
    let x2 = x + width;
    let y2 = y + height;

    ctx.beginPath();

    ctx.moveTo(x, y);
    ctx.lineTo(x2, y2);

    ctx.moveTo(x2, y);
    ctx.lineTo(x, y2);

    ctx.closePath();
    ctx.stroke();
}

//draws an ellipse inside a given rectangle
function draw_ellipse(ctx, x, y, width, height) {
    let radius_x = width / 2;
    let radius_y = height / 2;

    let center_x = x + radius_x;
    let center_y = y + radius_y;

    ctx.beginPath();

    ctx.ellipse(center_x, center_y, radius_x, radius_y, 0, 0, 2 * Math.PI, false);

    ctx.closePath();
    ctx.stroke();
}


class GameElem {
    constructor(type, x, y) {
        this.type = type;
        this.x = x;
        this.y = y;
    }
}

class GameView {
    constructor(rows, cols, res_x, res_y) {
        this.rows = rows;
        this.cols = cols;

        this.res_x = res_x;
        this.res_y = res_y;
        this.game_elements = [];
    }

    draw(ctx) {
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        ctx.beginPath();

        for (let col = 1; col < this.cols; ++col) {
            let x = step_x * col;
            ctx.moveTo(x, 0);
            ctx.lineTo(x, this.res_y);
        }

        for (let row = 1; row < this.rows; ++row) {
            let y = step_y * row;
            ctx.moveTo(0, y);
            ctx.lineTo(this.res_x, y);
        }

        ctx.closePath();
        ctx.stroke();

        for (let i in this.game_elements) {
            let elem = this.game_elements[i];

            let x = elem.x * step_x;
            let y = elem.y * step_y;

            if (elem.type == 'X') {
                draw_cross(ctx, x, y, step_x, step_y);
            }
            else if (elem.type == 'O') {
                draw_ellipse(ctx, x, y, step_x, step_y);
            }
        }
    }

    draw_elem(ctx, elem) {
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        let x = elem.x * step_x;
        let y = elem.y * step_y;

        if (elem.type == 'X') {
            draw_cross(ctx, x, y, step_x, step_y);
        }
        else if (elem.type == 'O') {
            draw_ellipse(ctx, x, y, step_x, step_y);
        }
    }

    view_to_logic_coords(x, y) {
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        return { x: Math.floor(x / step_x), y: Math.floor(y / step_y) };
    }
}
