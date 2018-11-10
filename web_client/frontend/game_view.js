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

        ctx.lineWidth = 2;
        ctx.strokeStyle = "#99ccff";
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
        ctx.lineWidth = 3;
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        let x = elem.x * step_x;
        let y = elem.y * step_y;

        if (elem.type == 'X') {
            ctx.strokeStyle = "#000080"
            draw_cross(ctx, x, y, step_x, step_y);
        }
        else if (elem.type == 'O') {
            ctx.strokeStyle = "#f81894";
            draw_ellipse(ctx, x, y, step_x, step_y);
        }
    }

    draw_winning_line(ctx, p1, p2) {
        let direction_vec = {
            x: Math.sign(p2.x - p1.x),
            y: Math.sign(p2.y - p1.y)
        };

        console.log(direction_vec);

        if(direction_vec.y == - 1
        ||(direction_vec.y == 0 && direction_vec.x == -1)) {
            let temp = p1;
            p1 = p2;
            p2 = temp;

            direction_vec.x *= -1;
            direction_vec.y *= -1;
        }

        if(direction_vec.x == -1) {
            p1.x += 1;
            p2.x += 1;
        }

        p2.x += direction_vec.x;
        p2.y += direction_vec.y;

        console.log(p1);
        console.log(p2);

        let p1_view = this.logic_to_view_coords(p1.x, p1.y);
        let p2_view = this.logic_to_view_coords(p2.x, p2.y);

        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        if(p1_view.x == p2_view.x) {
            p1_view.x += step_x / 2;
            p2_view.x += step_x / 2;
        }
        else if(p1_view.y == p2_view.y) {
            p1_view.y += step_y / 2;
            p2_view.y += step_y / 2;
        }

        let old_style = ctx.strokeStyle;
        ctx.strokeStyle = "#66ee30";
        draw_line(ctx, p1_view.x, p1_view.y, p2_view.x, p2_view.y, 6);
        ctx.strokeStyle = old_style;
    }

    view_to_logic_coords(x, y) {
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        return { x: Math.floor(x / step_x), y: Math.floor(y / step_y) };
    }

    logic_to_view_coords(x, y) {
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        return { x: x * step_x, y: y * step_y };
    }
}
