// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn cmd_print() {
    println!("Working!!!!");
}

#[tauri::command]
fn create_cheese() -> String {
    format!("A LOT OF CHEESE YAAAAAY!!!!!!! :33333333")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![cmd_print, create_cheese])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/*
document.addEventListener("DOMContentLoaded", () => {
    const output = document.getElementById("output");
    const buttons = document.querySelectorAll("button[data-action]");

    buttons.forEach((btn) => {
        btn.addEventListener("click", async () => {
            const action = (btn as HTMLElement).dataset.action; 
            const name = (btn as HTMLElement).textContent; // Текст кнопки (для красоты)

            if (!action) return; // Если вдруг атрибута нет — выходим

            try {
                const result: string = await invoke(`create_${action}`);
                
                if (output) output.textContent = `${name}: ${result}`;
                
            } catch (err) {
                console.error(`Ошибка в ${action}:`, err);
                if (output) output.textContent = "Ой, что-то сломалось...";
            }
        });
    });
});
*/