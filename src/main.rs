use pyo3::prelude::*;
use colored::*;

fn printok (msg: &str) {
    println!("{} {}", msg, "[OK]".green().bold());
}

fn printerr (msg: &str) {
    println!("{} {}", msg, "[ERROR]".red().bold());
}

fn main() -> PyResult<()> {
    println!("Hello, world! By Rust!");

    Python::attach(|py| {
        // Add to sys.path
        let sys = py.import("sys")?;
        sys.getattr("path")?.call_method1("append", ("./src/pyscripts",))?;

        // Import
        let test_module = py.import("testmodule")?;

        let hello_msg = test_module.call_method1("hello_world", ())?;
        let msg: String = hello_msg.extract()?;
        println!("{}", msg);

        println!("Start link tests...");

        match msg.as_str() {
            "Done." => printok("hellotest"),
            _ => printerr("hellotest"),
        }

        // Canculate test
        let libtest = test_module.call_method1("calculate", (3, 4))?;
        let value: f32 = libtest.extract()?;

        match value {
            5.0 => printok("pylibtest"),
            _ => printerr("pylibtest"),
        }
        Ok(())
    })
}
