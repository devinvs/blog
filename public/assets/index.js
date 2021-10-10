document.addEventListener("DOMContentLoaded", () => {
    const copyright = document.getElementById("copyright");
    const date = new Date();
    copyright.innerText = `© ${date.getFullYear()} Devin Vander Stelt`;
});
