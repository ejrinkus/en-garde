/* Load the navbar contents from the shared file. */
$(function () {
    $("#titlebar").load("/templates/navbar.html.hbs");
});

/* The "transformed" class will turn the hamburger menu into a close-button. The
 * "open" class will extend the sidenav from the left side of the screen. The
 * "pushed" class will push the main content to the right to make room for the
 * sidenav. The two functions below manipulate these classes at once in order
 * to open/close the sidenav with the proper animations. closeNav() specifically
 * is needed to support behavior where the sidenav will automatically close if
 * the user clicks outside the sidenav.
 */
function closeNav() {
    $("#hamburger").removeClass("transformed");
    $("#sidenav").removeClass("open");
    $("#main").removeClass("pushed");
}
function toggleNav() {
    $("#hamburger").toggleClass("transformed");
    $("#sidenav").toggleClass("open");
    $("#main").toggleClass("pushed");
}