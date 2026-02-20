// Pre-hook script to transform module_name into struct_module_name
// This converts kebab-case to PascalCase for Rust struct names

let module_name = variable::get("module_name");

// Convert to PascalCase: capitalize first letter of each word and remove hyphens
let parts = module_name.split("-");
let struct_name = "";

for part in parts {
    if part.len() > 0 {
        let first_char = part.sub_string(0, 1).to_upper();
        let rest = part.sub_string(1, part.len());
        struct_name += first_char + rest;
    }
}

// Set the new variable for use in templates
variable::set("struct_module_name", struct_name);
