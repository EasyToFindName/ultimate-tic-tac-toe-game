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

//draws ellipse inside given rectangle
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

function draw_line(ctx, x1, y1, x2, y2, line_weight) {
    let old_weight = ctx.lineWidth;
    ctx.lineWidth = line_weight;

    ctx.beginPath();

    ctx.moveTo(x1, y1);
    ctx.lineTo(x2, y2);

    ctx.closePath();
    ctx.stroke();

    ctx.lineWidth = old_weight;
}
