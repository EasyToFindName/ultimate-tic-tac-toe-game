 document.addEventListener('DOMContentLoaded', function() {
    var elems = document.querySelectorAll('.modal');
    var instances = M.Modal.init(elems);

    var create_lobby_form = document.getElementById('create_lobby_form');
    var submit = document.getElementById('lobby_submit');

    console.log(submit);
    console.log(create_lobby_form);

    submit.addEventListener('click', function() {
        create_lobby_form.submit();
    });

  });