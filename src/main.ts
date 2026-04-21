import { invoke } from "@tauri-apps/api/core";

invoke('cmd_print');

document.addEventListener("DOMContentLoaded", () => {
    const button = document.querySelector("button");
    const status = document.querySelector("p");

    if (button && status) {
        button.addEventListener("click", async () => {
            try {
                const cheese: string = await invoke("create_cheese");
                
                status.textContent = cheese;
                
                button.textContent = "MORE!";
            } catch (err) {
                console.error("Ошибка при вызове команды:", err);
                status.textContent = "Не удалось получить сыр...";
            }
        });
    }
});