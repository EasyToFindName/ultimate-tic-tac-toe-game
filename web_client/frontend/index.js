document.addEventListener('DOMContentLoaded', function () {
    var elems = document.querySelectorAll('.modal');
    var instances = M.Modal.init(elems);

    let submit = document.getElementById('lobby_submit');

    submit.addEventListener('click', function () {
        let width_field = document.getElementById('game_field_width');
        let height_field = document.getElementById('game_field_height');
        let win_seq_len_field = document.getElementById('win_seq_len');
        let public_field = document.getElementById('public');
        let hardcore_mode_on_field = document.getElementById('hardcore_mode_on');

        let game_cfg = {
            width: parseInt(width_field.value),
            height: parseInt(height_field.value),
            win_seq_len: parseInt(win_seq_len_field.value),
            public: public_field.checked,
            hardcore_mode_on: hardcore_mode_on_field.checked
        };

        let xhr = new XMLHttpRequest();
        xhr.open("POST", "/lobby/create", true);

        xhr.onreadystatechange = function () {
            if (xhr.readyState === 4 && xhr.status === 200) {
                console.log(xhr.getAllResponseHeaders());
                console.log(xhr.responseText);
            }
        };

        xhr.setRequestHeader("Content-Type", "application/json");
        console.log(game_cfg);
        xhr.send(JSON.stringify(game_cfg));
    });
});