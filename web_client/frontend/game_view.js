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
        let step_x = this.res_x / this.cols;
        let step_y = this.res_y / this.rows;

        let p1_view = this.logic_to_view_coords(p1.x, p1.y);
        let p2_view = this.logic_to_view_coords(p2.x, p2.y);

        if(p1_view.x == p2_view.x) {
            if(p1_view.y > p2_view.y) {
                let temp = p1_view;
                p1_view = p2_view;
                p2_view = temp;
            }

            p1_view.x += step_x / 2;
            p2_view.x += step_x / 2;
            p2_view.y += step_y;
        }
        else if(p1_view.y == p2_view.y) {
            if(p1_view.x > p2_view.x) {
                let temp = p1_view;
                p1_view = p2_view;
                p2_view = temp;
            }
            p1_view.y += step_y / 2;
            p2_view.y += step_y / 2;
            p2_view.x += step_x;
        }
        else {
            if(p1_view.x < p2_view.x) {
                p2_view.x += step_x;
                p2_view.y += step_y;
            }

            if(p1_view.y < p2_view.y) {
                p1_view.y += step_y;
                p2_view.x -= step_x;
            }
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
