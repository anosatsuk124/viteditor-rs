import init, { open } from "../web-ui/pkg";
await init();

const obj = document.getElementById("file");
obj?.addEventListener("change", (event) => {
    const target = event.target as HTMLInputElement;
    const file = (target.files as FileList)[0];

    file.text().then((text) => {
        open(text);
    });
});
