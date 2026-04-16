use pyo3::prelude::*;

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

        // Canculate test
        //let result = my_module.call_method1("calculate", (3, 4))?;
        //let value: i32 = result.extract()?;
        //println!("Python result: {}", value);
        Ok(())
    })
}
