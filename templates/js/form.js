$(function () {
    $(document).tooltip({
        position: {
            my: "center bottom",
            at: "center top-16",
            using: function (position, feedback) {
                $(this).css(position);
                $("<div>")
                    .addClass("arrow")
                    .addClass(feedback.vertical)
                    .addClass(feedback.horizontal)
                    .appendTo(this);
            }
        }
    });
});

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

$(function () {
    $("#addCharacterForm").submit(function (e) {
        e.preventDefault();
        var formData = new FormData($('#addCharacterForm')[0]);
        $.ajax({
            url: '/add_character',
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

function manualAddCharacter(playerId) {
    $('#characterPlayerId').val(playerId);
    $('#addCharacterBody').removeClass('hidden');
    $('#addCharacterBody').addClass('modal');
}

function closeAddCharacter() {
    $('#addCharacterBody').addClass('hidden');
    $('#addCharacterBody').removeClass('modal');
}