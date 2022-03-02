$(function () {
    $("#addPlayerForm").submit(function (e) {
        e.preventDefault();
        var formData = new FormData($('#addPlayerForm')[0]);
        $.ajax({
            url: '/add_player',
            type: 'POST',
            data: formData,
            async: false,
            cache: false,
            contentType: false,
            processData: false,
            success: function (response) {
                console.log(response);
            }
        });
        location.reload();
    })
});